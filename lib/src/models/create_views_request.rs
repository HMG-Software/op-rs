/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateViewsRequest {
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    pub _links: Option<Box<models::CreateViewsRequestLinks>>,
}

impl CreateViewsRequest {
    pub fn new() -> CreateViewsRequest {
        CreateViewsRequest { _links: None }
    }
}
