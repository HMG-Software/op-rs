/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateViewsRequestLinksQuery {
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl CreateViewsRequestLinksQuery {
    pub fn new() -> CreateViewsRequestLinksQuery {
        CreateViewsRequestLinksQuery { href: None }
    }
}
