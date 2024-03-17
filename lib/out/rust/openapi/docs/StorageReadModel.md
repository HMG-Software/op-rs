# StorageReadModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Storage id | 
**_type** | **String** |  | 
**name** | **String** | Storage name | 
**has_application_password** | Option<**bool**> | Whether the storage has the application password to use for the Nextcloud storage.  Ignored if the provider type is not Nextcloud | [optional]
**created_at** | Option<**String**> | Time of creation | [optional]
**updated_at** | Option<**String**> | Time of the most recent change to the storage | [optional]
**_embedded** | Option<[**models::StorageReadModelEmbedded**](StorageReadModel__embedded.md)> |  | [optional]
**_links** | [**models::StorageReadModelLinks**](StorageReadModel__links.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


