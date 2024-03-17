/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MembershipReadModelEmbedded {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<models::ProjectModel>>,
    #[serde(rename = "principal", skip_serializing_if = "Option::is_none")]
    pub principal: Option<Box<models::MembershipReadModelEmbeddedPrincipal>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<models::RoleModel>>,
}

impl MembershipReadModelEmbedded {
    pub fn new() -> MembershipReadModelEmbedded {
        MembershipReadModelEmbedded {
            project: None,
            principal: None,
            roles: None,
        }
    }
}
