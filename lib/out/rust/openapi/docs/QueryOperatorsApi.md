# \QueryOperatorsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_query_operator**](QueryOperatorsApi.md#view_query_operator) | **GET** /api/v3/queries/operators/{id} | View Query Operator



## view_query_operator

> models::QueryOperatorModel view_query_operator(id)
View Query Operator

Retrieve an individual QueryOperator as identified by the `id` parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | QueryOperator id | [required] |

### Return type

[**models::QueryOperatorModel**](Query_OperatorModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

