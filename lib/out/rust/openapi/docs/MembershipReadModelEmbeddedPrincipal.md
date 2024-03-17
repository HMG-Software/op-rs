# MembershipReadModelEmbeddedPrincipal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | 
**id** | Option<[**serde_json::Value**](.md)> | The principal's unique identifier. | 
**name** | Option<[**serde_json::Value**](.md)> | The principal's display name, layout depends on instance settings. | 
**created_at** | Option<[**serde_json::Value**](.md)> | Time of creation | [optional]
**updated_at** | Option<[**serde_json::Value**](.md)> | Time of the most recent change to the user | [optional]
**_links** | [**models::UserModelAllOfLinks**](UserModel_allOf__links.md) |  | 
**avatar** | Option<[**serde_json::Value**](.md)> | URL to user's avatar | 
**login** | Option<[**serde_json::Value**](.md)> | The user's login name  # Conditions  - User is self, or `create_user` or `manage_user` permission globally | [optional]
**first_name** | Option<[**serde_json::Value**](.md)> | The user's first name  # Conditions  - User is self, or `create_user` or `manage_user` permission globally | [optional]
**last_name** | Option<[**serde_json::Value**](.md)> | The user's last name  # Conditions  - User is self, or `create_user` or `manage_user` permission globally | [optional]
**email** | Option<[**serde_json::Value**](.md)> | The user's email address  # Conditions  - E-Mail address not hidden - User is not a new record - User is self, or `create_user` or `manage_user` permission globally | [optional]
**admin** | Option<[**serde_json::Value**](.md)> | Flag indicating whether or not the user is an admin  # Conditions  - `admin` | [optional]
**status** | Option<[**serde_json::Value**](.md)> | The current activation status of the user.  # Conditions  - User is self, or `create_user` or `manage_user` permission globally | [optional]
**language** | Option<[**serde_json::Value**](.md)> | User's language | ISO 639-1 format  # Conditions  - User is self, or `create_user` or `manage_user` permission globally | [optional]
**identity_url** | Option<[**serde_json::Value**](.md)> | User's identity_url for OmniAuth authentication.  # Conditions  - User is self, or `create_user` or `manage_user` permission globally | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


