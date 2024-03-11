// https://www.openproject.org/docs/api/endpoints/time-entries/

use chrono::{DateTime, Duration, TimeDelta, Utc};
use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
struct Timesheet {
    activity: String,
    // #[serde(with = "ISO8601DateTime")]
    created_at: DateTime<Utc>,
    // #[serde(with = "OpenProjectText")]
    comment: String,
    // #[serde(with = "ISO8601DateTime")]
    //pub hours: Duration,
    id: isize,
    // #[serde(with = "ISO8601DateTime")]
    pub spent_on: DateTime<Utc>,
    // #[serde(default="me")]
    user: String,
    // #[serde(with = "OpenProjectLinkTitle")]
    work_package: isize,
}

// https://serde.rs/custom-date-format.html
