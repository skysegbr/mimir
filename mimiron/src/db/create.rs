//! Amazon AWS RDS DB Create Actions
use clap::{App, SubCommand};

/// DBs Create Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create").about("Create RDS DB")
}
