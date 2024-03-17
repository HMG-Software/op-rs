/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RevisionModelLinksAuthor {
    /// URL to the referenced resource (might be relative)
    #[serde(rename = "href", deserialize_with = "Option::deserialize")]
    pub href: Option<serde_json::Value>,
    /// Representative label for the resource
    #[serde(
        rename = "title",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<serde_json::Value>>,
    /// If true the href contains parts that need to be replaced by the client
    #[serde(
        rename = "templated",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub templated: Option<Option<serde_json::Value>>,
    /// The HTTP verb to use when requesting the resource
    #[serde(
        rename = "method",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub method: Option<Option<serde_json::Value>>,
    /// The payload to send in the request to achieve the desired result
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// An optional unique identifier to the link object
    #[serde(
        rename = "identifier",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub identifier: Option<Option<serde_json::Value>>,
    /// The MIME-Type of the returned resource.
    #[serde(
        rename = "type",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<Option<serde_json::Value>>,
}

impl RevisionModelLinksAuthor {
    pub fn new(href: Option<serde_json::Value>) -> RevisionModelLinksAuthor {
        RevisionModelLinksAuthor {
            href,
            title: None,
            templated: None,
            method: None,
            payload: None,
            identifier: None,
            r#type: None,
        }
    }
}
