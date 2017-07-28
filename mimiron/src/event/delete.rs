//! Amazon AWS RDS Event Delete Actions
use clap::{App, SubCommand};

/// Delete Event Subscription subcommand.
fn subscription_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("subscription").about("Delete RDS Event Subscription")
}

/// Event Delete Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete RDS Event Subscriptions")
        .subcommand(subscription_subcommand())
}
