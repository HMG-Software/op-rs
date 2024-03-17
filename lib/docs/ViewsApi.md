# \ViewsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_views**](ViewsApi.md#create_views) | **POST** /api/v3/views/{id} | Create view
[**list_views**](ViewsApi.md#list_views) | **GET** /api/v3/views | List views
[**view_view**](ViewsApi.md#view_view) | **GET** /api/v3/views/{id} | View view



## create_views

> serde_json::Value create_views(id, create_views_request)
Create view

When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body. The required fields of a View can be found in its schema, which is embedded in the respective form. Note that it is only allowed to provide properties or links supporting the write operation.  There are different subtypes of `Views` (e.g. `Views::WorkPackagesTable`) with each having its own endpoint for creating that subtype e.g.  * `/api/v3/views/work_packages_table` for `Views::WorkPackagesTable` * `/api/v3/views/team_planner` for `Views::TeamPlanner` * `/api/v3/views/work_packages_calendar` for `Views::WorkPackagesCalendar`  **Not yet implemented** To get the list of available subtypes and by that the endpoints for creating a subtype, use the ```   /api/v3/views/schemas ``` endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The view identifier | [required] |
**create_views_request** | Option<[**CreateViewsRequest**](CreateViewsRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_views

> list_views(filters)
List views

Returns a collection of Views. The collection can be filtered via query parameters similar to how work packages are filtered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Currently supported filters are:  + project: filters views by the project their associated query is assigned to. If the project filter is passed with the `!*` (not any) operator, global views are returned.  + id: filters views based on their id  + type: filters views based on their type |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_view

> view_view(id)
View view



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | View id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

