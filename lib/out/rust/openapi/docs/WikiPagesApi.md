# \WikiPagesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_wiki_page**](WikiPagesApi.md#view_wiki_page) | **GET** /api/v3/wiki_pages/{id} | View Wiki Page



## view_wiki_page

> models::WikiPageModel view_wiki_page(id)
View Wiki Page

Retrieve an individual wiki page as identified by the id parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Wiki page identifier | [required] |

### Return type

[**models::WikiPageModel**](Wiki_PageModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

