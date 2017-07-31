//! Amazon AWS RDS Event Delete Actions
use clap::{App, Arg, ArgMatches, SubCommand};
use error::{ErrorKind, Result};
use rusoto_core::{self, ProfileProvider, Region};
use rusoto_rds::{Rds, RdsClient, DeleteEventSubscriptionMessage};
use term;

/// Delete Event Subscription subcommand.
fn subscription_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("subscription")
        .about("Delete RDS Event Subscription")
        .arg(Arg::with_name("subscription_name")
                 .help("The name of the subscription to delete")
                 .required(true))
}

/// Event Delete Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete RDS Event Subscriptions")
        .subcommand(subscription_subcommand())
}

/// Delete an Event Subscription
pub fn subscriptions(region: Region, matches: &ArgMatches) -> Result<()> {
    let mut stdout = term::stdout().ok_or_else(|| ErrorKind::CreateTerm)?;
    let provider = ProfileProvider::new()?;
    let tls_client = rusoto_core::default_tls_client()?;
    let client = RdsClient::new(tls_client, provider, region);
    let mut message: DeleteEventSubscriptionMessage = Default::default();

    if let Some(subscr_name) = matches.value_of("subscription_name") {
        message.subscription_name = subscr_name.to_string();
    }

    let res_message = client.delete_event_subscription(&message)?;

    if let Some(subscription) = res_message.event_subscription {
        stdout.fg(term::color::GREEN)?;
        stdout.attr(term::Attr::Bold)?;
        write!(stdout,
               "{}:",
               or_none!(subscription.subscription_creation_time))?;
        stdout.reset()?;
        stdout.flush()?;
        stdout.fg(term::color::GREEN)?;

        let mut categories_str = String::new();
        try_join!(categories_str, subscription.event_categories_list);

        let mut source_ids_str = String::new();
        try_join!(source_ids_str, subscription.source_ids_list);

        write!(stdout, " {}", or_none!(subscription.source_type))?;

        if !categories_str.is_empty() {
            write!(stdout, " {}", categories_str)?;
        }

        if !source_ids_str.is_empty() {
            write!(stdout, " {}", source_ids_str)?;
        }

        writeln!(stdout,
                 " {} {}",
                 or_none!(subscription.cust_subscription_id),
                 or_none!(b => subscription.enabled))?;
        stdout.reset()?;
        stdout.flush()?;
    }

    Ok(())
}
