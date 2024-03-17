/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WikiPageModelLinks {
    #[serde(rename = "addAttachment", skip_serializing_if = "Option::is_none")]
    pub add_attachment: Option<Box<models::WikiPageModelLinksAddAttachment>>,
}

impl WikiPageModelLinks {
    pub fn new() -> WikiPageModelLinks {
        WikiPageModelLinks {
            add_attachment: None,
        }
    }
}
