/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridCollectionModelAllOfEmbedded {
    #[serde(rename = "elements")]
    pub elements: Vec<models::GridReadModel>,
}

impl GridCollectionModelAllOfEmbedded {
    pub fn new(elements: Vec<models::GridReadModel>) -> GridCollectionModelAllOfEmbedded {
        GridCollectionModelAllOfEmbedded { elements }
    }
}
