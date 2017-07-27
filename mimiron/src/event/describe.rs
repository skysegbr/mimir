//! Amazon AWS RDS Event Description Actions
use clap::ArgMatches;
use error::{ErrorKind, Result};
use rusoto_core::{self, ProfileProvider, Region};
use rusoto_rds::{DescribeEventsMessage, DescribeEventCategoriesMessage, Rds, RdsClient};
use term;

/// Description for Events
pub fn events(region: Region, _matches: &ArgMatches) -> Result<()> {
    let mut stdout = term::stdout().ok_or_else(|| ErrorKind::CreateTerm)?;
    let provider = ProfileProvider::new()?;
    let tls_client = rusoto_core::default_tls_client()?;
    let client = RdsClient::new(tls_client, provider, region);
    let mut message: DescribeEventsMessage = Default::default();

    // TODO: mutate message based on matches.
    message.duration = Some(1440);

    let out_message = client.describe_events(&message)?;

    if let Some(events_list) = out_message.events {
        for event in events_list {
            stdout.fg(term::color::GREEN)?;
            stdout.attr(term::Attr::Bold)?;
            write!(stdout,
                   "{}:",
                   event.date.unwrap_or_else(|| "none".to_string()))?;
            stdout.reset()?;
            stdout.flush()?;
            stdout.fg(term::color::GREEN)?;
            let mut categories_str = String::from("[ ");
            if let Some(categories) = event.event_categories {
                for category in categories {
                    categories_str.push_str(&category);
                    categories_str.push(',');
                }
            }
            let mut trimmed_cs = String::from(categories_str.trim_right_matches(','));
            trimmed_cs.push_str(" ]");

            writeln!(stdout,
                     " {} {} {} - {}",
                     event.source_type.unwrap_or_else(|| "none".to_string()),
                     trimmed_cs,
                     event
                         .source_identifier
                         .unwrap_or_else(|| "none".to_string()),
                     event.message.unwrap_or_else(|| "none".to_string()))?;
        }
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
pub fn subscriptions(_region: Region, _matches: &ArgMatches) -> Result<()> {
    Ok(())
}
