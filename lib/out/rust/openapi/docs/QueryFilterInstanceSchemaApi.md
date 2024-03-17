# \QueryFilterInstanceSchemaApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_query_filter_instance_schemas**](QueryFilterInstanceSchemaApi.md#list_query_filter_instance_schemas) | **GET** /api/v3/queries/filter_instance_schemas | List Query Filter Instance Schemas
[**list_query_filter_instance_schemas_for_project**](QueryFilterInstanceSchemaApi.md#list_query_filter_instance_schemas_for_project) | **GET** /api/v3/projects/{id}/queries/filter_instance_schemas | List Query Filter Instance Schemas for Project
[**view_query_filter_instance_schema**](QueryFilterInstanceSchemaApi.md#view_query_filter_instance_schema) | **GET** /api/v3/queries/filter_instance_schemas/{id} | View Query Filter Instance Schema



## list_query_filter_instance_schemas

> serde_json::Value list_query_filter_instance_schemas()
List Query Filter Instance Schemas

Returns the list of QueryFilterInstanceSchemas defined for a global query. That is a query not assigned to a project.

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


## list_query_filter_instance_schemas_for_project

> serde_json::Value list_query_filter_instance_schemas_for_project(id)
List Query Filter Instance Schemas for Project

Returns the list of QueryFilterInstanceSchemas defined for a query of the specified project.

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


## view_query_filter_instance_schema

> models::QueryFilterInstanceSchemaModel view_query_filter_instance_schema(id)
View Query Filter Instance Schema

Retrieve an individual QueryFilterInstanceSchema as identified by the id parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | QueryFilterInstanceSchema identifier. The identifier is the filter identifier. | [required] |

### Return type

[**models::QueryFilterInstanceSchemaModel**](Query_Filter_Instance_SchemaModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

