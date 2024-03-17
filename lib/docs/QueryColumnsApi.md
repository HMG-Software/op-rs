# \QueryColumnsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_query_column**](QueryColumnsApi.md#view_query_column) | **GET** /api/v3/queries/columns/{id} | View Query Column



## view_query_column

> models::QueryColumnModel view_query_column(id)
View Query Column

Retrieve an individual QueryColumn as identified by the `id` parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | QueryColumn id | [required] |

### Return type

[**models::QueryColumnModel**](Query_ColumnModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

