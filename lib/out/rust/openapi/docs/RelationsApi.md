# \RelationsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_relation**](RelationsApi.md#delete_relation) | **DELETE** /api/v3/relations/{id} | Delete Relation
[**edit_relation**](RelationsApi.md#edit_relation) | **PATCH** /api/v3/relations/{id} | Edit Relation
[**list_relations**](RelationsApi.md#list_relations) | **GET** /api/v3/relations | List Relations
[**relation_edit_form**](RelationsApi.md#relation_edit_form) | **POST** /api/v3/relations/{id}/form | Relation edit form
[**view_relation**](RelationsApi.md#view_relation) | **GET** /api/v3/relations/{id} | View Relation
[**view_relation_schema**](RelationsApi.md#view_relation_schema) | **GET** /api/v3/relations/schema | View relation schema
[**view_relation_schema_for_type**](RelationsApi.md#view_relation_schema_for_type) | **GET** /api/v3/relations/schema/{type} | View relation schema for type



## delete_relation

> delete_relation(id)
Delete Relation

Deletes the relation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Relation ID | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_relation

> models::RelationModel edit_relation(id)
Edit Relation

When calling this endpoint the client provides a single object, containing the properties and links that it wants to change, in the body. It is only allowed to provide properties or links supporting the **write** operation.  Note that changing the `type` of a relation invariably also changes the respective `reverseType` as well as the \"name\" of it. The returned Relation object will reflect that change. For instance if you change a Relation's `type` to \"follows\" then the `reverseType` will be changed to `precedes`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Relation ID | [required] |

### Return type

[**models::RelationModel**](RelationModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_relations

> serde_json::Value list_relations(filters, sort_by)
List Relations

Lists all relations according to the given (optional, logically conjunctive) filters and ordered by ID. The response only includes relations between work packages which the user is allowed to see.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Valid fields to filter by are:  + id - ID of relation  + from - ID of work package from which the filtered relations emanates.  + to - ID of work package to which this related points.  + involved - ID of either the `from` or the `to` work package.  + type - The type of relation to filter by, e.g. \"follows\". |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## relation_edit_form

> serde_json::Value relation_edit_form(id)
Relation edit form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the relation being modified | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_relation

> models::RelationModel view_relation(id)
View Relation



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Relation id | [required] |

### Return type

[**models::RelationModel**](RelationModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_relation_schema

> serde_json::Value view_relation_schema()
View relation schema



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


## view_relation_schema_for_type

> serde_json::Value view_relation_schema_for_type(r#type)
View relation schema for type



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Type of the schema | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

