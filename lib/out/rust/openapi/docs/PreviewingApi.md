# \PreviewingApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**preview_markdown_document**](PreviewingApi.md#preview_markdown_document) | **POST** /api/v3/render/markdown | Preview Markdown document
[**preview_plain_document**](PreviewingApi.md#preview_plain_document) | **POST** /api/v3/render/plain | Preview plain document



## preview_markdown_document

> String preview_markdown_document(context)
Preview Markdown document



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | Option<**String**> | API-Link to the context in which the rendering occurs, for example a specific work package.  If left out only context-agnostic rendering takes place. Please note that OpenProject features markdown-extensions on top of the extensions GitHub Flavored Markdown (gfm) already provides that can only work given a context (e.g. display attached images).  **Supported contexts:**  * `/api/v3/work_packages/{id}` - an existing work package |  |

### Return type

**String**

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## preview_plain_document

> String preview_plain_document()
Preview plain document



### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

