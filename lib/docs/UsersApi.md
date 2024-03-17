# \UsersApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /api/v3/users | Create User
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /api/v3/users/{id} | Delete user
[**list_users**](UsersApi.md#list_users) | **GET** /api/v3/users | List Users
[**lock_user**](UsersApi.md#lock_user) | **POST** /api/v3/users/{id}/lock | Lock user
[**unlock_user**](UsersApi.md#unlock_user) | **DELETE** /api/v3/users/{id}/lock | Unlock user
[**update_user**](UsersApi.md#update_user) | **PATCH** /api/v3/users/{id} | Update user
[**user_update_form**](UsersApi.md#user_update_form) | **POST** /api/v3/users/{id}/form | User update form
[**view_user**](UsersApi.md#view_user) | **GET** /api/v3/users/{id} | View user
[**view_user_schema**](UsersApi.md#view_user_schema) | **GET** /api/v3/users/schema | View user schema



## create_user

> models::UserModel create_user(user_create_model)
Create User

Creates a new user. Only administrators and users with manage_user global permission are allowed to do so. When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body.  Valid values for `status`:  1) \"active\" - In this case a password has to be provided in addition to the other attributes.  2) \"invited\" - In this case nothing but the email address is required. The rest is optional. An invitation will be sent to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create_model** | Option<[**UserCreateModel**](UserCreateModel.md)> |  |  |

### Return type

[**models::UserModel**](UserModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(id)
Delete user

Permanently deletes the specified user account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | User id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> models::UserCollectionModel list_users(offset, page_size, filters, sort_by, select)
List Users

Lists users. Only administrators or users with any of the following can access this resource:  - `manage_members` - `manage_user` - `share_work_packages`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Page number inside the requested collection. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page. |  |
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + status: Status the user has  + group: Name of the group in which to-be-listed users are members.  + name: Filter users in whose first or last names, or email addresses the given string occurs.  + login: User's login |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. |  |
**select** | Option<**String**> | Comma separated list of properties to include. |  |

### Return type

[**models::UserCollectionModel**](UserCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_user

> models::UserModel lock_user(id)
Lock user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | User id | [required] |

### Return type

[**models::UserModel**](UserModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_user

> models::UserModel unlock_user(id)
Unlock user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | User id | [required] |

### Return type

[**models::UserModel**](UserModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::UserModel update_user(id, user_create_model)
Update user

Updates the user's writable attributes. When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | User id | [required] |
**user_create_model** | Option<[**UserCreateModel**](UserCreateModel.md)> |  |  |

### Return type

[**models::UserModel**](UserModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_update_form

> user_update_form(id)
User update form



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | User id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_user

> models::UserModel view_user(id)
View user



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id. Use `me` to reference current user, if any. | [required] |

### Return type

[**models::UserModel**](UserModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_user_schema

> serde_json::Value view_user_schema()
View user schema

The schema response use two exemplary custom fields that extend the schema response. Depending on your instance and custom field configuration, the response will look somewhat different.

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

