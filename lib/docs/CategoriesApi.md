# \CategoriesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_categories_of_a_project**](CategoriesApi.md#list_categories_of_a_project) | **GET** /api/v3/projects/{id}/categories | List categories of a project
[**view_category**](CategoriesApi.md#view_category) | **GET** /api/v3/categories/{id} | View Category



## list_categories_of_a_project

> serde_json::Value list_categories_of_a_project(id)
List categories of a project



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the project whose categories will be listed | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_category

> models::CategoryModel view_category(id)
View Category



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Category id | [required] |

### Return type

[**models::CategoryModel**](CategoryModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

