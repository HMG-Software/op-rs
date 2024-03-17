/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MembershipSchemaModel {
    #[serde(rename = "_type", deserialize_with = "Option::deserialize")]
    pub _type: Option<Type>,
    /// A list of dependencies between one property's value and another property
    #[serde(
        rename = "_dependencies",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub _dependencies: Option<Option<serde_json::Value>>,
    #[serde(rename = "_links")]
    pub _links: Box<models::SchemaModelLinks>,
    #[serde(rename = "id")]
    pub id: Box<models::SchemaPropertyModel>,
    #[serde(rename = "createdAt")]
    pub created_at: Box<models::SchemaPropertyModel>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Box<models::SchemaPropertyModel>,
    #[serde(rename = "notificationMessage")]
    pub notification_message: Box<models::SchemaPropertyModel>,
    #[serde(rename = "project")]
    pub project: Box<models::SchemaPropertyModel>,
    #[serde(rename = "principal")]
    pub principal: Box<models::SchemaPropertyModel>,
    #[serde(rename = "roles")]
    pub roles: Box<models::SchemaPropertyModel>,
}

impl MembershipSchemaModel {
    pub fn new(
        _type: Option<Type>,
        _links: models::SchemaModelLinks,
        id: models::SchemaPropertyModel,
        created_at: models::SchemaPropertyModel,
        updated_at: models::SchemaPropertyModel,
        notification_message: models::SchemaPropertyModel,
        project: models::SchemaPropertyModel,
        principal: models::SchemaPropertyModel,
        roles: models::SchemaPropertyModel,
    ) -> MembershipSchemaModel {
        MembershipSchemaModel {
            _type,
            _dependencies: None,
            _links: Box::new(_links),
            id: Box::new(id),
            created_at: Box::new(created_at),
            updated_at: Box::new(updated_at),
            notification_message: Box::new(notification_message),
            project: Box::new(project),
            principal: Box::new(principal),
            roles: Box::new(roles),
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Schema")]
    Schema,
}

impl Default for Type {
    fn default() -> Type {
        Self::Schema
    }
}
