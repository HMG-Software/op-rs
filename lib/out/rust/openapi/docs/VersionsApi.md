# \VersionsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**available_projects_for_versions**](VersionsApi.md#available_projects_for_versions) | **GET** /api/v3/versions/available_projects | Available projects for versions
[**create_version**](VersionsApi.md#create_version) | **POST** /api/v3/versions | Create version
[**delete_version**](VersionsApi.md#delete_version) | **DELETE** /api/v3/versions/{id} | Delete version
[**list_versions**](VersionsApi.md#list_versions) | **GET** /api/v3/versions | List versions
[**list_versions_available_in_a_project**](VersionsApi.md#list_versions_available_in_a_project) | **GET** /api/v3/projects/{id}/versions | List versions available in a project
[**update_version**](VersionsApi.md#update_version) | **PATCH** /api/v3/versions/{id} | Update Version
[**version_create_form**](VersionsApi.md#version_create_form) | **POST** /api/v3/versions/form | Version create form
[**version_update_form**](VersionsApi.md#version_update_form) | **POST** /api/v3/versions/{id}/form | Version update form
[**view_version**](VersionsApi.md#view_version) | **GET** /api/v3/versions/{id} | View version
[**view_version_schema**](VersionsApi.md#view_version_schema) | **GET** /api/v3/versions/schema | View version schema



## available_projects_for_versions

> serde_json::Value available_projects_for_versions()
Available projects for versions

Gets a list of projects in which a version can be created in. The list contains all projects in which the user issuing the request has the manage versions permissions.

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


## create_version

> models::VersionModel create_version()
Create version

Creates a new version applying the attributes provided in the body. Please note that while there is a fixed set of attributes, custom fields can extend a version's attributes and are accepted by the endpoint.  You can use the form and schema to be retrieve the valid attribute values and by that be guided towards successful creation.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::VersionModel**](VersionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_version

> delete_version(id)
Delete version

Deletes the version. Work packages associated to the version will no longer be assigned to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Version id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_versions

> serde_json::Value list_versions(filters)
List versions

Returns a collection of versions. The client can choose to filter the versions similar to how work packages are filtered. In addition to the provided filters, the server will reduce the result set to only contain versions, for which the requesting client has sufficient permissions (*view_work_packages*).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + sharing: filters versions by how they are shared within the server (*none*, *descendants*, *hierarchy*, *tree*, *system*). |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_versions_available_in_a_project

> serde_json::Value list_versions_available_in_a_project(id)
List versions available in a project

This endpoint lists the versions that are *available* in a given project. Note that due to sharing this might be more than the versions *defined* by that project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the project whose versions will be listed | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_version

> models::VersionModel update_version(id)
Update Version

Updates the given version by applying the attributes provided in the body. Please note that while there is a fixed set of attributes, custom fields can extend a version's attributes and are accepted by the endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Version id | [required] |

### Return type

[**models::VersionModel**](VersionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## version_create_form

> version_create_form()
Version create form



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


## version_update_form

> version_update_form(id)
Version update form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_version

> models::VersionModel view_version(id)
View version



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Version id | [required] |

### Return type

[**models::VersionModel**](VersionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_version_schema

> serde_json::Value view_version_schema()
View version schema



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

