//! Amazon RDS DB Actions
use clap::{App, SubCommand};

mod copy;
mod create;
mod delete;
mod describe;
mod modify;
mod reboot;
mod reset;
mod restore;
mod revoke;
mod start;
mod stop;

/// The event submodule declaration.
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("db")
        .about("Work with Amazon AWS RDS DBs")
        .subcommand(copy::subcommand())
        .subcommand(create::subcommand())
        .subcommand(delete::subcommand())
        .subcommand(describe::subcommand())
        .subcommand(modify::subcommand())
        .subcommand(reboot::subcommand())
        .subcommand(reset::subcommand())
        .subcommand(restore::subcommand())
        .subcommand(revoke::subcommand())
        .subcommand(start::subcommand())
        .subcommand(stop::subcommand())
}
