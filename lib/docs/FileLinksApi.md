# \FileLinksApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_storage**](FileLinksApi.md#create_storage) | **POST** /api/v3/storages | Creates a storage.
[**create_storage_oauth_credentials**](FileLinksApi.md#create_storage_oauth_credentials) | **POST** /api/v3/storages/{id}/oauth_client_credentials | Creates an oauth client credentials object for a storage.
[**create_work_package_file_link**](FileLinksApi.md#create_work_package_file_link) | **POST** /api/v3/work_packages/{id}/file_links | Creates file links.
[**delete_file_link**](FileLinksApi.md#delete_file_link) | **DELETE** /api/v3/file_links/{id} | Removes a file link.
[**delete_storage**](FileLinksApi.md#delete_storage) | **DELETE** /api/v3/storages/{id} | Delete a storage
[**download_file_link**](FileLinksApi.md#download_file_link) | **GET** /api/v3/file_links/{id}/download | Creates a download uri of the linked file.
[**get_project_storage**](FileLinksApi.md#get_project_storage) | **GET** /api/v3/project_storages/{id} | Gets a project storage
[**get_storage**](FileLinksApi.md#get_storage) | **GET** /api/v3/storages/{id} | Get a storage
[**get_storage_files**](FileLinksApi.md#get_storage_files) | **GET** /api/v3/storages/{id}/files | Gets files of a storage.
[**list_project_storages**](FileLinksApi.md#list_project_storages) | **GET** /api/v3/project_storages | Gets a list of project storages
[**list_storages**](FileLinksApi.md#list_storages) | **GET** /api/v3/storages | Get Storages
[**list_work_package_file_links**](FileLinksApi.md#list_work_package_file_links) | **GET** /api/v3/work_packages/{id}/file_links | Gets all file links of a work package
[**open_file_link**](FileLinksApi.md#open_file_link) | **GET** /api/v3/file_links/{id}/open | Creates an opening uri of the linked file.
[**open_project_storage**](FileLinksApi.md#open_project_storage) | **GET** /api/v3/project_storages/{id}/open | Open the project storage
[**open_storage**](FileLinksApi.md#open_storage) | **GET** /api/v3/storages/{id}/open | Open the storage
[**prepare_storage_file_upload**](FileLinksApi.md#prepare_storage_file_upload) | **POST** /api/v3/storages/{id}/files/prepare_upload | Preparation of a direct upload of a file to the given storage.
[**update_storage**](FileLinksApi.md#update_storage) | **PATCH** /api/v3/storages/{id} | Update a storage
[**view_file_link**](FileLinksApi.md#view_file_link) | **GET** /api/v3/file_links/{id} | Gets a file link.



## create_storage

> models::StorageReadModel create_storage(storage_write_model)
Creates a storage.

Creates a storage resource. When creating a storage, a confidential OAuth 2 provider application is created automatically. The oauth client id and secret of the created OAuth application are returned in the response.  **IMPORTANT:** This is the only time, the oauth client secret is visible to the consumer. After that, the secret is hidden.  To update the storage with OAuth client credentials, which enable the storage resource to behave as an OAuth 2 client against an external OAuth 2 provider application, another request must be made to create those, see `POST /api/v3/storages/{id}/oauth_client_credentials`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_write_model** | Option<[**StorageWriteModel**](StorageWriteModel.md)> |  |  |

### Return type

[**models::StorageReadModel**](StorageReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_storage_oauth_credentials

> models::StorageReadModel create_storage_oauth_credentials(id, o_auth_client_credentials_write_model)
Creates an oauth client credentials object for a storage.

Inserts the OAuth 2 credentials into the storage, to allow the storage to act as an OAuth 2 client. Calling this endpoint on a storage that already contains OAuth 2 client credentials will replace them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Storage id | [required] |
**o_auth_client_credentials_write_model** | Option<[**OAuthClientCredentialsWriteModel**](OAuthClientCredentialsWriteModel.md)> |  |  |

### Return type

[**models::StorageReadModel**](StorageReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_work_package_file_link

> models::FileLinkCollectionReadModel create_work_package_file_link(id, file_link_collection_write_model)
Creates file links.

Creates file links on a work package.  The request is interpreted as a bulk insert, where every element of the collection is validated separately. Each element contains the origin meta data and a link to the storage, the file link is about to point to. The storage link can be provided as a resource link with id or as the host url.  The file's id and name are considered mandatory information. The rest of the origin meta data SHOULD be provided by the client. The _mimeType_ SHOULD be a standard mime type. An empty mime type will be handled as unknown. To link a folder, the custom mime type `application/x-op-directory` MUST be used.  Up to 20 file links can be submitted at once.  If any element data is invalid, no file links will be created.  If a file link with matching origin id, work package, and storage already exists, then it will not create an additional file link or update the meta data. Instead the information from the existing file link will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**file_link_collection_write_model** | Option<[**FileLinkCollectionWriteModel**](FileLinkCollectionWriteModel.md)> |  |  |

### Return type

[**models::FileLinkCollectionReadModel**](FileLinkCollectionReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file_link

> delete_file_link(id)
Removes a file link.

Removes a file link on a work package.  The request contains only the file link identifier as a path parameter. No request body is needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | File link id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_storage

> delete_storage(id)
Delete a storage

Deletes a storage resource. This also deletes all related records, like the created oauth application, client, and any file links created within this storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Storage id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file_link

> download_file_link(id)
Creates a download uri of the linked file.

Creates a uri to download the origin file linked by the given file link. This uri depends on the storage type and is always located on the origin storage itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | File link id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_storage

> models::ProjectStorageModel get_project_storage(id)
Gets a project storage

Gets a project storage resource. This resource contains all data that is applicable on the relation between a storage and a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project storage id | [required] |

### Return type

[**models::ProjectStorageModel**](ProjectStorageModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage

> models::StorageReadModel get_storage(id)
Get a storage

Gets a storage resource. As a side effect, a live connection to the storages origin is established to retrieve connection state data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Storage id | [required] |

### Return type

[**models::StorageReadModel**](StorageReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_files

> models::StorageFilesModel get_storage_files(id, parent)
Gets files of a storage.

Gets a collection of files from a storage.  If no `parent` context is given, the result is the content of the document root. With `parent` context given, the result contains the collections of files/directories from within the given parent file id.  If given `parent` context is no directory, `400 Bad Request` is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Storage id | [required] |
**parent** | Option<**String**> | Parent file identification |  |

### Return type

[**models::StorageFilesModel**](StorageFilesModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_storages

> models::ProjectStorageCollectionModel list_project_storages(filters)
Gets a list of project storages

Gets a collection of all project storages that meet the provided filters and the user has permission to see them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  - project_id - storage_id - storage_url |  |[default to []]

### Return type

[**models::ProjectStorageCollectionModel**](ProjectStorageCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_storages

> models::StorageCollectionModel list_storages()
Get Storages

Returns a collection of storages.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StorageCollectionModel**](StorageCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_work_package_file_links

> models::FileLinkCollectionReadModel list_work_package_file_links(id, filters)
Gets all file links of a work package

Gets all file links of a work package.  As a side effect, for every file link a request is sent to the storage's origin to fetch live data and patch the file link's data before returning, as well as retrieving permissions of the user on this origin file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Work package id | [required] |
**filters** | Option<**String**> | JSON specifying filter conditions. Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. The following filters are supported:  - storage |  |

### Return type

[**models::FileLinkCollectionReadModel**](FileLinkCollectionReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_file_link

> open_file_link(id, location)
Creates an opening uri of the linked file.

Creates a uri to open the origin file linked by the given file link. This uri depends on the storage type and is always located on the origin storage itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | File link id | [required] |
**location** | Option<**bool**> | Boolean flag indicating, if the file should be opened directly or rather the directory location. |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_project_storage

> open_project_storage(id)
Open the project storage

Gets a redirect to the location of the project storage's remote origin. If the project storage has a project folder, it is opened at this location. If not, the storage root is opened.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Project storage id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_storage

> open_storage(id)
Open the storage

Gets a redirect to the location of the storage's remote origin. The storage's files root should be the target location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Storage id | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_storage_file_upload

> models::StorageFileUploadLinkModel prepare_storage_file_upload(id, storage_file_upload_preparation_model)
Preparation of a direct upload of a file to the given storage.

Executes a request that prepares a link for a direct upload to the storage.  The background here is, that the client needs to make a direct request to the storage instance for file uploading, but should not get access to the credentials, which are stored in the backend. The response contains a link object, that enables the client to execute a file upload without the real credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Storage id | [required] |
**storage_file_upload_preparation_model** | Option<[**StorageFileUploadPreparationModel**](StorageFileUploadPreparationModel.md)> |  |  |

### Return type

[**models::StorageFileUploadLinkModel**](StorageFileUploadLinkModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_storage

> models::StorageReadModel update_storage(id, storage_write_model)
Update a storage

Updates a storage resource. Only data that is not generated by the server can be updated. This excludes the OAuth 2 application data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Storage id | [required] |
**storage_write_model** | Option<[**StorageWriteModel**](StorageWriteModel.md)> |  |  |

### Return type

[**models::StorageReadModel**](StorageReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_file_link

> models::FileLinkReadModel view_file_link(id)
Gets a file link.

Gets a single file link resource of a work package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | File link id | [required] |

### Return type

[**models::FileLinkReadModel**](FileLinkReadModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

