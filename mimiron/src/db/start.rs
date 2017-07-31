//! Amazon AWS RDS DB Start Actions
use clap::{App, SubCommand};

/// DBs Start Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("start").about("Start RDS DB")
}
