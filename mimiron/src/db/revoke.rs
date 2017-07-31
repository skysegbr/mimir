//! Amazon AWS RDS DB Revoke Actions
use clap::{App, SubCommand};

/// DBs Revoke Submodule
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("revoke").about("Revoke RDS DB")
}
