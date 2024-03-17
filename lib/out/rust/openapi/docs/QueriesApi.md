# \QueriesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**available_projects_for_query**](QueriesApi.md#available_projects_for_query) | **GET** /api/v3/queries/available_projects | Available projects for query
[**create_query**](QueriesApi.md#create_query) | **POST** /api/v3/queries | Create query
[**delete_query**](QueriesApi.md#delete_query) | **DELETE** /api/v3/queries/{id} | Delete query
[**edit_query**](QueriesApi.md#edit_query) | **PATCH** /api/v3/queries/{id} | Edit Query
[**list_queries**](QueriesApi.md#list_queries) | **GET** /api/v3/queries | List queries
[**query_create_form**](QueriesApi.md#query_create_form) | **POST** /api/v3/queries/form | Query Create Form
[**query_update_form**](QueriesApi.md#query_update_form) | **POST** /api/v3/queries/{id}/form | Query Update Form
[**star_query**](QueriesApi.md#star_query) | **PATCH** /api/v3/queries/{id}/star | Star query
[**unstar_query**](QueriesApi.md#unstar_query) | **PATCH** /api/v3/queries/{id}/unstar | Unstar query
[**view_default_query**](QueriesApi.md#view_default_query) | **GET** /api/v3/queries/default | View default query
[**view_default_query_for_project**](QueriesApi.md#view_default_query_for_project) | **GET** /api/v3/projects/{id}/queries/default | View default query for project
[**view_query**](QueriesApi.md#view_query) | **GET** /api/v3/queries/{id} | View query
[**view_schema_for_global_queries**](QueriesApi.md#view_schema_for_global_queries) | **GET** /api/v3/queries/schema | View schema for global queries
[**view_schema_for_project_queries**](QueriesApi.md#view_schema_for_project_queries) | **GET** /api/v3/projects/{id}/queries/schema | View schema for project queries



## available_projects_for_query

> serde_json::Value available_projects_for_query()
Available projects for query

Gets a list of projects that are available as projects a query can be assigned to.

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


## create_query

> models::QueryModel create_query(query_create_form)
Create query

When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body. The required fields of a Query can be found in its schema, which is embedded in the respective form. Note that it is only allowed to provide properties or links supporting the write operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_create_form** | Option<[**QueryCreateForm**](QueryCreateForm.md)> |  |  |

### Return type

[**models::QueryModel**](QueryModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_query

> delete_query(id)
Delete query

Delete the query identified by the id parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Query id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_query

> models::QueryModel edit_query(id, query_update_form)
Edit Query

When calling this endpoint the client provides a single object, containing the properties and links that it wants to change, in the body. Note that it is only allowed to provide properties or links supporting the **write** operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Query id | [required] |
**query_update_form** | Option<[**QueryUpdateForm**](QueryUpdateForm.md)> |  |  |

### Return type

[**models::QueryModel**](QueryModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_queries

> serde_json::Value list_queries(filters)
List queries

Returns a collection of queries. The collection can be filtered via query parameters similar to how work packages are filtered. Please note however, that the filters are applied to the queries and not to the work packages the queries in turn might return.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Currently supported filters are:  + project: filters queries by the project they are assigned to. If the project filter is passed with the `!*` (not any) operator, global queries are returned.  + id: filters queries based on their id  + updated_at: filters queries based on the last time they where updated |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_create_form

> query_create_form(query_create_form)
Query Create Form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_create_form** | Option<[**QueryCreateForm**](QueryCreateForm.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_update_form

> query_update_form(id, query_update_form)
Query Update Form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Query id | [required] |
**query_update_form** | Option<[**QueryUpdateForm**](QueryUpdateForm.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## star_query

> serde_json::Value star_query(id)
Star query



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Query id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unstar_query

> serde_json::Value unstar_query(id)
Unstar query



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Query id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_default_query

> serde_json::Value view_default_query(filters, offset, page_size, sort_by, group_by, show_sums, timestamps, timeline_visible, timeline_zoom_level, show_hierarchies)
View default query

Same as [viewing an existing, persisted Query](https://www.openproject.org/docs/api/endpoints/queries/#list-queries) in its response, this resource returns an unpersisted query and by that allows to get the default query configuration. The client may also provide additional parameters which will modify the default query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. The filters provided as parameters are not applied to the query but are instead used to override the query's persisted filters. All filters also accepted by the work packages endpoint are accepted. If no filter is to be applied, the client should send an empty array (`[]`). |  |[default to [{ "status_id": { "operator": "o", "values": null }}]]
**offset** | Option<**i32**> | Page number inside the queries' result collection of work packages. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page for the queries' result collection of work packages. |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. The sort criteria is applied to the query's result collection of work packages overriding the query's persisted sort criteria. |  |[default to [["id", "asc"]]]
**group_by** | Option<**String**> | The column to group by. The grouping criteria is applied to the to the query's result collection of work packages overriding the query's persisted group criteria. |  |
**show_sums** | Option<**bool**> | Indicates whether properties should be summed up if they support it. The showSums parameter is applied to the to the query's result collection of work packages overriding the query's persisted sums property. |  |[default to false]
**timestamps** | Option<**String**> | Indicates the timestamps to filter by when showing changed attributes on work packages. Values can be either ISO8601 dates, ISO8601 durations and the following relative date keywords: \"oneDayAgo@HH:MM+HH:MM\", \"lastWorkingDay@HH:MM+HH:MM\", \"oneWeekAgo@HH:MM+HH:MM\", \"oneMonthAgo@HH:MM+HH:MM\". The first \"HH:MM\" part represents the zero paded hours and minutes. The last \"+HH:MM\" part represents the timezone offset from UTC associated with the time, the offset can be positive or negative e.g.\"oneDayAgo@01:00+01:00\", \"oneDayAgo@01:00-01:00\". Values older than 1 day are accepted only with valid Enterprise Token available.  |  |[default to PT0S]
**timeline_visible** | Option<**bool**> | Indicates whether the timeline should be shown. |  |[default to false]
**timeline_zoom_level** | Option<**String**> | Indicates in what zoom level the timeline should be shown. Valid values are  `days`, `weeks`, `months`, `quarters`, and `years`. |  |[default to days]
**show_hierarchies** | Option<**bool**> | Indicates whether the hierarchy mode should be enabled. |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_default_query_for_project

> serde_json::Value view_default_query_for_project(id, filters, offset, page_size, sort_by, group_by, show_sums, timestamps, timeline_visible, show_hierarchies)
View default query for project

Same as [viewing an existing, persisted Query](https://www.openproject.org/docs/api/endpoints/queries/#list-queries) in its response, this resource returns an unpersisted query and by that allows to get the default query configuration. The client may also provide additional parameters which will modify the default query. The query will already be scoped for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Id of the project the default query is requested for | [required] |
**filters** | Option<**String**> | JSON specifying filter conditions. The filters provided as parameters are not applied to the query but are instead used to override the query's persisted filters. All filters also accepted by the work packages endpoint are accepted. If no filter is to be applied, the client should send an empty array (`[]`). |  |[default to [{ "status_id": { "operator": "o", "values": null }}]]
**offset** | Option<**i32**> | Page number inside the queries' result collection of work packages. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page for the queries' result collection of work packages. |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. The sort criteria is applied to the query's result collection of work packages overriding the query's persisted sort criteria. |  |[default to [["id", "asc"]]]
**group_by** | Option<**String**> | The column to group by. The grouping criteria is applied to the to the query's result collection of work packages overriding the query's persisted group criteria. |  |
**show_sums** | Option<**bool**> | Indicates whether properties should be summed up if they support it. The showSums parameter is applied to the to the query's result collection of work packages overriding the query's persisted sums property. |  |[default to false]
**timestamps** | Option<**String**> | Indicates the timestamps to filter by when showing changed attributes on work packages. Values can be either ISO8601 dates, ISO8601 durations and the following relative date keywords: \"oneDayAgo@HH:MM+HH:MM\", \"lastWorkingDay@HH:MM+HH:MM\", \"oneWeekAgo@HH:MM+HH:MM\", \"oneMonthAgo@HH:MM+HH:MM\". The first \"HH:MM\" part represents the zero paded hours and minutes. The last \"+HH:MM\" part represents the timezone offset from UTC associated with the time. Values older than 1 day are accepted only with valid Enterprise Token available.  |  |[default to PT0S]
**timeline_visible** | Option<**bool**> | Indicates whether the timeline should be shown. |  |[default to false]
**show_hierarchies** | Option<**bool**> | Indicates whether the hierarchy mode should be enabled. |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_query

> models::QueryModel view_query(id, filters, offset, page_size, columns, sort_by, group_by, show_sums, timestamps, timeline_visible, timeline_labels, highlighting_mode, highlighted_attributes, show_hierarchies)
View query

Retrieve an individual query as identified by the id parameter. Then end point accepts a number of parameters that can be used to override the resources' persisted parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Query id | [required] |
**filters** | Option<**String**> | JSON specifying filter conditions. The filters provided as parameters are not applied to the query but are instead used to override the query's persisted filters. All filters also accepted by the work packages endpoint are accepted. If no filter is to be applied, the client should send an empty array (`[]`). |  |[default to [{ "status_id": { "operator": "o", "values": null }}]]
**offset** | Option<**i32**> | Page number inside the queries' result collection of work packages. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page for the queries' result collection of work packages. |  |
**columns** | Option<**String**> | Selected columns for the table view. |  |[default to ['type', 'priority']]
**sort_by** | Option<**String**> | JSON specifying sort criteria. The sort criteria is applied to the query's result collection of work packages overriding the query's persisted sort criteria. |  |[default to [["id", "asc"]]]
**group_by** | Option<**String**> | The column to group by. The grouping criteria is applied to the to the query's result collection of work packages overriding the query's persisted group criteria. |  |
**show_sums** | Option<**bool**> | Indicates whether properties should be summed up if they support it. The showSums parameter is applied to the to the query's result collection of work packages overriding the query's persisted sums property. |  |[default to false]
**timestamps** | Option<**String**> | Indicates the timestamps to filter by when showing changed attributes on work packages. Values can be either ISO8601 dates, ISO8601 durations and the following relative date keywords: \"oneDayAgo@HH:MM+HH:MM\", \"lastWorkingDay@HH:MM+HH:MM\", \"oneWeekAgo@HH:MM+HH:MM\", \"oneMonthAgo@HH:MM+HH:MM\". The first \"HH:MM\" part represents the zero paded hours and minutes. The last \"+HH:MM\" part represents the timezone offset from UTC associated with the time, the offset can be positive or negative e.g.\"oneDayAgo@01:00+01:00\", \"oneDayAgo@01:00-01:00\". Values older than 1 day are accepted only with valid Enterprise Token available.  |  |[default to PT0S]
**timeline_visible** | Option<**bool**> | Indicates whether the timeline should be shown. |  |[default to false]
**timeline_labels** | Option<**String**> | Overridden labels in the timeline view |  |[default to {}]
**highlighting_mode** | Option<**String**> | Highlighting mode for the table view. |  |[default to inline]
**highlighted_attributes** | Option<**String**> | Highlighted attributes mode for the table view when `highlightingMode` is `inline`. When set to `[]` all highlightable attributes will be returned as `highlightedAttributes`. |  |[default to ['type', 'priority']]
**show_hierarchies** | Option<**bool**> | Indicates whether the hierarchy mode should be enabled. |  |[default to true]

### Return type

[**models::QueryModel**](QueryModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_schema_for_global_queries

> serde_json::Value view_schema_for_global_queries()
View schema for global queries

Retrieve the schema for global queries, those, that are not assigned to a project.

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


## view_schema_for_project_queries

> serde_json::Value view_schema_for_project_queries(id)
View schema for project queries

Retrieve the schema for project queries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

