/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentWorkPackageRequest {
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Box<models::UpdateActivityRequestComment>>,
}

impl CommentWorkPackageRequest {
    pub fn new() -> CommentWorkPackageRequest {
        CommentWorkPackageRequest { comment: None }
    }
}
