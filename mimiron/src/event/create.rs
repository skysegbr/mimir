//! Amazon AWS RDS Event Create Actions
use clap::{App, Arg, ArgMatches, SubCommand};
use error::{ErrorKind, Result};
use rusoto_core::{self, ProfileProvider, Region};
use rusoto_rds::{CreateEventSubscriptionMessage, Rds, RdsClient};
use term;

/// Create Event Subscriptions subcommand.
fn subscription_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("subscription")
        .about("Create RDS Event Subscription")
        .arg(Arg::with_name("sns_topic_arn")
                 .help("The ARN for the SNS topic you are subscribing to.")
                 .index(1)
                 .required(true))
        .arg(Arg::with_name("subscription_name")
                 .help("The name of the subscription.  Must be less that 255 characters.")
                 .index(2)
                 .required(true))
        .arg(Arg::with_name("disabled")
                 .help("Is this subscription disabled by default?")
                 .short("d")
                 .long("disabled"))
        .arg(Arg::with_name("category")
                 .help("An event category you wish to subscribe to.")
                 .short("c")
                 .long("category")
                 .multiple(true)
                 .takes_value(true)
                 .number_of_values(1)
                 .value_name("CATEGORY"))
        .arg(Arg::with_name("source_id")
                 .help("An event source id you wish to subscribe to.")
                 .short("s")
                 .long("source_id")
                 .multiple(true)
                 .takes_value(true)
                 .number_of_values(1)
                 .value_name("SOURCE_ID")
                 .requires("source_type"))
        .arg(Arg::with_name("source_type")
                 .help("The type of source that will be generating the events.")
                 .short("t")
                 .long("source_type")
                 .takes_value(true)
                 .value_name("SOURCE_TYPE"))
}

/// Event Create Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create RDS Event Subscriptions")
        .subcommand(subscription_subcommand())
}

/// Delete an Event Subscription
pub fn subscription(region: Region, matches: &ArgMatches) -> Result<()> {
    let mut stdout = term::stdout().ok_or_else(|| ErrorKind::CreateTerm)?;
    let provider = ProfileProvider::new()?;
    let tls_client = rusoto_core::default_tls_client()?;
    let client = RdsClient::new(tls_client, provider, region);
    let mut message: CreateEventSubscriptionMessage = Default::default();

    if let Some(arn) = matches.value_of("subscr_arn") {
        message.sns_topic_arn = arn.to_string();
    }

    message.enabled = Some(!matches.is_present("disabled"));

    let res_message = client.create_event_subscription(&message)?;

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
