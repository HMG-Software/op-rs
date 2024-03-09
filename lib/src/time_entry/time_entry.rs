// https://www.openproject.org/docs/api/endpoints/time-entries/

use serde::Deserialize;


#[derive(Debug, serde::Deserialize)]
struct Timesheet {
    activity: String,
    #[serde(with = "ISO8601DateTime")]
    created_at: DateTime<Utc>,
    #[serde(with = "OpenProjectText")]
    comment: String,
    #[serde(with = "ISO8601DateTime")]
    pub hours: DateTime<Utc>,
    id: i8,
    #[serde(with = "ISO8601DateTime")]
    pub spent_on: DateTime<Utc>,
    ticket: i8,
    #[serde(default="me")]
    user: String,
    #[serde(with = "OpenProjectLinkTitle")]
    work_package: String,
}


// https://serde.rs/custom-date-format.html
