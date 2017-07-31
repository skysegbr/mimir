//! Amazon AWS RDS DB Copy Actions
use clap::{App, SubCommand};

/// DBs Copy Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("copy").about("Copy RDS DB")
}
