//! Amazon AWS RDS DB Delete Actions
use clap::{App, SubCommand};

/// DBs Delete Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete").about("Delete RDS DB")
}
