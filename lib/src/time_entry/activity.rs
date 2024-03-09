// https://www.openproject.org/docs/api/endpoints/time-entry-activities/

use serde::Deserialize;


#[derive(Debug, serde::Deserialize)]
struct Timesheet {
    id: i8,
    name: String,
    position: i8,
    default: bool,
    #[serde(with = "OpenProjectLinkTitle")]
    projects: vec<OPLink>,
}
