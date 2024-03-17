/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeEntryActivityModelEmbedded {
    #[serde(rename = "projects")]
    pub projects: Vec<models::ProjectModel>,
}

impl TimeEntryActivityModelEmbedded {
    pub fn new(projects: Vec<models::ProjectModel>) -> TimeEntryActivityModelEmbedded {
        TimeEntryActivityModelEmbedded { projects }
    }
}
