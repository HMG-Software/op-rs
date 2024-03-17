/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupModel {
    /// The group id
    #[serde(rename = "id")]
    pub id: i32,
    /// Group's full name, formatting depends on instance settings  # Conditions - admin
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Time of creation  # Conditions - admin
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Time of the most recent change to the user
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "_links")]
    pub _links: Box<models::GroupModelLinks>,
}

impl GroupModel {
    pub fn new(id: i32, _links: models::GroupModelLinks) -> GroupModel {
        GroupModel {
            id,
            name: None,
            created_at: None,
            updated_at: None,
            _links: Box::new(_links),
        }
    }
}

