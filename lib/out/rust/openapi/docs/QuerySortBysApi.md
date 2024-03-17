# \QuerySortBysApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_query_sort_by**](QuerySortBysApi.md#view_query_sort_by) | **GET** /api/v3/queries/sort_bys/{id} | View Query Sort By



## view_query_sort_by

> models::QuerySortByModel view_query_sort_by(id)
View Query Sort By

Retrieve an individual QuerySortBy as identified by the id parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | QuerySortBy identifier. The identifier is a combination of the column identifier and the direction. | [required] |

### Return type

[**models::QuerySortByModel**](Query_Sort_ByModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

