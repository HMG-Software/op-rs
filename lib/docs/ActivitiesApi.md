# \ActivitiesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_activity**](ActivitiesApi.md#update_activity) | **PATCH** /api/v3/activities/{id} | Update activity
[**view_activity**](ActivitiesApi.md#view_activity) | **GET** /api/v3/activities/{id} | View activity



## update_activity

> models::ActivityModel update_activity(id, update_activity_request)
Update activity

Updates an activity's comment and, on success, returns the updated activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Activity id | [required] |
**update_activity_request** | Option<[**UpdateActivityRequest**](UpdateActivityRequest.md)> |  |  |

### Return type

[**models::ActivityModel**](ActivityModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_activity

> models::ActivityModel view_activity(id)
View activity



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Activity id | [required] |

### Return type

[**models::ActivityModel**](ActivityModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

