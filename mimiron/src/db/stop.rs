//! Amazon AWS RDS DB Stop Actions
use clap::{App, SubCommand};

/// DBs Stop Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("stop").about("Stop RDS DB")
}
