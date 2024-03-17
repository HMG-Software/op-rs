# \ProjectsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](ProjectsApi.md#create_project) | **POST** /api/v3/projects | Create project
[**create_project_copy**](ProjectsApi.md#create_project_copy) | **POST** /api/v3/projects/{id}/copy | Create project copy
[**delete_project**](ProjectsApi.md#delete_project) | **DELETE** /api/v3/projects/{id} | Delete Project
[**list_available_parent_project_candidates**](ProjectsApi.md#list_available_parent_project_candidates) | **GET** /api/v3/projects/available_parent_projects | List available parent project candidates
[**list_projects**](ProjectsApi.md#list_projects) | **GET** /api/v3/projects | List projects
[**list_projects_with_version**](ProjectsApi.md#list_projects_with_version) | **GET** /api/v3/versions/{id}/projects | List projects having version
[**project_copy_form**](ProjectsApi.md#project_copy_form) | **POST** /api/v3/projects/{id}/copy/form | Project copy form
[**project_create_form**](ProjectsApi.md#project_create_form) | **POST** /api/v3/projects/form | Project create form
[**project_update_form**](ProjectsApi.md#project_update_form) | **POST** /api/v3/projects/{id}/form | Project update form
[**update_project**](ProjectsApi.md#update_project) | **PATCH** /api/v3/projects/{id} | Update Project
[**view_project**](ProjectsApi.md#view_project) | **GET** /api/v3/projects/{id} | View project
[**view_project_schema**](ProjectsApi.md#view_project_schema) | **GET** /api/v3/projects/schema | View project schema
[**view_project_status**](ProjectsApi.md#view_project_status) | **GET** /api/v3/project_statuses/{id} | View project status



## create_project

> models::ProjectModel create_project(body)
Create project

Creates a new project, applying the attributes provided in the body.  You can use the form and schema to be retrieve the valid attribute values and by that be guided towards successful creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::ProjectModel**](ProjectModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_copy

> create_project_copy(id)
Create project copy



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


## delete_project

> delete_project(id)
Delete Project

Deletes the project permanently. As this is a lengthy process, the actual deletion is carried out asynchronously. So the project might exist well after the request has returned successfully. To prevent unwanted changes to the project scheduled for deletion, it is archived at once.

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


## list_available_parent_project_candidates

> serde_json::Value list_available_parent_project_candidates(filters, of, sort_by)
List available parent project candidates

Lists projects which can become parent to another project. Only sound candidates are returned. For instance a project cannot become parent of itself or it's children.  To specify the project for which a parent is queried for, the `of` parameter can be provided. If no `of` parameter is provided, a new project is assumed. Then, the check for the hierarchy is omitted as a new project cannot be part of a hierarchy yet.  Candidates can be filtered. Most commonly one will want to filter by name or identifier. You can do this through the `filters` parameter which works just like the work package index.  For instance to find all parent candidates with \"rollout\" in their name:  ``` ?filters=[{\"name_and_identifier\":{\"operator\":\"~\",\"values\":[\"rollout\"]}}] ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. |  |
**of** | Option<**String**> | The id or identifier of the project the parent candidate is determined for |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint and allows all the filters and sortBy supported by the project list end point. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_projects

> models::ProjectCollectionModel list_projects(filters, sort_by, select)
List projects

Returns a collection of projects. The collection can be filtered via query parameters similar to how work packages are filtered. In addition to the provided filter, the result set is always limited to only contain projects the client is allowed to see.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + active: based on the active property of the project + ancestor: filters projects by their ancestor. A project is not considered to be it's own ancestor. + created_at: based on the time the project was created + latest_activity_at: based on the time the last activity was registered on a project. + name_and_identifier: based on both the name and the identifier. + parent_id: filters projects by their parent. + principal: based on members of the project. + storage_id: filters projects by linked storages + storage_url: filters projects by linked storages identified by the host url + type_id: based on the types active in a project. + user_action: based on the actions the current user has in the project. + id: based on projects' id. + visible: based on the visibility for the user (id) provided as the filter value. This filter is useful for admins to identify the projects visible to a user.  There might also be additional filters based on the custom fields that have been configured. |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Currently supported orders are:  + id + name + typeahead (sorting by hierarchy and name) + created_at + public + latest_activity_at + required_disk_space  There might also be additional orders based on the custom fields that have been configured. |  |
**select** | Option<**String**> | Comma separated list of properties to include. |  |

### Return type

[**models::ProjectCollectionModel**](ProjectCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_projects_with_version

> serde_json::Value list_projects_with_version(id)
List projects having version

This endpoint lists the projects where the given version is available.  The projects returned depend on the sharing settings of the given version, but are also limited to the projects that the current user is allowed to see.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Version id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_copy_form

> project_copy_form(id)
Project copy form



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


## project_create_form

> serde_json::Value project_create_form(body)
Project create form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_update_form

> project_update_form(id, body)
Project update form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project id | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> models::ProjectModel update_project(id, body)
Update Project

Updates the given project by applying the attributes provided in the body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project id | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::ProjectModel**](ProjectModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_project

> models::ProjectModel view_project(id)
View project



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project id | [required] |

### Return type

[**models::ProjectModel**](ProjectModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_project_schema

> serde_json::Value view_project_schema()
View project schema



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


## view_project_status

> serde_json::Value view_project_status(id)
View project status



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Project status id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

