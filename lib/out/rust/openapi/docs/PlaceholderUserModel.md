# PlaceholderUserModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** |  | 
**id** | Option<[**serde_json::Value**](.md)> | The principal's unique identifier. | 
**name** | Option<[**serde_json::Value**](.md)> | The principal's display name, layout depends on instance settings. | 
**created_at** | Option<[**serde_json::Value**](.md)> | Time of creation | [optional]
**updated_at** | Option<[**serde_json::Value**](.md)> | Time of the most recent change to the principal | [optional]
**_links** | [**models::PlaceholderUserModelAllOfLinks**](PlaceholderUserModel_allOf__links.md) |  | 
**status** | Option<**String**> | The current activation status of the placeholder user.  # Conditions  - User has `manage_placeholder_user` permission globally | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


