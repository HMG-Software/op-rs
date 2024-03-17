# \ActionsCapabilitiesApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_actions**](ActionsCapabilitiesApi.md#list_actions) | **GET** /api/v3/actions | List actions
[**list_capabilities**](ActionsCapabilitiesApi.md#list_capabilities) | **GET** /api/v3/capabilities | List capabilities
[**view_action**](ActionsCapabilitiesApi.md#view_action) | **GET** /api/v3/actions/{id} | View action
[**view_capabilities**](ActionsCapabilitiesApi.md#view_capabilities) | **GET** /api/v3/capabilities/{id} | View capabilities
[**view_global_context**](ActionsCapabilitiesApi.md#view_global_context) | **GET** /api/v3/capabilities/context/global | View global context



## list_actions

> serde_json::Value list_actions(filters, sort_by)
List actions

Returns a collection of actions. The client can choose to filter the actions similar to how work packages are filtered. In addition to the provided filters, the server will reduce the result set to only contain actions, for which the requesting client has sufficient permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + id: Returns only the action having the id or all actions except those having the id(s). |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported sorts are:  + *No sort supported yet* |  |[default to [["id", "asc"]]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_capabilities

> serde_json::Value list_capabilities(filters, sort_by)
List capabilities

Returns a collection of actions assigned to a principal in a context. The client can choose to filter the actions similar to how work packages are filtered. In addition to the provided filters, the server will reduce the result set to only contain actions, for which the requesting client has sufficient permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint.  + action: Get all capabilities of a certain action  + principal: Get all capabilities of a principal  + context: Get all capabilities within a context. Note that for a project context the client needs to provide `p{id}`, e.g. `p5` and for the global context a `g` |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported sorts are:  + id: Sort by the capabilities id |  |[default to [["id", "asc"]]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_action

> serde_json::Value view_action(id)
View action

Returns an individual action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | action id which is the name of the action | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_capabilities

> serde_json::Value view_capabilities(id)
View capabilities



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | capability id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_global_context

> serde_json::Value view_global_context()
View global context

Returns the global capability context. This context is necessary to consistently link to a context even if the context is not a project.

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

