# \GroupsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group**](GroupsApi.md#create_group) | **POST** /api/v3/groups | Create group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /api/v3/groups/{id} | Delete group
[**get_group**](GroupsApi.md#get_group) | **GET** /api/v3/groups/{id} | Get group
[**list_groups**](GroupsApi.md#list_groups) | **GET** /api/v3/groups | List groups
[**update_group**](GroupsApi.md#update_group) | **PATCH** /api/v3/groups/{id} | Update group



## create_group

> models::GroupModel create_group(group_write_model)
Create group

Creates a new group applying the attributes provided in the body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_write_model** | Option<[**GroupWriteModel**](GroupWriteModel.md)> |  |  |

### Return type

[**models::GroupModel**](GroupModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> delete_group(id)
Delete group

Deletes the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Group id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> models::GroupModel get_group(id)
Get group

Fetches a group resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Group id | [required] |

### Return type

[**models::GroupModel**](GroupModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_groups

> models::GroupCollectionModel list_groups(sort_by, select)
List groups

Returns a collection of groups. The client can choose to filter the groups similar to how work packages are filtered. In addition to the provided filters, the server will reduce the result set to only contain groups, for which the requesting client has sufficient permissions (*view_members*, *manage_members*).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported sorts are:  + id: Sort by primary key  + created_at: Sort by group creation datetime  + updated_at: Sort by the time the group was updated last |  |[default to [["id", "asc"]]]
**select** | Option<**String**> | Comma separated list of properties to include. |  |

### Return type

[**models::GroupCollectionModel**](GroupCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> models::GroupModel update_group(id, group_write_model)
Update group

Updates the given group by applying the attributes provided in the body.  Please note that the `members` array provided will override the existing set of members (similar to a PUT). A client thus has to provide the complete list of members the group is to have after the PATCH even if only one member is to be added.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Group id | [required] |
**group_write_model** | Option<[**GroupWriteModel**](GroupWriteModel.md)> |  |  |

### Return type

[**models::GroupModel**](GroupModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

