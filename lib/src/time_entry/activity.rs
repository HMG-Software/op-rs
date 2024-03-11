// https://www.openproject.org/docs/api/endpoints/time-entry-activities/

use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
struct Timesheet {
    id: isize,
    name: String,
    position: isize,
    default: bool,
    // #[serde(with = "OpenProjectLinkTitle")]
    // projects: vec<crate::schema::Link>,
}
