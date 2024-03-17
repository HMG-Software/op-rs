# StorageWriteModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Storage name, if not provided, falls back to a default. | [optional]
**application_password** | Option<**String**> | The application password to use for the Nextcloud storage. Ignored if the provider type is not Nextcloud.  If a string is provided, the password is set and automatic management is enabled for the storage. If null is provided, the password is unset and automatic management is disabled for the storage. | [optional]
**_links** | Option<[**models::StorageWriteModelLinks**](StorageWriteModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


