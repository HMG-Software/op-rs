# \PrincipalsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_placeholder_user**](PrincipalsApi.md#create_placeholder_user) | **POST** /api/v3/placeholder_users | Create placeholder user
[**create_user**](PrincipalsApi.md#create_user) | **POST** /api/v3/users | Create User
[**delete_placeholder_user**](PrincipalsApi.md#delete_placeholder_user) | **DELETE** /api/v3/placeholder_users/{id} | Delete placeholder user
[**delete_user**](PrincipalsApi.md#delete_user) | **DELETE** /api/v3/users/{id} | Delete user
[**list_placeholder_users**](PrincipalsApi.md#list_placeholder_users) | **GET** /api/v3/placeholder_users | List placehoder users
[**list_principals**](PrincipalsApi.md#list_principals) | **GET** /api/v3/principals | List principals
[**list_users**](PrincipalsApi.md#list_users) | **GET** /api/v3/users | List Users
[**update_placeholder_user**](PrincipalsApi.md#update_placeholder_user) | **PATCH** /api/v3/placeholder_users/{id} | Update placeholder user
[**update_user**](PrincipalsApi.md#update_user) | **PATCH** /api/v3/users/{id} | Update user
[**view_placeholder_user**](PrincipalsApi.md#view_placeholder_user) | **GET** /api/v3/placeholder_users/{id} | View placeholder user
[**view_user**](PrincipalsApi.md#view_user) | **GET** /api/v3/users/{id} | View user



## create_placeholder_user

> models::PlaceholderUserModel create_placeholder_user(placeholder_user_create_model)
Create placeholder user

Creates a new placeholder user. Only administrators and users with `manage_placeholder_user` global permission are allowed to do so. When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**placeholder_user_create_model** | Option<[**PlaceholderUserCreateModel**](PlaceholderUserCreateModel.md)> |  |  |

### Return type

[**models::PlaceholderUserModel**](PlaceholderUserModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## delete_placeholder_user

> delete_placeholder_user(id)
Delete placeholder user

Set the specified placeholder user to deleted status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Placeholder user id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

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


## list_placeholder_users

> models::PrincipalCollectionModel list_placeholder_users(filters, select)
List placehoder users

List all placeholder users. This can only be accessed if the requesting user has the global permission `manage_placeholder_user` or `manage_members` in any project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  - name: filters placeholder users by the name. - group: filters placeholder by the group it is contained in. - status: filters placeholder by the status it has. |  |
**select** | Option<**String**> | Comma separated list of properties to include. |  |

### Return type

[**models::PrincipalCollectionModel**](PrincipalCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_principals

> models::PrincipalCollectionModel list_principals(filters, select)
List principals

List all principals. The client can choose to filter the principals similar to how work packages are filtered. In addition to the provided filters, the server will reduce the result set to only contain principals who are members in projects the client is allowed to see.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  - type: filters principals by their type (*User*, *Group*, *PlaceholderUser*). - member: filters principals by the projects they are members in. - name: filters principals by the user or group name. - any_name_attribute: filters principals by the user or group first- and last name, email or login. - status: filters principals by their status number (active = *1*, registered = *2*, locked = *3*, invited = *4*) |  |
**select** | Option<**String**> | Comma separated list of properties to include. |  |

### Return type

[**models::PrincipalCollectionModel**](PrincipalCollectionModel.md)

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


## update_placeholder_user

> models::PlaceholderUserModel update_placeholder_user(id, placeholder_user_create_model)
Update placeholder user

Updates the placeholder user's writable attributes. When calling this endpoint the client provides a single object, containing at least the properties and links that are required, in the body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Placeholder user id | [required] |
**placeholder_user_create_model** | Option<[**PlaceholderUserCreateModel**](PlaceholderUserCreateModel.md)> |  |  |

### Return type

[**models::PlaceholderUserModel**](PlaceholderUserModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
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


## view_placeholder_user

> models::PlaceholderUserModel view_placeholder_user(id)
View placeholder user

Return the placeholder user resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The placeholder user id | [required] |

### Return type

[**models::PlaceholderUserModel**](PlaceholderUserModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

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

