# \StatusesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_all_statuses**](StatusesApi.md#list_all_statuses) | **GET** /api/v3/statuses | List all Statuses
[**view_status**](StatusesApi.md#view_status) | **GET** /api/v3/statuses/{id} | View Status



## list_all_statuses

> models::StatusCollectionModel list_all_statuses()
List all Statuses



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StatusCollectionModel**](StatusCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_status

> models::StatusModel view_status(id)
View Status



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Status id | [required] |

### Return type

[**models::StatusModel**](StatusModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

