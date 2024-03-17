# \CustomOptionsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_custom_option**](CustomOptionsApi.md#view_custom_option) | **GET** /api/v3/custom_options/{id} | View Custom Option



## view_custom_option

> models::CustomOptionModel view_custom_option(id)
View Custom Option



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The custom option's identifier | [required] |

### Return type

[**models::CustomOptionModel**](CustomOptionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

