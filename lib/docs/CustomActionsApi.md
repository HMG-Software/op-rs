# \CustomActionsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**execute_custom_action**](CustomActionsApi.md#execute_custom_action) | **POST** /api/v3/custom_actions/{id}/execute | Execute custom action
[**get_custom_action**](CustomActionsApi.md#get_custom_action) | **GET** /api/v3/custom_actions/{id} | Get a custom action



## execute_custom_action

> execute_custom_action(id, execute_custom_action_request)
Execute custom action

A POST to this end point executes the custom action on the work package provided in the payload. The altered work package will be returned. In order to avoid executing  the custom action unbeknown to a change that has already taken place, the client has to provide the work package's current lockVersion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the custom action to execute | [required] |
**execute_custom_action_request** | Option<[**ExecuteCustomActionRequest**](ExecuteCustomActionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_action

> models::CustomActionModel get_custom_action(id)
Get a custom action

Retrieves a custom action by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the custom action to fetch | [required] |

### Return type

[**models::CustomActionModel**](CustomActionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

