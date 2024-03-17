# \AttachmentsApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_attachment_to_meeting**](AttachmentsApi.md#add_attachment_to_meeting) | **POST** /api/v3/meetings/{id}/attachments | Add attachment to meeting
[**add_attachment_to_post**](AttachmentsApi.md#add_attachment_to_post) | **POST** /api/v3/posts/{id}/attachments | Add attachment to post
[**add_attachment_to_wiki_page**](AttachmentsApi.md#add_attachment_to_wiki_page) | **POST** /api/v3/wiki_pages/{id}/attachments | Add attachment to wiki page
[**create_attachment**](AttachmentsApi.md#create_attachment) | **POST** /api/v3/attachments | Create Attachment
[**create_work_package_attachment**](AttachmentsApi.md#create_work_package_attachment) | **POST** /api/v3/work_packages/{id}/attachments | Create work package attachment
[**delete_attachment**](AttachmentsApi.md#delete_attachment) | **DELETE** /api/v3/attachments/{id} | Delete attachment
[**list_attachments_by_meeting**](AttachmentsApi.md#list_attachments_by_meeting) | **GET** /api/v3/meetings/{id}/attachments | List attachments by meeting
[**list_attachments_by_post**](AttachmentsApi.md#list_attachments_by_post) | **GET** /api/v3/posts/{id}/attachments | List attachments by post
[**list_attachments_by_wiki_page**](AttachmentsApi.md#list_attachments_by_wiki_page) | **GET** /api/v3/wiki_pages/{id}/attachments | List attachments by wiki page
[**list_work_package_attachments**](AttachmentsApi.md#list_work_package_attachments) | **GET** /api/v3/work_packages/{id}/attachments | List attachments by work package
[**view_attachment**](AttachmentsApi.md#view_attachment) | **GET** /api/v3/attachments/{id} | View attachment



## add_attachment_to_meeting

> add_attachment_to_meeting(id)
Add attachment to meeting

Adds an attachment with the meeting as it's container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the meeting to receive the attachment | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_attachment_to_post

> add_attachment_to_post(id)
Add attachment to post

Adds an attachment with the post as it's container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the post to receive the attachment | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_attachment_to_wiki_page

> add_attachment_to_wiki_page(id)
Add attachment to wiki page

Adds an attachment with the wiki page as it's container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the wiki page to receive the attachment | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_attachment

> models::AttachmentModel create_attachment()
Create Attachment

Clients can create attachments without a container first and attach them later on. This is useful if the container does not exist at the time the attachment is uploaded. After the upload, the client can then claim such containerless attachments for any resource eligible (e.g. WorkPackage) on subsequent requests. The upload and the claiming *must* be done for the same user account. Attachments uploaded by another user cannot be claimed and once claimed for a resource, they cannot be claimed by another.  The upload request must be of type `multipart/form-data` with exactly two parts.  The first part *must* be called `metadata`. Its content type is expected to be `application/json`, the body *must* be a single JSON object, containing at least the `fileName` and optionally the attachments `description`.  The second part *must* be called `file`, its content type *should* match the mime type of the file. The body *must* be the raw content of the file. Note that a `filename` *must* be indicated in the `Content-Disposition` of this part, although it will be ignored. Instead the `fileName` inside the JSON of the metadata part will be used.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AttachmentModel**](AttachmentModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_work_package_attachment

> models::AttachmentModel create_work_package_attachment(id)
Create work package attachment

To add an attachment to a work package, a client needs to issue a request of type `multipart/form-data` with exactly two parts.  The first part *must* be called `metadata`. Its content type is expected to be `application/json`, the body *must* be a single JSON object, containing at least the `fileName` and optionally the attachments `description`.  The second part *must* be called `file`, its content type *should* match the mime type of the file. The body *must* be the raw content of the file. Note that a `filename` must be indicated in the `Content-Disposition` of this part, however it will be ignored. Instead the `fileName` inside the JSON of the metadata part will be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the work package to receive the attachment | [required] |

### Return type

[**models::AttachmentModel**](AttachmentModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_attachment

> delete_attachment(id)
Delete attachment

Permanently deletes the specified attachment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Attachment id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachments_by_meeting

> models::AttachmentsModel list_attachments_by_meeting(id)
List attachments by meeting



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the meeting whose attachments will be listed | [required] |

### Return type

[**models::AttachmentsModel**](Attachments_Model.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachments_by_post

> models::AttachmentsModel list_attachments_by_post(id)
List attachments by post



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the post whose attachments will be listed | [required] |

### Return type

[**models::AttachmentsModel**](Attachments_Model.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachments_by_wiki_page

> models::AttachmentsModel list_attachments_by_wiki_page(id)
List attachments by wiki page



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the wiki page whose attachments will be listed | [required] |

### Return type

[**models::AttachmentsModel**](Attachments_Model.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_work_package_attachments

> models::AttachmentsModel list_work_package_attachments(id)
List attachments by work package



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the work package whose attachments will be listed | [required] |

### Return type

[**models::AttachmentsModel**](Attachments_Model.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_attachment

> models::AttachmentModel view_attachment(id)
View attachment



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Attachment id | [required] |

### Return type

[**models::AttachmentModel**](AttachmentModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

