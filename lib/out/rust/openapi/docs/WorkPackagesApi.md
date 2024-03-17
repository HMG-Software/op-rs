# \WorkPackagesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_watcher**](WorkPackagesApi.md#add_watcher) | **POST** /api/v3/work_packages/{id}/watchers | Add watcher
[**available_projects_for_work_package**](WorkPackagesApi.md#available_projects_for_work_package) | **GET** /api/v3/work_packages/{id}/available_projects | Available projects for work package
[**available_responsibles**](WorkPackagesApi.md#available_responsibles) | **GET** /api/v3/projects/{id}/available_responsibles | Available responsibles
[**available_watchers**](WorkPackagesApi.md#available_watchers) | **GET** /api/v3/work_packages/{id}/available_watchers | Available watchers
[**comment_work_package**](WorkPackagesApi.md#comment_work_package) | **POST** /api/v3/work_packages/{id}/activities | Comment work package
[**create_project_work_package**](WorkPackagesApi.md#create_project_work_package) | **POST** /api/v3/projects/{id}/work_packages | Create work package in project
[**create_relation**](WorkPackagesApi.md#create_relation) | **POST** /api/v3/work_packages/{id}/relations | Create Relation
[**create_work_package**](WorkPackagesApi.md#create_work_package) | **POST** /api/v3/work_packages | Create Work Package
[**create_work_package_file_link**](WorkPackagesApi.md#create_work_package_file_link) | **POST** /api/v3/work_packages/{id}/file_links | Creates file links.
[**delete_work_package**](WorkPackagesApi.md#delete_work_package) | **DELETE** /api/v3/work_packages/{id} | Delete Work Package
[**get_project_work_package_collection**](WorkPackagesApi.md#get_project_work_package_collection) | **GET** /api/v3/projects/{id}/work_packages | Get work packages of project
[**list_available_relation_candidates**](WorkPackagesApi.md#list_available_relation_candidates) | **GET** /api/v3/work_packages/{id}/available_relation_candidates | Available relation candidates
[**list_relations**](WorkPackagesApi.md#list_relations) | **GET** /api/v3/work_packages/{id}/relations | List relations
[**list_watchers**](WorkPackagesApi.md#list_watchers) | **GET** /api/v3/work_packages/{id}/watchers | List watchers
[**list_work_package_activities**](WorkPackagesApi.md#list_work_package_activities) | **GET** /api/v3/work_packages/{id}/activities | List work package activities
[**list_work_package_file_links**](WorkPackagesApi.md#list_work_package_file_links) | **GET** /api/v3/work_packages/{id}/file_links | Gets all file links of a work package
[**list_work_package_schemas**](WorkPackagesApi.md#list_work_package_schemas) | **GET** /api/v3/work_packages/schemas | List Work Package Schemas
[**list_work_packages**](WorkPackagesApi.md#list_work_packages) | **GET** /api/v3/work_packages | List work packages
[**project_available_assignees**](WorkPackagesApi.md#project_available_assignees) | **GET** /api/v3/projects/{id}/available_assignees | Project Available assignees
[**remove_watcher**](WorkPackagesApi.md#remove_watcher) | **DELETE** /api/v3/work_packages/{id}/watchers/{user_id} | Remove watcher
[**revisions**](WorkPackagesApi.md#revisions) | **GET** /api/v3/work_packages/{id}/revisions | Revisions
[**update_work_package**](WorkPackagesApi.md#update_work_package) | **PATCH** /api/v3/work_packages/{id} | Update a Work Package
[**view_work_package**](WorkPackagesApi.md#view_work_package) | **GET** /api/v3/work_packages/{id} | View Work Package
[**view_work_package_schema**](WorkPackagesApi.md#view_work_package_schema) | **GET** /api/v3/work_packages/schemas/{identifier} | View Work Package Schema
[**work_package_available_assignees**](WorkPackagesApi.md#work_package_available_assignees) | **GET** /api/v3/work_packages/{id}/available_assignees | Work Package Available assignees
[**work_package_create_form**](WorkPackagesApi.md#work_package_create_form) | **POST** /api/v3/work_packages/form | Work Package Create Form
[**work_package_create_form_for_project**](WorkPackagesApi.md#work_package_create_form_for_project) | **POST** /api/v3/projects/{id}/work_packages/form | Work Package Create Form For Project
[**work_package_edit_form**](WorkPackagesApi.md#work_package_edit_form) | **POST** /api/v3/work_packages/{id}/form | Work Package Edit Form



## add_watcher

> add_watcher(id, add_watcher_request)
Add watcher

Adds a watcher to the specified work package.  The request is expected to contain a single JSON object, that contains a link object under the `user` key.  The response will be user added as watcher. In case the user was already watching the work package an `HTTP 200` is returned, an `HTTP 201` if the user was added as a new watcher.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**add_watcher_request** | Option<[**AddWatcherRequest**](AddWatcherRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## available_projects_for_work_package

> serde_json::Value available_projects_for_work_package(id)
Available projects for work package

Gets a list of projects that are available as projects to which the work package can be moved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | work package id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## available_responsibles

> serde_json::Value available_responsibles(id)
Available responsibles

Gets a list of users that can be assigned as the responsible of a work package in the given project.

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


## available_watchers

> serde_json::Value available_watchers(id)
Available watchers

Gets a list of users that are able to be watchers of the specified work package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | work package id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comment_work_package

> comment_work_package(id, notify, comment_work_package_request)
Comment work package

Creates an activity for the selected work package and, on success, returns the updated activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**notify** | Option<**bool**> | Indicates whether change notifications (e.g. via E-Mail) should be sent. Note that this controls notifications for all users interested in changes to the work package (e.g. watchers, author and assignee), not just the current user. |  |[default to true]
**comment_work_package_request** | Option<[**CommentWorkPackageRequest**](CommentWorkPackageRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_work_package

> models::WorkPackageModel create_project_work_package(id, notify, work_package_model)
Create work package in project

When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body. The required fields of a WorkPackage can be found in its schema, which is embedded in the respective form. Note that it is only allowed to provide properties or links supporting the write operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project id | [required] |
**notify** | Option<**bool**> | Indicates whether change notifications (e.g. via E-Mail) should be sent. Note that this controls notifications for all users interested in changes to the work package (e.g. watchers, author and assignee), not just the current user. |  |[default to true]
**work_package_model** | Option<[**WorkPackageModel**](WorkPackageModel.md)> |  |  |

### Return type

[**models::WorkPackageModel**](Work_PackageModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_relation

> create_relation(id)
Create Relation

When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body. The required fields of a Relation can be found in its schema, which is embedded in the respective form. Note that it is only allowed to provide properties or links supporting the write operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_work_package

> models::WorkPackageModel create_work_package(notify, work_package_model)
Create Work Package

When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body. The required fields of a WorkPackage can be found in its schema, which is embedded in the respective form. Note that it is only allowed to provide properties or links supporting the write operation.  A project link must be set when creating work packages through this route.  When setting start date, finish date, and duration together, their correctness will be checked and a 422 error will be returned if one value does not match with the two others. You can make the server compute a value: set only two values in the request and the third one will be computed and returned in the response. For instance, when sending `{ \"startDate\": \"2022-08-23\", duration: \"P2D\" }`, the response will include `{ \"dueDate\": \"2022-08-24\" }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notify** | Option<**bool**> | Indicates whether change notifications (e.g. via E-Mail) should be sent. Note that this controls notifications for all users interested in changes to the work package (e.g. watchers, author and assignee), not just the current user. |  |[default to true]
**work_package_model** | Option<[**WorkPackageModel**](WorkPackageModel.md)> |  |  |

### Return type

[**models::WorkPackageModel**](Work_PackageModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_work_package_file_link

> models::FileLinkCollectionReadModel create_work_package_file_link(id, file_link_collection_write_model)
Creates file links.

Creates file links on a work package.  The request is interpreted as a bulk insert, where every element of the collection is validated separately. Each element contains the origin meta data and a link to the storage, the file link is about to point to. The storage link can be provided as a resource link with id or as the host url.  The file's id and name are considered mandatory information. The rest of the origin meta data SHOULD be provided by the client. The _mimeType_ SHOULD be a standard mime type. An empty mime type will be handled as unknown. To link a folder, the custom mime type `application/x-op-directory` MUST be used.  Up to 20 file links can be submitted at once.  If any element data is invalid, no file links will be created.  If a file link with matching origin id, work package, and storage already exists, then it will not create an additional file link or update the meta data. Instead the information from the existing file link will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**file_link_collection_write_model** | Option<[**FileLinkCollectionWriteModel**](FileLinkCollectionWriteModel.md)> |  |  |

### Return type

[**models::FileLinkCollectionReadModel**](FileLinkCollectionReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_work_package

> delete_work_package(id)
Delete Work Package

Deletes the work package, as well as:  - all associated time entries - its hierarchy of child work packages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_work_package_collection

> models::WorkPackagesModel get_project_work_package_collection(id, offset, page_size, filters, sort_by, group_by, show_sums, select)
Get work packages of project

Returns the collection of work packages that are related to the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project id | [required] |
**offset** | Option<**i32**> | Page number inside the requested collection. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page. |  |
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. If no filter is to be applied, the client should send an empty array (`[]`). |  |[default to [{ "status_id": { "operator": "o", "values": null }}]]
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. |  |[default to [["id", "asc"]]]
**group_by** | Option<**String**> | The column to group by. |  |
**show_sums** | Option<**bool**> | Indicates whether properties should be summed up if they support it. |  |[default to false]
**select** | Option<**String**> | Comma separated list of properties to include. |  |

### Return type

[**models::WorkPackagesModel**](Work_PackagesModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_relation_candidates

> serde_json::Value list_available_relation_candidates(id, page_size, filters, query, r#type, sort_by)
Available relation candidates



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project id | [required] |
**page_size** | Option<**i32**> | Maximum number of candidates to list (default 10) |  |
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same filters as the [work packages](https://www.openproject.org/docs/api/endpoints/work-packages/) endpoint. |  |
**query** | Option<**String**> | Shortcut for filtering by ID or subject |  |
**r#type** | Option<**String**> | Type of relation to find candidates for (default \"relates\") |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same sort criteria as the [work packages](https://www.openproject.org/docs/api/endpoints/work-packages/) endpoint. |  |[default to [["id", "asc"]]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_relations

> list_relations(id)
List relations

Lists all relations this work package is involved in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_watchers

> models::WatchersModel list_watchers(id)
List watchers



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |

### Return type

[**models::WatchersModel**](WatchersModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_work_package_activities

> serde_json::Value list_work_package_activities(id)
List work package activities



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_work_package_file_links

> models::FileLinkCollectionReadModel list_work_package_file_links(id, filters)
Gets all file links of a work package

Gets all file links of a work package.  As a side effect, for every file link a request is sent to the storage's origin to fetch live data and patch the file link's data before returning, as well as retrieving permissions of the user on this origin file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. The following filters are supported:  - storage |  |

### Return type

[**models::FileLinkCollectionReadModel**](FileLinkCollectionReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_work_package_schemas

> serde_json::Value list_work_package_schemas(filters)
List Work Package Schemas

List work package schemas.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | **String** | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + id: The schema's id  Schema id has the form `project_id-work_package_type_id`. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_work_packages

> models::WorkPackagesModel list_work_packages(offset, page_size, filters, sort_by, group_by, show_sums, select, timestamps)
List work packages

Returns a collection of work packages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Page number inside the requested collection. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page. |  |
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. If no filter is to be applied, the client should send an empty array (`[]`), otherwise a default filter is applied. A Currently supported filters are (there are additional filters added by modules):  - assigned_to - assignee_or_group - attachment_base - attachment_content - attachment_file_name - author - blocked - blocks - category - comment - created_at - custom_field - dates_interval - description - done_ratio - due_date - duplicated - duplicates - duration - estimated_hours - file_link_origin_id - follows - group - id - includes - linkable_to_storage_id - linkable_to_storage_url - manual_sort - milestone - only_subproject - parent - partof - precedes - principal_base - priority - project - relatable - relates - required - requires - responsible - role - search - start_date - status - storage_id - storage_url - subject - subject_or_id - subproject - type - typeahead - updated_at - version - watcher - work_package |  |[default to [{ "status_id": { "operator": "o", "values": null }}]]
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. |  |[default to [["id", "asc"]]]
**group_by** | Option<**String**> | The column to group by. |  |
**show_sums** | Option<**bool**> | Indicates whether properties should be summed up if they support it. |  |[default to false]
**select** | Option<**String**> | Comma separated list of properties to include. |  |
**timestamps** | Option<**String**> | In order to perform a [baseline comparison](/docs/api/baseline-comparisons), you may provide one or several timestamps in ISO-8601 format as comma-separated list. The timestamps may be absolute or relative, such as ISO8601 dates, ISO8601 durations and the following relative date keywords: \"oneDayAgo@HH:MM+HH:MM\", \"lastWorkingDay@HH:MM+HH:MM\", \"oneWeekAgo@HH:MM+HH:MM\", \"oneMonthAgo@HH:MM+HH:MM\". The first \"HH:MM\" part represents the zero paded hours and minutes. The last \"+HH:MM\" part represents the timezone offset from UTC associated with the time, the offset can be positive or negative e.g.\"oneDayAgo@01:00+01:00\", \"oneDayAgo@01:00-01:00\".  Usually, the first timestamp is the baseline date, the last timestamp is the current date. Values older than 1 day are accepted only with valid Enterprise Token available. |  |[default to PT0S]

### Return type

[**models::WorkPackagesModel**](Work_PackagesModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_available_assignees

> serde_json::Value project_available_assignees(id)
Project Available assignees

Gets a list of users that can be assigned to work packages in the given project.

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


## remove_watcher

> remove_watcher(id, user_id)
Remove watcher

Removes the specified user from the list of watchers for the given work package.  If the request succeeds, the specified user is not watching the work package anymore.  *Note: This might also be the case, if the specified user did not watch the work package prior to the request.*

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**user_id** | **i32** | User id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revisions

> serde_json::Value revisions(id)
Revisions

Gets a list of revisions that are linked to this work package, e.g., because it is referenced in the commit message of the revision. Only linked revisions from repositories are shown if the user has the view changesets permission in the defining project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_work_package

> models::WorkPackagePatchModel update_work_package(id, notify, work_package_model)
Update a Work Package

When calling this endpoint the client provides a single object, containing the properties and links that it wants to change, in the body. Note that it is only allowed to provide properties or links supporting the **write** operation.  Additionally to the fields the client wants to change, it is mandatory to provide the value of `lockVersion` which was received by the `GET` request this change originates from.  The value of `lockVersion` is used to implement [optimistic locking](https://en.wikipedia.org/wiki/Optimistic_concurrency_control).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**notify** | Option<**bool**> | Indicates whether change notifications should be sent. Note that this controls notifications for all users interested in changes to the work package (e.g. watchers, author and assignee), not just the current user. |  |[default to true]
**work_package_model** | Option<[**WorkPackageModel**](WorkPackageModel.md)> |  |  |

### Return type

[**models::WorkPackagePatchModel**](WorkPackagePatchModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_work_package

> models::WorkPackageModel view_work_package(id, timestamps)
View Work Package

Returns the specified work package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**timestamps** | Option<**String**> | In order to perform a [baseline comparison](/docs/api/baseline-comparisons) of the work-package attributes, you may provide one or several timestamps in ISO-8601 format as comma-separated list. The timestamps may be absolute or relative, such as ISO8601 dates, ISO8601 durations and the following relative date keywords: \"oneDayAgo@HH:MM+HH:MM\", \"lastWorkingDay@HH:MM+HH:MM\", \"oneWeekAgo@HH:MM+HH:MM\", \"oneMonthAgo@HH:MM+HH:MM\". The first \"HH:MM\" part represents the zero paded hours and minutes. The last \"+HH:MM\" part represents the timezone offset from UTC associated with the time, the offset can be positive or negative e.g.\"oneDayAgo@01:00+01:00\", \"oneDayAgo@01:00-01:00\".  Usually, the first timestamp is the baseline date, the last timestamp is the current date. Values older than 1 day are accepted only with valid Enterprise Token available. |  |[default to PT0S]

### Return type

[**models::WorkPackageModel**](Work_PackageModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_work_package_schema

> view_work_package_schema(identifier)
View Work Package Schema



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of the schema | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## work_package_available_assignees

> serde_json::Value work_package_available_assignees(id)
Work Package Available assignees

Gets a list of users that can be assigned to the given work package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## work_package_create_form

> work_package_create_form()
Work Package Create Form



### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## work_package_create_form_for_project

> work_package_create_form_for_project(id)
Work Package Create Form For Project



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the project in which the work package will be created | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## work_package_edit_form

> work_package_edit_form(id, work_package_model)
Work Package Edit Form

When calling this endpoint, the client provides a single object containing the properties and links to be edited, in the body.  Note that it is only allowed to provide properties or links supporting the write operation.  When setting start date, finish date, and duration together, their correctness will be checked and a 422 error will be returned if one value does not match with the two others. You can make the server compute a value: set only two values in the request and the third one will be computed and returned in the response. For instance, when sending `{ \"startDate\": \"2022-08-23\", duration: \"P2D\" }`, the response will include `{ \"dueDate\": \"2022-08-24\" }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the work package being modified | [required] |
**work_package_model** | Option<[**WorkPackageModel**](WorkPackageModel.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

