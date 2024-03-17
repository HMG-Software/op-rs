# \BudgetsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**view_budget**](BudgetsApi.md#view_budget) | **GET** /api/v3/budgets/{id} | view Budget
[**view_budgets_of_a_project**](BudgetsApi.md#view_budgets_of_a_project) | **GET** /api/v3/projects/{id}/budgets | view Budgets of a Project



## view_budget

> models::BudgetModel view_budget(id)
view Budget



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Budget id | [required] |

### Return type

[**models::BudgetModel**](BudgetModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_budgets_of_a_project

> serde_json::Value view_budgets_of_a_project(id)
view Budgets of a Project



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

