# \NotificationsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_notifications**](NotificationsApi.md#list_notifications) | **GET** /api/v3/notifications | Get notification collection
[**read_notification**](NotificationsApi.md#read_notification) | **POST** /api/v3/notifications/{id}/read_ian | Read notification
[**read_notifications**](NotificationsApi.md#read_notifications) | **POST** /api/v3/notifications/read_ian | Read all notifications
[**unread_notification**](NotificationsApi.md#unread_notification) | **POST** /api/v3/notifications/{id}/unread_ian | Unread notification
[**unread_notifications**](NotificationsApi.md#unread_notifications) | **POST** /api/v3/notifications/unread_ian | Unread all notifications
[**view_notification**](NotificationsApi.md#view_notification) | **GET** /api/v3/notifications/{id} | Get the notification
[**view_notification_detail**](NotificationsApi.md#view_notification_detail) | **GET** /api/v3/notifications/{notification_id}/details/{id} | Get a notification detail



## list_notifications

> models::NotificationCollectionModel list_notifications(offset, page_size, sort_by, group_by, filters)
Get notification collection

Returns the collection of available in-app notifications. The notifications returned depend on the provided parameters and also on the requesting user's permissions.  Contrary to most collections, this one also links to and embeds schemas for the `details` properties of the notifications returned. This is an optimization. Clients will receive the information necessary to display the various types of details that a notification can carry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Page number inside the requested collection. |  |[default to 1]
**page_size** | Option<**i32**> | Number of elements to display per page. |  |[default to 20]
**sort_by** | Option<**String**> | JSON specifying sort criteria. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported sorts are:  + id: Sort by primary key  + reason: Sort by notification reason  + readIAN: Sort by read status |  |
**group_by** | Option<**String**> | string specifying group_by criteria.  + reason: Group by notification reason  + project: Sort by associated project |  |
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + id: Filter by primary key  + project: Filter by the project the notification was created in  + readIAN: Filter by read status  + reason: Filter by the reason, e.g. 'mentioned' or 'assigned' the notification was created because of  + resourceId: Filter by the id of the resource the notification was created for. Ideally used together with the `resourceType` filter.  + resourceType: Filter by the type of the resource the notification was created for. Ideally used together with the `resourceId` filter. |  |

### Return type

[**models::NotificationCollectionModel**](NotificationCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_notification

> read_notification(id)
Read notification

Marks the given notification as read.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | notification id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_notifications

> read_notifications(filters)
Read all notifications

Marks the whole notification collection as read. The collection contains only elements the authenticated user can see, and can be further reduced with filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + id: Filter by primary key  + project: Filter by the project the notification was created in  + reason: Filter by the reason, e.g. 'mentioned' or 'assigned' the notification was created because of  + resourceId: Filter by the id of the resource the notification was created for. Ideally used together with the   `resourceType` filter.  + resourceType: Filter by the type of the resource the notification was created for. Ideally used together with   the `resourceId` filter. |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unread_notification

> unread_notification(id)
Unread notification

Marks the given notification as unread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | notification id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unread_notifications

> unread_notifications(filters)
Unread all notifications

Marks the whole notification collection as unread. The collection contains only elements the authenticated user can see, and can be further reduced with filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + id: Filter by primary key  + project: Filter by the project the notification was created in  + reason: Filter by the reason, e.g. 'mentioned' or 'assigned' the notification was created because of  + resourceId: Filter by the id of the resource the notification was created for. Ideally used together with the   `resourceType` filter.  + resourceType: Filter by the type of the resource the notification was created for. Ideally used together with   the `resourceId` filter. |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_notification

> models::NotificationModel view_notification(id)
Get the notification

Returns the notification identified by the notification id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | notification id | [required] |

### Return type

[**models::NotificationModel**](NotificationModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_notification_detail

> models::ValuesPropertyModel view_notification_detail(notification_id, id)
Get a notification detail

Returns an individual detail of a notification identified by the notification id and the id of the detail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **i32** | notification id | [required] |
**id** | **i32** | detail id | [required] |

### Return type

[**models::ValuesPropertyModel**](ValuesPropertyModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

