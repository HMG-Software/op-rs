/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeEntryCollectionModel {
    #[serde(rename = "_type", deserialize_with = "Option::deserialize")]
    pub _type: Option<Type>,
    /// The total amount of elements available in the collection.
    #[serde(rename = "total", deserialize_with = "Option::deserialize")]
    pub total: Option<serde_json::Value>,
    /// Actual amount of elements in this response.
    #[serde(rename = "count", deserialize_with = "Option::deserialize")]
    pub count: Option<serde_json::Value>,
    #[serde(rename = "_links")]
    pub _links: Box<models::TimeEntryCollectionModelAllOfLinks>,
    #[serde(rename = "_embedded")]
    pub _embedded: Box<models::TimeEntryCollectionModelAllOfEmbedded>,
}

impl TimeEntryCollectionModel {
    pub fn new(
        _type: Option<Type>,
        total: Option<serde_json::Value>,
        count: Option<serde_json::Value>,
        _links: models::TimeEntryCollectionModelAllOfLinks,
        _embedded: models::TimeEntryCollectionModelAllOfEmbedded,
    ) -> TimeEntryCollectionModel {
        TimeEntryCollectionModel {
            _type,
            total,
            count,
            _links: Box::new(_links),
            _embedded: Box::new(_embedded),
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Collection")]
    Collection,
}

impl Default for Type {
    fn default() -> Type {
        Self::Collection
    }
}
