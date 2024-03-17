# \RolesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_roles**](RolesApi.md#list_roles) | **GET** /api/v3/roles | List roles
[**view_role**](RolesApi.md#view_role) | **GET** /api/v3/roles/{id} | View role



## list_roles

> serde_json::Value list_roles(filters)
List roles

List all defined roles. This includes built in roles like 'Anonymous' and 'Non member'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + grantable: filters roles based on whether they are selectable for a membership  + unit: filters roles based on the unit ('project' or 'system') for which they are selectable for a membership |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_role

> models::RoleModel view_role(id)
View role

Fetch an individual role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Role id | [required] |

### Return type

[**models::RoleModel**](RoleModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

