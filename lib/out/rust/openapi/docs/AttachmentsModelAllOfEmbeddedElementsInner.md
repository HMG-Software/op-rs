# AttachmentsModelAllOfEmbeddedElementsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | Attachment's id | [optional]
**title** | Option<[**serde_json::Value**](.md)> | The name of the file | 
**file_name** | Option<[**serde_json::Value**](.md)> | The name of the uploaded file | 
**file_size** | Option<[**serde_json::Value**](.md)> | The size of the uploaded file in Bytes | [optional]
**description** | [**models::AttachmentModelDescription**](AttachmentModel_description.md) |  | 
**content_type** | Option<[**serde_json::Value**](.md)> | The files MIME-Type as determined by the server | 
**digest** | Option<[**serde_json::Value**](.md)> | A checksum for the files content | 
**created_at** | Option<[**serde_json::Value**](.md)> | Time of creation | 
**_links** | Option<[**models::AttachmentModelLinks**](AttachmentModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


