# OAuthApplicationReadModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**_type** | **String** |  | 
**name** | **String** | The name of the OAuth 2 application | 
**client_id** | **String** | OAuth 2 client id | 
**client_secret** | Option<**String**> | OAuth 2 client secret. This is only returned when creating a new OAuth application. | [optional]
**confidential** | **bool** | true, if OAuth 2 credentials are confidential, false, if no secret is stored | 
**created_at** | Option<**String**> | The time the OAuth 2 Application was created at | [optional]
**updated_at** | Option<**String**> | The time the OAuth 2 Application was last updated | [optional]
**scopes** | Option<**Vec<String>**> | An array of the scopes of the OAuth 2 Application | [optional]
**_links** | Option<[**models::OAuthApplicationReadModelLinks**](OAuthApplicationReadModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


