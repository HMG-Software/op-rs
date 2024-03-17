# \UserPreferencesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**show_my_preferences**](UserPreferencesApi.md#show_my_preferences) | **GET** /api/v3/my_preferences | Show my preferences
[**update_user_preferences**](UserPreferencesApi.md#update_user_preferences) | **PATCH** /api/v3/my_preferences | Update my preferences



## show_my_preferences

> serde_json::Value show_my_preferences()
Show my preferences



### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_preferences

> serde_json::Value update_user_preferences(update_user_preferences_request)
Update my preferences

When calling this endpoint the client provides a single object, containing the properties that it wants to change, in the body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_preferences_request** | Option<[**UpdateUserPreferencesRequest**](UpdateUserPreferencesRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

