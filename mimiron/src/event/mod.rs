//! Amazon RDS Event Actions
use clap::{App, ArgMatches, SubCommand};
use error::{ErrorKind, Result};
use rusoto_core::Region;

mod create;
mod delete;
mod describe;

/// The event submodule declaration.
pub fn subcommand<'a, 'b>(range: &'b [String]) -> App<'a, 'b> {
    SubCommand::with_name("event")
        .about("Work with Amazon AWS RDS Events")
        .subcommand(create::subcommand())
        .subcommand(delete::subcommand())
        .subcommand(describe::subcommand(range))
}

/// Event related dispatching
pub fn dispatch(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("create", Some(delete_matches)) => {
            match delete_matches.subcommand() {
                ("subscription", Some(subscr_matches)) => {
                    create::subscription(Region::UsEast2, subscr_matches)?;
                }
                _ => return Err(ErrorKind::InvalidCommand.into()),
            }
        }
        ("delete", Some(delete_matches)) => {
            match delete_matches.subcommand() {
                ("subscription", Some(subscr_matches)) => {
                    delete::subscriptions(Region::UsEast2, subscr_matches)?;
                }
                _ => return Err(ErrorKind::InvalidCommand.into()),
            }
        }
        ("describe", Some(describe_matches)) => {
            match describe_matches.subcommand() {
                ("categories", Some(categories_matches)) => {
                    describe::categories(Region::UsEast2, categories_matches)?;
                }
                ("events", Some(events_matches)) => {
                    describe::events(Region::UsEast2, events_matches)?;
                }
                ("subscriptions", Some(subscr_matches)) => {
                    describe::subscriptions(Region::UsEast2, subscr_matches)?;
                }
                _ => return Err(ErrorKind::InvalidCommand.into()),
            }
        }
        _ => return Err(ErrorKind::InvalidCommand.into()),
    }

    Ok(())
}
