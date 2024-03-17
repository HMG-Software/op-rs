# \MembershipsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_membership**](MembershipsApi.md#create_membership) | **POST** /api/v3/memberships | Create a membership
[**delete_membership**](MembershipsApi.md#delete_membership) | **DELETE** /api/v3/memberships/{id} | Delete membership
[**form_create_membership**](MembershipsApi.md#form_create_membership) | **POST** /api/v3/memberships/form | Form create membership
[**form_update_membership**](MembershipsApi.md#form_update_membership) | **POST** /api/v3/memberships/{id}/form | Form update membership
[**get_membership**](MembershipsApi.md#get_membership) | **GET** /api/v3/memberships/{id} | Get a membership
[**get_membership_schema**](MembershipsApi.md#get_membership_schema) | **GET** /api/v3/memberships/schema | Schema membership
[**get_memberships_available_projects**](MembershipsApi.md#get_memberships_available_projects) | **GET** /api/v3/memberships/available_projects | Available projects for memberships
[**list_memberships**](MembershipsApi.md#list_memberships) | **GET** /api/v3/memberships | List memberships
[**update_membership**](MembershipsApi.md#update_membership) | **PATCH** /api/v3/memberships/{id} | Update membership



## create_membership

> models::MembershipReadModel create_membership(membership_write_model)
Create a membership

Creates a new membership applying the attributes provided in the body.  You can use the form and schema to retrieve the valid attribute values and by that be guided towards successful creation.  By providing a `notificationMessage` within the `_meta` block of the payload, the client can include a customized message to the user of the newly created membership. In case of a group, the message will be sent to every user belonging to the group.  By including `{ \"sendNotifications\": false }` within the `_meta` block of the payload, no notifications is send out at all.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_write_model** | Option<[**MembershipWriteModel**](MembershipWriteModel.md)> |  |  |

### Return type

[**models::MembershipReadModel**](MembershipReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_membership

> delete_membership(id)
Delete membership

Deletes the membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Membership id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## form_create_membership

> models::MembershipFormModel form_create_membership(membership_write_model)
Form create membership

Requests and validates the creation form for memberships. The request payload, if sent, is validated. The form endpoint itself does not create a membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_write_model** | Option<[**MembershipWriteModel**](MembershipWriteModel.md)> |  |  |

### Return type

[**models::MembershipFormModel**](MembershipFormModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## form_update_membership

> models::MembershipReadModel form_update_membership(id, membership_write_model)
Form update membership

Requests and validates the update form for a membership identified by the given id. The request payload, if sent, is validated. The form endpoint itself does not change the membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Membership id | [required] |
**membership_write_model** | Option<[**MembershipWriteModel**](MembershipWriteModel.md)> |  |  |

### Return type

[**models::MembershipReadModel**](MembershipReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_membership

> models::MembershipReadModel get_membership(id)
Get a membership

Retrieves a membership resource identified by the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Membership id | [required] |

### Return type

[**models::MembershipReadModel**](MembershipReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_membership_schema

> models::MembershipSchemaModel get_membership_schema()
Schema membership

Retrieves the schema for the membership resource object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MembershipSchemaModel**](MembershipSchemaModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_memberships_available_projects

> models::ProjectCollectionModel get_memberships_available_projects()
Available projects for memberships

Gets a list of projects in which a membership can be created in. The list contains all projects in which the user issuing the request has the manage members permissions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProjectCollectionModel**](ProjectCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_memberships

> models::MembershipCollectionModel list_memberships(filters, sort_by)
List memberships

Returns a collection of memberships. The client can choose to filter the memberships similar to how work packages are filtered. In addition to the provided filters, the server will reduce the result set to only contain memberships, for which the requesting client has sufficient permissions (*view_members*, *manage_members*).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + any_name_attribute: filters memberships based on the name of the principal. All possible name variants   (and also email and login) are searched. + blocked: reduces the result set to all memberships that are temporarily blocked or that are not blocked   temporarily. + group: filters memberships based on the name of a group. The group however is not the principal used for   filtering. Rather, the memberships of the group are used as the filter values. + name: filters memberships based on the name of the principal. Note that only the name is used which depends   on a setting in the OpenProject instance. + principal: filters memberships based on the id of the principal. + project: filters memberships based on the id of the project. + role: filters memberships based on the id of any role assigned to the membership. + status: filters memberships based on the status of the principal. + created_at: filters memberships based on the time the membership was created. + updated_at: filters memberships based on the time the membership was updated last. |  |
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported sorts are:  + id: Sort by primary key + name: Sort by the name of the principal. Note that this depends on the setting for how the name is to be   displayed at least for users. + email: Sort by the email address of the principal. Groups and principal users, which do not have an email,   are sorted last. + status: Sort by the status of the principal. Groups and principal users, which do not have a status, are   sorted together with the active users. + created_at: Sort by membership creation datetime + updated_at: Sort by the time the membership was updated last |  |[default to [["id", "asc"]]]

### Return type

[**models::MembershipCollectionModel**](MembershipCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_membership

> models::MembershipReadModel update_membership(id, membership_write_model)
Update membership

Updates the given membership by applying the attributes provided in the body.  By providing a `notificationMessage` within the `_meta` block of the payload, the client can include a customized message to the user of the updated membership. In case of a group, the message will be sent to every user belonging to the group.  By including `{ \"sendNotifications\": false }` within the `_meta` block of the payload, no notifications is send out at all.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Membership id | [required] |
**membership_write_model** | Option<[**MembershipWriteModel**](MembershipWriteModel.md)> |  |  |

### Return type

[**models::MembershipReadModel**](MembershipReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

