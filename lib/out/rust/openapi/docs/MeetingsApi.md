# \MeetingsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_meeting**](MeetingsApi.md#view_meeting) | **GET** /api/v3/meetings/{id} | View Meeting Page



## view_meeting

> models::MeetingModel view_meeting(id)
View Meeting Page

Retrieve an individual meeting as identified by the id parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Meeting identifier | [required] |

### Return type

[**models::MeetingModel**](MeetingModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

