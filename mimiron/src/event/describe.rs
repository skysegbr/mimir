//! Amazon AWS RDS Event Description Actions
use SOURCE_TYPES;
use clap::{App, Arg, ArgMatches, SubCommand};
use error::{ErrorKind, Result};
use rusoto_core::{self, ProfileProvider, Region};
use rusoto_rds::{DescribeEventsMessage, DescribeEventCategoriesMessage,
                 DescribeEventSubscriptionsMessage, Rds, RdsClient};
use std::ops::Deref;
use term;

/// Describe Event Categories subcommand.
fn categories_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("categories")
        .about("Describe RDS Event Categories")
        .arg(Arg::with_name("source_type")
                 .help("A source type to filter on.")
                 .possible_values(&SOURCE_TYPES))
}

/// Describe Events subcommand.
fn events_subcommand<'a, 'b>(range: &'b [String]) -> App<'a, 'b> {
    let max_records_vec: Vec<&str> = range.iter().map(Deref::deref).collect();
    SubCommand::with_name("events")
        .about("Describe RDS Events")
        .arg(Arg::with_name("duration")
                 .help("The duration in minutes to search.")
                 .short("d")
                 .long("duration")
                 .takes_value(true)
                 .value_name("DURATION")
                 .conflicts_with_all(&["start_time", "end_time"]))
        .arg(Arg::with_name("start_time")
                 .help("A start time (YYYY-MM-DDTHH:MMZ) to search from.")
                 .short("s")
                 .long("start_time")
                 .takes_value(true)
                 .value_name("START_TIME"))
        .arg(Arg::with_name("end_time")
                 .help("An end time (YYYY-MM-DDTHH:MMZ) to search until.")
                 .short("e")
                 .long("end_time")
                 .takes_value(true)
                 .value_name("END_TIME"))
        .arg(Arg::with_name("max_records")
                 .help("The maximum number of records to return. (20 - 100)")
                 .short("m")
                 .long("max_records")
                 .takes_value(true)
                 .value_name("MAX_RECORDS")
                 .possible_values(&max_records_vec[..])
                 .hide_possible_values(true))
        .arg(Arg::with_name("source_type")
                 .help("A source type to filter on.")
                 .short("t")
                 .long("source_type")
                 .takes_value(true)
                 .value_name("SOURCE_TYPE")
                 .possible_values(&SOURCE_TYPES))
        .arg(Arg::with_name("source_identifier")
                 .help("A source identifier to filter on.")
                 .short("i")
                 .long("source_id")
                 .takes_value(true)
                 .requires("source_type")
                 .value_name("SOURCE_IDENTIFIER"))
        .arg(Arg::with_name("marker")
                 .help("Specify a marker from a previous response to start search from.")
                 .short("k")
                 .long("marker")
                 .takes_value(true)
                 .value_name("MARKER"))
        .arg(Arg::with_name("categories")
                 .help("Event categories to filter on.")
                 .multiple(true))
}

/// Describe Event Subscriptions subcommand.
fn subscription_subcommand<'a, 'b>(range: &'b [String]) -> App<'a, 'b> {
    let max_records_vec: Vec<&str> = range.iter().map(Deref::deref).collect();
    SubCommand::with_name("subscriptions")
        .about("Describe RDS Event Subscriptions")
        .arg(Arg::with_name("max_records")
                 .help("The maximum number of records to return. (20 - 100)")
                 .short("m")
                 .long("max_records")
                 .takes_value(true)
                 .value_name("MAX_RECORDS")
                 .possible_values(&max_records_vec[..])
                 .hide_possible_values(true))
        .arg(Arg::with_name("marker")
                 .help("Specify a marker from a previous response to start search from.")
                 .short("k")
                 .long("marker")
                 .takes_value(true)
                 .value_name("MARKER"))
        .arg(Arg::with_name("name").help("The name of the subscription you wish to describe"))
}

/// Event Description Submodule
pub fn subcommand<'a, 'b>(range: &'b [String]) -> App<'a, 'b> {
    SubCommand::with_name("describe")
        .about("Describe RDS Event Categories, Events, and Event Subscriptions")
        .subcommand(categories_subcommand())
        .subcommand(events_subcommand(range))
        .subcommand(subscription_subcommand(range))
}

