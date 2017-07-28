//! Amazon RDS Event Actions
use clap::{App, SubCommand};

pub mod create;
pub mod delete;
pub mod describe;

/// The event submodule declaration.
pub fn subcommand<'a, 'b>(range: &'b [String]) -> App<'a, 'b> {
    SubCommand::with_name("event")
        .about("Work with Amazon AWS RDS Events")
        .subcommand(create::subcommand())
        .subcommand(delete::subcommand())
        .subcommand(describe::subcommand(range))
}
