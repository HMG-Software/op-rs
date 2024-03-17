# \TimeEntriesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**available_projects_for_time_entries**](TimeEntriesApi.md#available_projects_for_time_entries) | **GET** /api/v3/time_entries/available_projects | Available projects for time entries
[**create_time_entry**](TimeEntriesApi.md#create_time_entry) | **POST** /api/v3/time_entries | Create time entry
[**delete_time_entry**](TimeEntriesApi.md#delete_time_entry) | **DELETE** /api/v3/time_entries/{id} | Delete time entry
[**get_time_entry**](TimeEntriesApi.md#get_time_entry) | **GET** /api/v3/time_entries/{id} | Get time entry
[**list_time_entries**](TimeEntriesApi.md#list_time_entries) | **GET** /api/v3/time_entries | List time entries
[**time_entry_create_form**](TimeEntriesApi.md#time_entry_create_form) | **POST** /api/v3/time_entries/form | Time entry create form
[**time_entry_update_form**](TimeEntriesApi.md#time_entry_update_form) | **POST** /api/v3/time_entries/{id}/form | Time entry update form
[**update_time_entry**](TimeEntriesApi.md#update_time_entry) | **PATCH** /api/v3/time_entries/{id} | update time entry
[**view_time_entry_schema**](TimeEntriesApi.md#view_time_entry_schema) | **GET** /api/v3/time_entries/schema | View time entry schema



## available_projects_for_time_entries

> serde_json::Value available_projects_for_time_entries()
Available projects for time entries

Gets a list of projects in which a time entry can be created in or be assigned to on update. The list contains all projects in which the user issuing the request has the necessary permissions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_time_entry

> models::TimeEntryModel create_time_entry()
Create time entry

Creates a new time entry applying the attributes provided in the body. Please note that while there is a fixed set of attributes, custom fields can extend a time entries' attributes and are accepted by the endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TimeEntryModel**](TimeEntryModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_time_entry

> delete_time_entry(id)
Delete time entry

Permanently deletes the specified time entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Time entry id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_entry

> models::TimeEntryModel get_time_entry(id)
Get time entry

Retrieves a single time entry identified by the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | time entry id | [required] |

### Return type

[**models::TimeEntryModel**](TimeEntryModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_time_entries

> models::TimeEntryCollectionModel list_time_entries(offset, page_size, sort_by, filters)
List time entries

Lists time entries. The time entries returned depend on the filters provided and also on the permission of the requesting user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Page number inside the requested collection. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page. |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported sorts are:  + id: Sort by primary key  + hours: Sort by logged hours  + spent_on: Sort by spent on date  + created_at: Sort by time entry creation datetime  + updated_at: Sort by the time the time entry was updated last |  |[default to ["spent_on", "asc"]]
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + work_package: Filter time entries by work package  + project: Filter time entries by project  + user: Filter time entries by users  + ongoing: Filter for your ongoing timers  + spent_on: Filter time entries by spent on date  + created_at: Filter time entries by creation datetime  + updated_at: Filter time entries by the last time they where updated  + activity: Filter time entries by time entry activity |  |

### Return type

[**models::TimeEntryCollectionModel**](TimeEntryCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_entry_create_form

> time_entry_create_form()
Time entry create form



### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_entry_update_form

> time_entry_update_form(id, body)
Time entry update form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Time entries activity id | [required] |
**body** | **i32** | Time entries activity id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_time_entry

> models::TimeEntryModel update_time_entry(id)
update time entry

Updates the given time entry by applying the attributes provided in the body. Please note that while there is a fixed set of attributes, custom fields can extend a time entries' attributes and are accepted by the endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Time entry id | [required] |

### Return type

[**models::TimeEntryModel**](TimeEntryModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_time_entry_schema

> serde_json::Value view_time_entry_schema()
View time entry schema



### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

