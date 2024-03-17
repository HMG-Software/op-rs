/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageFileUploadLinkModelLinks {
    #[serde(rename = "self")]
    pub param_self: Box<models::StorageFileUploadLinkModelLinksSelf>,
    #[serde(rename = "destination")]
    pub destination: Box<models::StorageFileUploadLinkModelLinksDestination>,
}

impl StorageFileUploadLinkModelLinks {
    pub fn new(
        param_self: models::StorageFileUploadLinkModelLinksSelf,
        destination: models::StorageFileUploadLinkModelLinksDestination,
    ) -> StorageFileUploadLinkModelLinks {
        StorageFileUploadLinkModelLinks {
            param_self: Box::new(param_self),
            destination: Box::new(destination),
        }
    }
}
