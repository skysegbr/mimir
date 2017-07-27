// Copyright (c) 2017 mimiron developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `mimiron` runtime
use clap::{App, AppSettings, Arg, SubCommand};
use event;
use error::{ErrorKind, Result};
use rusoto_core::{default_tls_client, ProfileProvider, Region};
use rusoto_rds::{DBInstanceMessage, DescribeDBInstancesMessage, Rds, RdsClient,
                 StartDBInstanceMessage, StopDBInstanceMessage};
use std::collections::HashMap;
use std::iter;
use term;

/// The events submodule declaration.
fn events_submodule<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("event")
        .about("Work with Amazon AWS RDS Events")
        .subcommand(SubCommand::with_name("describe")
                        .about("Describe RDS Events")
                        .subcommand(SubCommand::with_name("categories")
                                        .about("Describe RDS Event Categories")
                                        .arg(Arg::with_name("source_type")
                                                 .help("A source type to filter on.")
                                                 .possible_values(&["db-instance",
                                                                    "db-parameter-group",
                                                                    "db-security-group",
                                                                    "db-snapshot",
                                                                    "db-cluster",
                                                                    "db-cluster-snapshot"])))
                        .subcommand(SubCommand::with_name("events"))
                        .subcommand(SubCommand::with_name("subscriptions")
                                        .about("Describe RDS Event Subscriptions")))
}

/// CLI Runtime
pub fn run() -> Result<i32> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Manage Oracle RDS instances.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("create")
                        .about("Create an Oracle RDS instance.")
                        .arg(Arg::with_name("instance_id").help("The unique instance identifier")))
        .subcommand(events_submodule())
        .subcommand(SubCommand::with_name("start")
                        .about("Start an RDS instance with the given identifier")
                        .arg(Arg::with_name("instance_id")
                                 .help("The unique instance identifier")
                                 .required(true)))
        .subcommand(SubCommand::with_name("status")
                        .about("Display instance status information.")
                        .arg(Arg::with_name("instance_id").help("The unique instance identifier")))
        .subcommand(SubCommand::with_name("stop")
                        .about("Stop an RDS instance with the given identifier")
                        .arg(Arg::with_name("instance_id")
                                 .help("The unique instance identifier")
                                 .required(true))
                        .arg(Arg::with_name("snapshot_id")
                                 .help("An optional pre-shutdown snapshot identifier")))
        .get_matches();

    let mut stderr = term::stderr().ok_or_else(|| ErrorKind::CreateTerm)?;
    let provider = ProfileProvider::new()?;
    let client = RdsClient::new(default_tls_client()?, provider, Region::UsEast2);

    if let Some(start_matches) = matches.subcommand_matches("create") {
        if let Some(instance_id) = start_matches.value_of("instance_id") {
            let mut start_message: StartDBInstanceMessage = Default::default();
            start_message.db_instance_identifier = instance_id.to_string();
            let _db_instance = client.start_db_instance(&start_message)?;
        }
    } else if let Some(event_matches) = matches.subcommand_matches("event") {
        match event_matches.subcommand() {
            ("describe", Some(describe_matches)) => {
                match describe_matches.subcommand() {
                    ("categories", Some(categories_matches)) => {
                        event::describe::categories(Region::UsEast2, categories_matches)?;
                    }
                    ("events", Some(events_matches)) => {
                        event::describe::events(Region::UsEast2, events_matches)?;
                    }
                    ("subscriptions", Some(subscr_matches)) => {
                        event::describe::subscriptions(Region::UsEast2, subscr_matches)?;
                    }
                    _ => return Err(ErrorKind::InvalidCommand.into()),
                }
            }
            _ => return Err(ErrorKind::InvalidCommand.into()),
        }
    } else if let Some(start_matches) = matches.subcommand_matches("start") {
        if let Some(instance_id) = start_matches.value_of("instance_id") {
            let mut start_message: StartDBInstanceMessage = Default::default();
            start_message.db_instance_identifier = instance_id.to_string();
            let _db_instance = client.start_db_instance(&start_message)?;
        }
    } else if let Some(stop_matches) = matches.subcommand_matches("stop") {
        if let Some(instance_id) = stop_matches.value_of("instance_id") {
            let mut stop_message: StopDBInstanceMessage = Default::default();
            stop_message.db_instance_identifier = instance_id.to_string();
            stop_message.db_snapshot_identifier =
                stop_matches.value_of("snapshot_id").map(|s| s.to_string());
            let _db_instance = client.stop_db_instance(&stop_message)?;
        }
    } else if let Some(status_matches) = matches.subcommand_matches("status") {
        let mut describe_message: DescribeDBInstancesMessage = Default::default();
        describe_message.db_instance_identifier = status_matches
            .value_of("instance_id")
            .map(|s| s.to_string());
        let instance_message = client.describe_db_instances(&describe_message)?;
        status(&instance_message)?;
    } else {
        stderr.fg(term::color::RED)?;
        stderr.attr(term::Attr::Bold)?;
        writeln!(stderr, "Unknown command!")?;
    }

    Ok(0)
}

/// Output the status messages.
fn status(message: &DBInstanceMessage) -> Result<()> {
    let mut stdout = term::stdout().ok_or_else(|| ErrorKind::CreateTerm)?;

    if let Some(ref instances) = message.db_instances {
        let mut statuses = HashMap::new();
        let mut max_len = 0;
        for instance in instances {
            if let Some(ref id) = instance.db_instance_identifier {
                if id.len() > max_len {
                    max_len = id.len();
                }
                let status = if let Some(ref status) = instance.db_instance_status {
                    status
                } else {
                    ""
                };
                statuses.insert(id, status);
            }
        }

        for (k, v) in statuses {
            let num_spaces = max_len - k.len();
            let mut output = String::new();

            if num_spaces > 0 {
                let spaces: String = iter::repeat(' ').take(num_spaces).collect();
                output.push_str(&spaces);
            }
            output.push_str(k);
            stdout.fg(term::color::GREEN)?;
            stdout.attr(term::Attr::Bold)?;
            write!(stdout, "{}:", output)?;
            stdout.reset()?;
            stdout.flush()?;
            stdout.fg(term::color::GREEN)?;
            writeln!(stdout, " {}", v)?;
        }
    }

    Ok(())
}
