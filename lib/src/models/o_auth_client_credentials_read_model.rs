/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthClientCredentialsReadModel {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "_type")]
    pub _type: Type,
    /// OAuth 2 client id
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// true, if OAuth 2 credentials are confidential, false, if no secret is stored
    #[serde(rename = "confidential")]
    pub confidential: bool,
    /// The time the OAuth client credentials were created at
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time the OAuth client credentials were last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "_links")]
    pub _links: Box<models::OAuthClientCredentialsReadModelLinks>,
}

impl OAuthClientCredentialsReadModel {
    pub fn new(
        id: i32,
        _type: Type,
        client_id: String,
        confidential: bool,
        _links: models::OAuthClientCredentialsReadModelLinks,
    ) -> OAuthClientCredentialsReadModel {
        OAuthClientCredentialsReadModel {
            id,
            _type,
            client_id,
            confidential,
            created_at: None,
            updated_at: None,
            _links: Box::new(_links),
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "OAuthClientCredentials")]
    OAuthClientCredentials,
}

impl Default for Type {
    fn default() -> Type {
        Self::OAuthClientCredentials
    }
}
