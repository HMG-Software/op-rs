# \PostsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_post**](PostsApi.md#view_post) | **GET** /api/v3/posts/{id} | View Post



## view_post

> models::PostModel view_post(id)
View Post

Retrieve an individual post as identified by the id parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Post's identifier | [required] |

### Return type

[**models::PostModel**](PostModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

