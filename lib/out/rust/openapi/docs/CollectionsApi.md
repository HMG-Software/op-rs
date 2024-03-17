# \CollectionsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_aggregated_result**](CollectionsApi.md#view_aggregated_result) | **GET** /api/v3/examples | view aggregated result



## view_aggregated_result

> view_aggregated_result(group_by, show_sums)
view aggregated result



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_by** | Option<**String**> | The column to group by. Note: Aggregation is as of now only supported by the work package collection. You can pass any column name as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. |  |
**show_sums** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

