use chrono::{DateTime, TimeDelta, Utc};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

// const DURATION_REGEX: &str = r"(?:(?P<hours>\d*\.?\d+?)h)?(?:(?P<minutes>\d*\.?\d+?)m)?";

/// Work with OpenProject Time Entries (Timesheets)
#[derive(Debug, Subcommand)]
pub(crate) enum TimeEntry {
    #[command(alias = "c")]
    Create(Create),

    #[command(alias = "f", arg_required_else_help = true)]
    Fetch(Fetch),
}

/// Create a new time entry
/// either by providing a CSV file or
/// or by providing the required args
#[derive(Args, Debug)]
pub(crate) struct Create {
    /// Activity name or ID
    #[arg(
        long,
        requires_all = &["hours", "work_package"],
        short = 'a',
        // value_parser = parse_activity,
    )]
    activity: Option<String>,

    /// Comment describing the time entry
    #[arg(long, short = 'm')]
    comment: Option<String>,

    /// Time spent on the work package
    #[arg(
        aliases = &["delta", "duration"],
        long,
        short = 'd',
        value_name = "DURATION",
        value_parser = parse_hours,
    )]
    hours: Option<TimeDelta>,

    #[arg(
        long,
        short = None,
        value_name = "DATE_TIME",
    )]
    spent_on: Option<DateTime<Utc>>,

    #[arg(
        default_value = Some(default_user()),
        long,
        requires_all = &["activity", "hours", "work_package"],
        short = 'u',
        value_name = "USER_ID",
    )]
    user: Option<String>,

    #[arg(
        aliases = &["wp", "ticket"],
        long,
        requires_all = &["activity", "hours"],
        short = None,
        value_name = "WORK_PACKAGE_ID",
    )]
    work_package: Option<isize>,

    /// Path to the CSV file
    /// containing i or more time entries
    #[arg(
        // either provide a CSV file or the other (required or optional) args
        exclusive = true,
        long,
        required = false,
        short = 'f',
        value_name = "CSV_FILE",
        value_hint = clap::ValueHint::FilePath,
    )]
    csv_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub(crate) struct Fetch {
    #[arg(
        default_value = default_user(),
        long,
        required = true,
        short = None,
    )]
    user: Option<String>,

    #[arg(
        long,
        required = true,
        short = None,
    )]
    work_package: Option<isize>,
}

/// parse an input string into TimeDelta
///
/// possible input format examples:
/// - "45m" (45 minutes)
/// - "1" (1 hour)
/// - "1h" (1 hour)
/// - "1.5" (1.5 hours)
/// - "1.5h" (1.5 hours)
/// - "1h30" (1 hour, 30 minutes)
/// - "1h30m" (1 hour, 30 minutes)
/// - ".25" (45 minutes)
/// - ".75h" (45 minutes)
/// - ":30" (30 minutes)
/// - ":30m" (30 minutes)
/// - "1:30" (1 hour, 30 minutes)
/// - "PT30M" (30 minutes)
/// - "PT1H30M" (1 hour, 30 minutes)
/// - "PT1H" (1 hour)
pub(crate) fn parse_hours(s: &str) -> Result<TimeDelta, String> {
    let seconds = 900;

    let delta = TimeDelta::new(seconds, 0).ok_or("Time delta is 0")?;

    #[cfg(feature = "config")]
    validate_hours(delta)?;

    Ok(delta)
}

pub(crate) fn default_user() -> &'static str {
    #[cfg(feature = "config")]
    {
        println!("DEMO: get pre-defined user from config")
    }
    "me"
}

#[cfg(feature = "config")]
fn validate_hours(delta: TimeDelta) -> Result<(), String> {
    // if feature "config" is enabled, throw an error if time is not evenly divisible by 15 minutes
    // (15 minutes == 900 seconds)
    if delta.num_seconds() % 900 != 0 {
        return Err("Time must be evenly divisible by 15 minutes".to_string());
    }
    Ok(())
}
