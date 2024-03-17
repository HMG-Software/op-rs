# StorageFileModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | Linked file's id on the origin | 
**name** | Option<[**serde_json::Value**](.md)> | Linked file's name on the origin | 
**mime_type** | Option<[**serde_json::Value**](.md)> | MIME type of the linked file.  To link a folder entity, the custom MIME type `application/x-op-directory` MUST be provided. Otherwise it defaults back to an unknown MIME type. | [optional]
**size** | Option<[**serde_json::Value**](.md)> | file size on origin in bytes | [optional]
**created_at** | Option<[**serde_json::Value**](.md)> | Timestamp of the creation datetime of the file on the origin | [optional]
**last_modified_at** | Option<[**serde_json::Value**](.md)> | Timestamp of the datetime of the last modification of the file on the origin | [optional]
**created_by_name** | Option<[**serde_json::Value**](.md)> | Display name of the author that created the file on the origin | [optional]
**last_modified_by_name** | Option<[**serde_json::Value**](.md)> | Display name of the author that modified the file on the origin last | [optional]
**_type** | **String** |  | 
**location** | **String** | Location identification for file in storage | 
**_links** | [**models::StorageFileModelAllOfLinks**](StorageFileModel_allOf__links.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


