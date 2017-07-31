//! Amazon AWS RDS DB Description Actions
use clap::{App, SubCommand};

/// DBs Description Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("describe").about("Describe RDS DB")
}
