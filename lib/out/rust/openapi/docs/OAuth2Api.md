# \OAuth2Api

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_oauth_application**](OAuth2Api.md#get_oauth_application) | **GET** /api/v3/oauth_applications/{id} | Get the oauth application.
[**get_oauth_client_credentials**](OAuth2Api.md#get_oauth_client_credentials) | **GET** /api/v3/oauth_client_credentials/{id} | Get the oauth client credentials object.



## get_oauth_application

> models::OAuthApplicationReadModel get_oauth_application(id)
Get the oauth application.

Retrieves the OAuth 2 provider application for the given identifier. The secret will not be part of the response, instead a `confidential` flag is indicating, whether there is a secret or not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | OAuth application id | [required] |

### Return type

[**models::OAuthApplicationReadModel**](OAuthApplicationReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_client_credentials

> models::OAuthClientCredentialsReadModel get_oauth_client_credentials(id)
Get the oauth client credentials object.

Retrieves the OAuth 2 client credentials for the given identifier. The secret will not be part of the response, instead a `confidential` flag is indicating, whether there is a secret or not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | OAuth Client Credentials id | [required] |

### Return type

[**models::OAuthClientCredentialsReadModel**](OAuthClientCredentialsReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

