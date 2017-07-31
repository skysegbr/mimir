//! Amazon AWS RDS DB Reset Actions
use clap::{App, SubCommand};

/// DBs Reset Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("reset").about("Reset RDS DB")
}
