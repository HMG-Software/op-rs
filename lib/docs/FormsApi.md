# \FormsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**show_or_validate_form**](FormsApi.md#show_or_validate_form) | **POST** /api/v3/example/form | show or validate form



## show_or_validate_form

> serde_json::Value show_or_validate_form(show_or_validate_form_request)
show or validate form

This is an example of how a form might look like. Note that this endpoint does not exist in the actual implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**show_or_validate_form_request** | Option<[**ShowOrValidateFormRequest**](ShowOrValidateFormRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

