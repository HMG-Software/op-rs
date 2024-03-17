# \ValuesPropertyApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_notification_detail**](ValuesPropertyApi.md#view_notification_detail) | **GET** /api/v3/notifications/{notification_id}/details/{id} | Get a notification detail
[**view_values_schema**](ValuesPropertyApi.md#view_values_schema) | **GET** /api/v3/values/schema/{id} | View Values schema



## view_notification_detail

> models::ValuesPropertyModel view_notification_detail(notification_id, id)
Get a notification detail

Returns an individual detail of a notification identified by the notification id and the id of the detail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **i32** | notification id | [required] |
**id** | **i32** | detail id | [required] |

### Return type

[**models::ValuesPropertyModel**](ValuesPropertyModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_values_schema

> models::SchemaModel view_values_schema(id)
View Values schema

The schema of a `Values` resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The identifier of the value. This is typically the value of the `property` property of the `Values` resource. It should be in lower camelcase format. | [required] |

### Return type

[**models::SchemaModel**](SchemaModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

