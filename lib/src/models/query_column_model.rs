/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryColumnModel {
    /// Query column id
    #[serde(rename = "id")]
    pub id: String,
    /// Query column name
    #[serde(rename = "name")]
    pub name: String,
}

impl QueryColumnModel {
    pub fn new(id: String, name: String) -> QueryColumnModel {
        QueryColumnModel { id, name }
    }
}
