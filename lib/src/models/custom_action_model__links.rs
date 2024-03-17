/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomActionModelLinks {
    #[serde(rename = "self")]
    pub param_self: Box<models::CustomActionModelLinksSelf>,
    #[serde(rename = "executeImmediately")]
    pub execute_immediately: Box<models::CustomActionModelLinksExecuteImmediately>,
}

impl CustomActionModelLinks {
    pub fn new(
        param_self: models::CustomActionModelLinksSelf,
        execute_immediately: models::CustomActionModelLinksExecuteImmediately,
    ) -> CustomActionModelLinks {
        CustomActionModelLinks {
            param_self: Box::new(param_self),
            execute_immediately: Box::new(execute_immediately),
        }
    }
}
