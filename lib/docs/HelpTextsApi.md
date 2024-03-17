# \HelpTextsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_help_text**](HelpTextsApi.md#get_help_text) | **GET** /api/v3/help_texts/{id} | Get help text
[**list_help_texts**](HelpTextsApi.md#list_help_texts) | **GET** /api/v3/help_texts | List help texts



## get_help_text

> models::HelpTextModel get_help_text(id)
Get help text

Fetches the help text from the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Help text id | [required] |

### Return type

[**models::HelpTextModel**](HelpTextModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_help_texts

> models::HelpTextCollectionModel list_help_texts()
List help texts

List the complete collection of help texts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HelpTextCollectionModel**](HelpTextCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

