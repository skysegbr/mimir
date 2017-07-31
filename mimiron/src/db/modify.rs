//! Amazon AWS RDS DB Modify Actions
use clap::{App, SubCommand};

/// DBs Modify Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("modify").about("Modify RDS DB")
}
