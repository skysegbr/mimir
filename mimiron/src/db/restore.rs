//! Amazon AWS RDS DB Restore Actions
use clap::{App, SubCommand};

/// DBs Restore Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("restore").about("Restore RDS DB")
}
