/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrincipalModel {
    #[serde(rename = "_type")]
    pub _type: Type,
    /// The principal's unique identifier.
    #[serde(rename = "id")]
    pub id: i32,
    /// The principal's display name, layout depends on instance settings.
    #[serde(rename = "name")]
    pub name: String,
    /// Time of creation
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Time of the most recent change to the principal
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "_links")]
    pub _links: Box<models::PrincipalModelLinks>,
}

impl PrincipalModel {
    pub fn new(
        _type: Type,
        id: i32,
        name: String,
        _links: models::PrincipalModelLinks,
    ) -> PrincipalModel {
        PrincipalModel {
            _type,
            id,
            name,
            created_at: None,
            updated_at: None,
            _links: Box::new(_links),
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Group")]
    Group,
    #[serde(rename = "PlaceholderUser")]
    PlaceholderUser,
}

impl Default for Type {
    fn default() -> Type {
        Self::User
    }
}
