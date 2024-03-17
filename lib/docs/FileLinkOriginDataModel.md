# FileLinkOriginDataModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Linked file's id on the origin | 
**name** | **String** | Linked file's name on the origin | 
**mime_type** | Option<**String**> | MIME type of the linked file.  To link a folder entity, the custom MIME type `application/x-op-directory` MUST be provided. Otherwise it defaults back to an unknown MIME type. | [optional]
**size** | Option<**i32**> | file size on origin in bytes | [optional]
**created_at** | Option<**String**> | Timestamp of the creation datetime of the file on the origin | [optional]
**last_modified_at** | Option<**String**> | Timestamp of the datetime of the last modification of the file on the origin | [optional]
**created_by_name** | Option<**String**> | Display name of the author that created the file on the origin | [optional]
**last_modified_by_name** | Option<**String**> | Display name of the author that modified the file on the origin last | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


