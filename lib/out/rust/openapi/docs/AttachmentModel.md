# AttachmentModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Attachment's id | [optional]
**title** | **String** | The name of the file | 
**file_name** | **String** | The name of the uploaded file | 
**file_size** | Option<**i32**> | The size of the uploaded file in Bytes | [optional]
**description** | [**models::AttachmentModelDescription**](AttachmentModel_description.md) |  | 
**content_type** | **String** | The files MIME-Type as determined by the server | 
**digest** | **String** | A checksum for the files content | 
**created_at** | **String** | Time of creation | 
**_links** | Option<[**models::AttachmentModelLinks**](AttachmentModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