/// Description for Events
pub fn events(region: Region, matches: &ArgMatches) -> Result<()> {
    let mut stdout = term::stdout().ok_or_else(|| ErrorKind::CreateTerm)?;
    let provider = ProfileProvider::new()?;
    let tls_client = rusoto_core::default_tls_client()?;
    let client = RdsClient::new(tls_client, provider, region);
    let mut message: DescribeEventsMessage = Default::default();

    if let Some(duration_str) = matches.value_of("duration") {
        message.duration = Some(duration_str.parse()?);
    }

    if let Some(start_time) = matches.value_of("start_time") {
        message.start_time = Some(start_time.to_string());
    }

    if let Some(end_time) = matches.value_of("end_time") {
        message.end_time = Some(end_time.to_string());
    }

    if let Some(max_records) = matches.value_of("max_records") {
        message.max_records = Some(max_records.parse()?);
    }

    if let Some(source_type) = matches.value_of("source_type") {
        message.source_type = Some(source_type.to_string());
    }

    if let Some(source_identifier) = matches.value_of("source_identifier") {
        message.source_identifier = Some(source_identifier.to_string());
    }

    if let Some(categories) = matches.values_of("categories") {
        let categories_vec: Vec<&str> = categories.collect();
        message.event_categories = Some(categories_vec.iter().map(|x| x.to_string()).collect());
    }

    if let Some(marker) = matches.value_of("marker") {
        message.marker = Some(marker.to_string());
    }

    let out_message = client.describe_events(&message)?;

    if let Some(events_list) = out_message.events {
        for event in events_list {
            stdout.fg(term::color::GREEN)?;
            stdout.attr(term::Attr::Bold)?;
            write!(stdout, "{}:", or_none!(event.date))?;
            stdout.reset()?;
            stdout.flush()?;
            stdout.fg(term::color::GREEN)?;

            let mut categories_str = String::new();
            try_join!(categories_str, event.event_categories);

            writeln!(stdout,
                     " {} {} {} - {}",
                     or_none!(event.source_type),
                     categories_str,
                     or_none!(event.source_identifier),
                     or_none!(event.message))?;
            stdout.reset()?;
            stdout.flush()?;
        }
    }

    if let Some(marker) = out_message.marker {
        stdout.fg(term::color::GREEN)?;
        stdout.attr(term::Attr::Bold)?;
        writeln!(stdout, "Marker: {}", marker)?;
        stdout.reset()?;
        stdout.flush()?;
    }

    Ok(())
}

/// Desciption for Event Categories
pub fn categories(region: Region, matches: &ArgMatches) -> Result<()> {
    let mut stdout = term::stdout().ok_or_else(|| ErrorKind::CreateTerm)?;
    let provider = ProfileProvider::new()?;
    let tls_client = rusoto_core::default_tls_client()?;
    let client = RdsClient::new(tls_client, provider, region);
    let mut message: DescribeEventCategoriesMessage = Default::default();

    if let Some(source_type) = matches.value_of("source_type") {
        message.source_type = Some(source_type.to_string());
    }

    let out_message = client.describe_event_categories(&message)?;

    if let Some(event_categories_map_list) = out_message.event_categories_map_list {
        for event_categories_map in event_categories_map_list {
            if let Some(source_type) = event_categories_map.source_type {
                writeln!(stdout, "{}", source_type)?;
                if let Some(event_categories_list) = event_categories_map.event_categories {
                    let len = event_categories_list.len();
                    for (idx, event_category) in event_categories_list.iter().enumerate() {
                        if idx < len - 1 {
                            writeln!(stdout, "  ├─ {}", event_category)?;
                        } else {
                            writeln!(stdout, "  └─ {}", event_category)?;
                        }
                    }

                }
                writeln!(stdout, "")?;
            }
        }
    }

    Ok(())
}

/// Description for Event Subscriptions
pub fn subscriptions(region: Region, matches: &ArgMatches) -> Result<()> {
    let mut stdout = term::stdout().ok_or_else(|| ErrorKind::CreateTerm)?;
    let provider = ProfileProvider::new()?;
    let tls_client = rusoto_core::default_tls_client()?;
    let client = RdsClient::new(tls_client, provider, region);
    let mut message: DescribeEventSubscriptionsMessage = Default::default();

    if let Some(max_records) = matches.value_of("max_records") {
        message.max_records = Some(max_records.parse()?);
    }

    if let Some(marker) = matches.value_of("marker") {
        message.marker = Some(marker.to_string());
    }

    if let Some(name) = matches.value_of("name") {
        message.subscription_name = Some(name.to_string());
    }

    let out_message = client.describe_event_subscriptions(&message)?;

    if let Some(event_subscriptions_list) = out_message.event_subscriptions_list {
        for subscription in event_subscriptions_list {
            stdout.fg(term::color::GREEN)?;
            stdout.attr(term::Attr::Bold)?;
            write!(stdout,
                   "{}:",
                   or_none!(subscription.subscription_creation_time))?;
            stdout.reset()?;
            stdout.flush()?;
            stdout.fg(term::color::GREEN)?;

            let mut categories_str = String::new();
            try_join!(categories_str, subscription.event_categories_list);

            let mut source_ids_str = String::new();
            try_join!(source_ids_str, subscription.source_ids_list);

            write!(stdout, " {}", or_none!(subscription.source_type))?;

            if !categories_str.is_empty() {
                write!(stdout, " {}", categories_str)?;
            }

            if !source_ids_str.is_empty() {
                write!(stdout, " {}", source_ids_str)?;
            }

            writeln!(stdout,
                     " {} {} {}",
                     or_none!(subscription.cust_subscription_id),
                     or_none!(b => subscription.enabled),
                     or_none!(subscription.sns_topic_arn))?;
            stdout.reset()?;
            stdout.flush()?;
        }
    }

    if let Some(marker) = out_message.marker {
        stdout.fg(term::color::GREEN)?;
        stdout.attr(term::Attr::Bold)?;
        writeln!(stdout, "Marker: {}", marker)?;
        stdout.reset()?;
        stdout.flush()?;
    }

    Ok(())
}
