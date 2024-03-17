# \QueryFiltersApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_query_filter**](QueryFiltersApi.md#view_query_filter) | **GET** /api/v3/queries/filters/{id} | View Query Filter



## view_query_filter

> models::QueryFilterModel view_query_filter(id)
View Query Filter

Retrieve an individual QueryFilter as identified by the id parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | QueryFilter identifier | [required] |

### Return type

[**models::QueryFilterModel**](Query_FilterModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

