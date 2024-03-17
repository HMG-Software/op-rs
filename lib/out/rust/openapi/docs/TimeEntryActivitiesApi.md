# \TimeEntryActivitiesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_time_entries_activity**](TimeEntryActivitiesApi.md#get_time_entries_activity) | **GET** /api/v3/time_entries/activity/{id} | View time entries activity



## get_time_entries_activity

> models::TimeEntryActivityModel get_time_entries_activity(id)
View time entries activity

Fetches the time entry activity resource by the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Time entries activity id | [required] |

### Return type

[**models::TimeEntryActivityModel**](TimeEntryActivityModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

