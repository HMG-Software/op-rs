/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DayCollectionModelAllOfLinks {
    #[serde(rename = "self")]
    pub param_self: Box<models::DayCollectionModelAllOfLinksSelf>,
}

impl DayCollectionModelAllOfLinks {
    pub fn new(
        param_self: models::DayCollectionModelAllOfLinksSelf,
    ) -> DayCollectionModelAllOfLinks {
        DayCollectionModelAllOfLinks {
            param_self: Box::new(param_self),
        }
    }
}
