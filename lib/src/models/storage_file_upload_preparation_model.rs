/*
 * OpenProject API V3 (Stable)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageFileUploadPreparationModel {
    /// The project identifier, from where a user starts uploading a file.
    #[serde(rename = "projectId")]
    pub project_id: i32,
    /// The file name.
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// The directory to which the file is to be uploaded. For root directories, the value `/` must be provided.
    #[serde(rename = "parent")]
    pub parent: String,
}

impl StorageFileUploadPreparationModel {
    pub fn new(
        project_id: i32,
        file_name: String,
        parent: String,
    ) -> StorageFileUploadPreparationModel {
        StorageFileUploadPreparationModel {
            project_id,
            file_name,
            parent,
        }
    }
}
