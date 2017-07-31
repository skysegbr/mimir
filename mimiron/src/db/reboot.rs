//! Amazon AWS RDS DB Reboot Actions
use clap::{App, SubCommand};

/// DBs Reboot Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("reboot").about("Reboot RDS DB")
}
