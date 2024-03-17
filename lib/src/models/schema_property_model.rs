/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaPropertyModel {
    /// The resource type for this property.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The name of the property.
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates, if the property is required for submitting a request of this schema.
    #[serde(rename = "required")]
    pub required: bool,
    /// Indicates, if the property has a default.
    #[serde(rename = "hasDefault")]
    pub has_default: bool,
    /// Indicates, if the property is writable when sending a request of this schema.
    #[serde(rename = "writable")]
    pub writable: bool,
    /// Additional options for the property.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<serde_json::Value>,
    /// Defines the json path where the property is located in the payload.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Useful links for this property (e.g. an endpoint to fetch allowed values)
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    pub _links: Option<serde_json::Value>,
}

impl SchemaPropertyModel {
    pub fn new(
        r#type: String,
        name: String,
        required: bool,
        has_default: bool,
        writable: bool,
    ) -> SchemaPropertyModel {
        SchemaPropertyModel {
            r#type,
            name,
            required,
            has_default,
            writable,
            object: None,
            location: None,
            _links: None,
        }
    }
}
