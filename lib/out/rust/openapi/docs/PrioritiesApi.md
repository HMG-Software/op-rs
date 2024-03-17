# \PrioritiesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_all_priorities**](PrioritiesApi.md#list_all_priorities) | **GET** /api/v3/priorities | List all Priorities
[**view_priority**](PrioritiesApi.md#view_priority) | **GET** /api/v3/priorities/{id} | View Priority



## list_all_priorities

> serde_json::Value list_all_priorities()
List all Priorities



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


## view_priority

> models::PriorityModel view_priority(id)
View Priority



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Priority id | [required] |

### Return type

[**models::PriorityModel**](PriorityModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

