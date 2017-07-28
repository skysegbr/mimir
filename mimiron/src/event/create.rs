//! Amazon AWS RDS Event Create Actions
use clap::{App, SubCommand};

/// Create Event Subscriptions subcommand.
fn subscription_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("subscription").about("Create RDS Event Subscription")
}

/// Event Create Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create RDS Event Subscriptions")
        .subcommand(subscription_subcommand())
}
