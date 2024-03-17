# \GridsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_grid**](GridsApi.md#create_grid) | **POST** /api/v3/grids | Create a grid
[**get_grid**](GridsApi.md#get_grid) | **GET** /api/v3/grids/{id} | Get a grid
[**grid_create_form**](GridsApi.md#grid_create_form) | **POST** /api/v3/grids/form | Grid Create Form
[**grid_update_form**](GridsApi.md#grid_update_form) | **POST** /api/v3/grids/{id}/form | Grid Update Form
[**list_grids**](GridsApi.md#list_grids) | **GET** /api/v3/grids | List grids
[**update_grid**](GridsApi.md#update_grid) | **PATCH** /api/v3/grids/{id} | Update a grid



## create_grid

> models::GridReadModel create_grid(grid_write_model)
Create a grid

Creates a new grid applying the attributes provided in the body. The constraints applied to the grid depend on the page the grid is placed in which is why the create form end point should be used to be guided when wanting to create a grid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grid_write_model** | Option<[**GridWriteModel**](GridWriteModel.md)> |  |  |

### Return type

[**models::GridReadModel**](GridReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_grid

> models::GridReadModel get_grid(id)
Get a grid

Fetches a single grid identified by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Grid id | [required] |

### Return type

[**models::GridReadModel**](GridReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grid_create_form

> grid_create_form()
Grid Create Form



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


## grid_update_form

> serde_json::Value grid_update_form(id)
Grid Update Form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the grid being modified | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_grids

> models::GridCollectionModel list_grids(offset, page_size, filters)
List grids

Lists all grids matching the provided filters and being part of the selected query page. The grids returned will also depend on the permissions of the requesting user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Page number inside the requested collection. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page. |  |[default to 30]
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  - page: Filter grid by work package |  |

### Return type

[**models::GridCollectionModel**](GridCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_grid

> models::GridReadModel update_grid(grid_write_model)
Update a grid

Updates the given grid by applying the attributes provided in the body. The constraints applied to the grid depend on the page the grid is placed in which is why the create form end point should be used to be guided when wanting to update a grid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grid_write_model** | Option<[**GridWriteModel**](GridWriteModel.md)> |  |  |

### Return type

[**models::GridReadModel**](GridReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

