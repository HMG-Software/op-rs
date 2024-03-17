/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkPackagesModelAllOfLinks {
    #[serde(rename = "self")]
    pub param_self: Box<models::WorkPackagesModelAllOfLinksSelf>,
}

impl WorkPackagesModelAllOfLinks {
    pub fn new(param_self: models::WorkPackagesModelAllOfLinksSelf) -> WorkPackagesModelAllOfLinks {
        WorkPackagesModelAllOfLinks {
            param_self: Box::new(param_self),
        }
    }
}
