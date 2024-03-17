# Rust API client for openproject-api.packageVersion=3.1.0

You're looking at the current **stable** documentation of the OpenProject APIv3. If you're interested in the current
development version, please go to [github.com/opf](https://github.com/opf/openproject/tree/dev/docs/api/apiv3).

## Introduction

The documentation for the APIv3 is written according to the [OpenAPI 3.0 Specification](https://swagger.io/specification/).
You can either view the static version of this documentation on the [website](https://www.openproject.org/docs/api/introduction/)
or the interactive version, rendered with [OpenAPI Explorer](https://github.com/Rhosys/openapi-explorer/blob/main/README.md),
in your OpenProject installation under `/api/docs`.
In the latter you can try out the various API endpoints directly interacting with our OpenProject data.
Moreover you can access the specification source itself under `/api/v3/spec.json` and `/api/v3/spec.yml`
(e.g. [here](https://community.openproject.org/api/v3/spec.yml)).

The APIv3 is a hypermedia REST API, a shorthand for \"Hypermedia As The Engine Of Application State\" (HATEOAS).
This means that each endpoint of this API will have links to other resources or actions defined in the resulting body.

These related resources and actions for any given resource will be context sensitive. For example, only actions that the
authenticated user can take are being rendered. This can be used to dynamically identify actions that the user might take for any
given response.

As an example, if you fetch a work package through the [Work Package endpoint](https://www.openproject.org/docs/api/endpoints/work-packages/), the `update` link will only
be present when the user you authenticated has been granted a permission to update the work package in the assigned project.

## HAL+JSON

HAL is a simple format that gives a consistent and easy way to hyperlink between resources in your API.
Read more in the following specification: [https://tools.ietf.org/html/draft-kelly-json-hal-08](https://tools.ietf.org/html/draft-kelly-json-hal-08)

**OpenProject API implementation of HAL+JSON format** enriches JSON and introduces a few meta properties:

- `_type` - specifies the type of the resource (e.g.: WorkPackage, Project)
- `_links` - contains all related resource and action links available for the resource
- `_embedded` - contains all embedded objects

HAL does not guarantee that embedded resources are embedded in their full representation, they might as well be
partially represented (e.g. some properties can be left out).
However in this API you have the guarantee that whenever a resource is **embedded**, it is embedded in its **full representation**.

## API response structure

All API responses contain a single HAL+JSON object, even collections of objects are technically represented by
a single HAL+JSON object that itself contains its members. More details on collections can be found
in the [Collections Section](https://www.openproject.org/docs/api/collections/).

## Authentication

The API supports the following authentication schemes: OAuth2, session based authentication, and basic auth.

Depending on the settings of the OpenProject instance many resources can be accessed without being authenticated.
In case the instance requires authentication on all requests the client will receive an **HTTP 401** status code
in response to any request.

Otherwise unauthenticated clients have all the permissions of the anonymous user.

### Session-based Authentication

This means you have to login to OpenProject via the Web-Interface to be authenticated in the API.
This method is well-suited for clients acting within the browser, like the Angular-Client built into OpenProject.

In this case, you always need to pass the HTTP header `X-Requested-With \"XMLHttpRequest\"` for authentication.

### API Key through Basic Auth

Users can authenticate towards the API v3 using basic auth with the user name `apikey` (NOT your login) and the API key as the password.
Users can find their API key on their account page.

Example:

```shell
API_KEY=2519132cdf62dcf5a66fd96394672079f9e9cad1
curl -u apikey:$API_KEY https://community.openproject.org/api/v3/users/42
```

### OAuth2.0 authentication

OpenProject allows authentication and authorization with OAuth2 with *Authorization code flow*, as well as *Client credentials* operation modes.

To get started, you first need to register an application in the OpenProject OAuth administration section of your installation.
This will save an entry for your application with a client unique identifier (`client_id`) and an accompanying secret key (`client_secret`).

You can then use one the following guides to perform the supported OAuth 2.0 flows:

- [Authorization code flow](https://oauth.net/2/grant-types/authorization-code)

- [Authorization code flow with PKCE](https://doorkeeper.gitbook.io/guides/ruby-on-rails/pkce-flow), recommended for clients unable to keep the client_secret confidential.

- [Client credentials](https://oauth.net/2/grant-types/client-credentials/) - Requires an application to be bound to an impersonating user for non-public access

### Why not username and password?

The simplest way to do basic auth would be to use a user's username and password naturally.
However, OpenProject already has supported API keys in the past for the API v2, though not through basic auth.

Using **username and password** directly would have some advantages:

* It is intuitive for the user who then just has to provide those just as they would when logging into OpenProject.

* No extra logic for token management necessary.

On the other hand using **API keys** has some advantages too, which is why we went for that:

* If compromised while saved on an insecure client the user only has to regenerate the API key instead of changing their password, too.

* They are naturally long and random which makes them invulnerable to dictionary attacks and harder to crack in general.

Most importantly users may not actually have a password to begin with. Specifically when they have registered
through an OpenID Connect provider.

## Cross-Origin Resource Sharing (CORS)

By default, the OpenProject API is _not_ responding with any CORS headers.
If you want to allow cross-domain AJAX calls against your OpenProject instance, you need to enable CORS headers being returned.

Please see [our API settings documentation](https://www.openproject.org/docs/system-admin-guide/api-and-webhooks/) on
how to selectively enable CORS.

## Allowed HTTP methods

- `GET` - Get a single resource or collection of resources

- `POST` - Create a new resource or perform

- `PATCH` - Update a resource

- `DELETE` - Delete a resource

## Compression

Responses are compressed if requested by the client. Currently [gzip](https://www.gzip.org/) and [deflate](https://tools.ietf.org/html/rfc1951)
are supported. The client signals the desired compression by setting the [`Accept-Encoding` header](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.3).
If no `Accept-Encoding` header is send, `Accept-Encoding: identity` is assumed which will result in the API responding uncompressed.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 3
- Package version: 3
- Generator version: 7.5.0-SNAPSHOT
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openproject-api.packageVersion&#x3D;3.1.0` and add the following to `Cargo.toml` under `[dependencies]`:

```
openproject-api.packageVersion=3.1.0 = { path = "./openproject-api.packageVersion=3.1.0" }
```

## Documentation for API Endpoints

All URIs are relative to *https://community.openproject.org*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ActionsCapabilitiesApi* | [**list_actions**](docs/ActionsCapabilitiesApi.md#list_actions) | **GET** /api/v3/actions | List actions
*ActionsCapabilitiesApi* | [**list_capabilities**](docs/ActionsCapabilitiesApi.md#list_capabilities) | **GET** /api/v3/capabilities | List capabilities
*ActionsCapabilitiesApi* | [**view_action**](docs/ActionsCapabilitiesApi.md#view_action) | **GET** /api/v3/actions/{id} | View action
*ActionsCapabilitiesApi* | [**view_capabilities**](docs/ActionsCapabilitiesApi.md#view_capabilities) | **GET** /api/v3/capabilities/{id} | View capabilities
*ActionsCapabilitiesApi* | [**view_global_context**](docs/ActionsCapabilitiesApi.md#view_global_context) | **GET** /api/v3/capabilities/context/global | View global context
*ActivitiesApi* | [**update_activity**](docs/ActivitiesApi.md#update_activity) | **PATCH** /api/v3/activities/{id} | Update activity
*ActivitiesApi* | [**view_activity**](docs/ActivitiesApi.md#view_activity) | **GET** /api/v3/activities/{id} | View activity
*AttachmentsApi* | [**add_attachment_to_meeting**](docs/AttachmentsApi.md#add_attachment_to_meeting) | **POST** /api/v3/meetings/{id}/attachments | Add attachment to meeting
*AttachmentsApi* | [**add_attachment_to_post**](docs/AttachmentsApi.md#add_attachment_to_post) | **POST** /api/v3/posts/{id}/attachments | Add attachment to post
*AttachmentsApi* | [**add_attachment_to_wiki_page**](docs/AttachmentsApi.md#add_attachment_to_wiki_page) | **POST** /api/v3/wiki_pages/{id}/attachments | Add attachment to wiki page
*AttachmentsApi* | [**create_attachment**](docs/AttachmentsApi.md#create_attachment) | **POST** /api/v3/attachments | Create Attachment
*AttachmentsApi* | [**create_work_package_attachment**](docs/AttachmentsApi.md#create_work_package_attachment) | **POST** /api/v3/work_packages/{id}/attachments | Create work package attachment
*AttachmentsApi* | [**delete_attachment**](docs/AttachmentsApi.md#delete_attachment) | **DELETE** /api/v3/attachments/{id} | Delete attachment
*AttachmentsApi* | [**list_attachments_by_meeting**](docs/AttachmentsApi.md#list_attachments_by_meeting) | **GET** /api/v3/meetings/{id}/attachments | List attachments by meeting
*AttachmentsApi* | [**list_attachments_by_post**](docs/AttachmentsApi.md#list_attachments_by_post) | **GET** /api/v3/posts/{id}/attachments | List attachments by post
*AttachmentsApi* | [**list_attachments_by_wiki_page**](docs/AttachmentsApi.md#list_attachments_by_wiki_page) | **GET** /api/v3/wiki_pages/{id}/attachments | List attachments by wiki page
*AttachmentsApi* | [**list_work_package_attachments**](docs/AttachmentsApi.md#list_work_package_attachments) | **GET** /api/v3/work_packages/{id}/attachments | List attachments by work package
*AttachmentsApi* | [**view_attachment**](docs/AttachmentsApi.md#view_attachment) | **GET** /api/v3/attachments/{id} | View attachment
*BudgetsApi* | [**view_budget**](docs/BudgetsApi.md#view_budget) | **GET** /api/v3/budgets/{id} | view Budget
*BudgetsApi* | [**view_budgets_of_a_project**](docs/BudgetsApi.md#view_budgets_of_a_project) | **GET** /api/v3/projects/{id}/budgets | view Budgets of a Project
*CategoriesApi* | [**list_categories_of_a_project**](docs/CategoriesApi.md#list_categories_of_a_project) | **GET** /api/v3/projects/{id}/categories | List categories of a project
*CategoriesApi* | [**view_category**](docs/CategoriesApi.md#view_category) | **GET** /api/v3/categories/{id} | View Category
*CollectionsApi* | [**view_aggregated_result**](docs/CollectionsApi.md#view_aggregated_result) | **GET** /api/v3/examples | view aggregated result
*ConfigurationApi* | [**view_configuration**](docs/ConfigurationApi.md#view_configuration) | **GET** /api/v3/configuration | View configuration
*CustomActionsApi* | [**execute_custom_action**](docs/CustomActionsApi.md#execute_custom_action) | **POST** /api/v3/custom_actions/{id}/execute | Execute custom action
*CustomActionsApi* | [**get_custom_action**](docs/CustomActionsApi.md#get_custom_action) | **GET** /api/v3/custom_actions/{id} | Get a custom action
*CustomOptionsApi* | [**view_custom_option**](docs/CustomOptionsApi.md#view_custom_option) | **GET** /api/v3/custom_options/{id} | View Custom Option
*DocumentsApi* | [**list_documents**](docs/DocumentsApi.md#list_documents) | **GET** /api/v3/documents | List Documents
*DocumentsApi* | [**view_document**](docs/DocumentsApi.md#view_document) | **GET** /api/v3/documents/{id} | View document
*FileLinksApi* | [**create_storage**](docs/FileLinksApi.md#create_storage) | **POST** /api/v3/storages | Creates a storage.
*FileLinksApi* | [**create_storage_oauth_credentials**](docs/FileLinksApi.md#create_storage_oauth_credentials) | **POST** /api/v3/storages/{id}/oauth_client_credentials | Creates an oauth client credentials object for a storage.
*FileLinksApi* | [**create_work_package_file_link**](docs/FileLinksApi.md#create_work_package_file_link) | **POST** /api/v3/work_packages/{id}/file_links | Creates file links.
*FileLinksApi* | [**delete_file_link**](docs/FileLinksApi.md#delete_file_link) | **DELETE** /api/v3/file_links/{id} | Removes a file link.
*FileLinksApi* | [**delete_storage**](docs/FileLinksApi.md#delete_storage) | **DELETE** /api/v3/storages/{id} | Delete a storage
*FileLinksApi* | [**download_file_link**](docs/FileLinksApi.md#download_file_link) | **GET** /api/v3/file_links/{id}/download | Creates a download uri of the linked file.
*FileLinksApi* | [**get_project_storage**](docs/FileLinksApi.md#get_project_storage) | **GET** /api/v3/project_storages/{id} | Gets a project storage
*FileLinksApi* | [**get_storage**](docs/FileLinksApi.md#get_storage) | **GET** /api/v3/storages/{id} | Get a storage
*FileLinksApi* | [**get_storage_files**](docs/FileLinksApi.md#get_storage_files) | **GET** /api/v3/storages/{id}/files | Gets files of a storage.
*FileLinksApi* | [**list_project_storages**](docs/FileLinksApi.md#list_project_storages) | **GET** /api/v3/project_storages | Gets a list of project storages
*FileLinksApi* | [**list_storages**](docs/FileLinksApi.md#list_storages) | **GET** /api/v3/storages | Get Storages
*FileLinksApi* | [**list_work_package_file_links**](docs/FileLinksApi.md#list_work_package_file_links) | **GET** /api/v3/work_packages/{id}/file_links | Gets all file links of a work package
*FileLinksApi* | [**open_file_link**](docs/FileLinksApi.md#open_file_link) | **GET** /api/v3/file_links/{id}/open | Creates an opening uri of the linked file.
*FileLinksApi* | [**open_project_storage**](docs/FileLinksApi.md#open_project_storage) | **GET** /api/v3/project_storages/{id}/open | Open the project storage
*FileLinksApi* | [**open_storage**](docs/FileLinksApi.md#open_storage) | **GET** /api/v3/storages/{id}/open | Open the storage
*FileLinksApi* | [**prepare_storage_file_upload**](docs/FileLinksApi.md#prepare_storage_file_upload) | **POST** /api/v3/storages/{id}/files/prepare_upload | Preparation of a direct upload of a file to the given storage.
*FileLinksApi* | [**update_storage**](docs/FileLinksApi.md#update_storage) | **PATCH** /api/v3/storages/{id} | Update a storage
*FileLinksApi* | [**view_file_link**](docs/FileLinksApi.md#view_file_link) | **GET** /api/v3/file_links/{id} | Gets a file link.
*FormsApi* | [**show_or_validate_form**](docs/FormsApi.md#show_or_validate_form) | **POST** /api/v3/example/form | show or validate form
*GridsApi* | [**create_grid**](docs/GridsApi.md#create_grid) | **POST** /api/v3/grids | Create a grid
*GridsApi* | [**get_grid**](docs/GridsApi.md#get_grid) | **GET** /api/v3/grids/{id} | Get a grid
*GridsApi* | [**grid_create_form**](docs/GridsApi.md#grid_create_form) | **POST** /api/v3/grids/form | Grid Create Form
*GridsApi* | [**grid_update_form**](docs/GridsApi.md#grid_update_form) | **POST** /api/v3/grids/{id}/form | Grid Update Form
*GridsApi* | [**list_grids**](docs/GridsApi.md#list_grids) | **GET** /api/v3/grids | List grids
*GridsApi* | [**update_grid**](docs/GridsApi.md#update_grid) | **PATCH** /api/v3/grids/{id} | Update a grid
*GroupsApi* | [**create_group**](docs/GroupsApi.md#create_group) | **POST** /api/v3/groups | Create group
*GroupsApi* | [**delete_group**](docs/GroupsApi.md#delete_group) | **DELETE** /api/v3/groups/{id} | Delete group
*GroupsApi* | [**get_group**](docs/GroupsApi.md#get_group) | **GET** /api/v3/groups/{id} | Get group
*GroupsApi* | [**list_groups**](docs/GroupsApi.md#list_groups) | **GET** /api/v3/groups | List groups
*GroupsApi* | [**update_group**](docs/GroupsApi.md#update_group) | **PATCH** /api/v3/groups/{id} | Update group
*HelpTextsApi* | [**get_help_text**](docs/HelpTextsApi.md#get_help_text) | **GET** /api/v3/help_texts/{id} | Get help text
*HelpTextsApi* | [**list_help_texts**](docs/HelpTextsApi.md#list_help_texts) | **GET** /api/v3/help_texts | List help texts
*MeetingsApi* | [**view_meeting**](docs/MeetingsApi.md#view_meeting) | **GET** /api/v3/meetings/{id} | View Meeting Page
*MembershipsApi* | [**create_membership**](docs/MembershipsApi.md#create_membership) | **POST** /api/v3/memberships | Create a membership
*MembershipsApi* | [**delete_membership**](docs/MembershipsApi.md#delete_membership) | **DELETE** /api/v3/memberships/{id} | Delete membership
*MembershipsApi* | [**form_create_membership**](docs/MembershipsApi.md#form_create_membership) | **POST** /api/v3/memberships/form | Form create membership
*MembershipsApi* | [**form_update_membership**](docs/MembershipsApi.md#form_update_membership) | **POST** /api/v3/memberships/{id}/form | Form update membership
*MembershipsApi* | [**get_membership**](docs/MembershipsApi.md#get_membership) | **GET** /api/v3/memberships/{id} | Get a membership
*MembershipsApi* | [**get_membership_schema**](docs/MembershipsApi.md#get_membership_schema) | **GET** /api/v3/memberships/schema | Schema membership
*MembershipsApi* | [**get_memberships_available_projects**](docs/MembershipsApi.md#get_memberships_available_projects) | **GET** /api/v3/memberships/available_projects | Available projects for memberships
*MembershipsApi* | [**list_memberships**](docs/MembershipsApi.md#list_memberships) | **GET** /api/v3/memberships | List memberships
*MembershipsApi* | [**update_membership**](docs/MembershipsApi.md#update_membership) | **PATCH** /api/v3/memberships/{id} | Update membership
*NewsApi* | [**list_news**](docs/NewsApi.md#list_news) | **GET** /api/v3/news | List News
*NewsApi* | [**view_news**](docs/NewsApi.md#view_news) | **GET** /api/v3/news/{id} | View news
*NotificationsApi* | [**list_notifications**](docs/NotificationsApi.md#list_notifications) | **GET** /api/v3/notifications | Get notification collection
*NotificationsApi* | [**read_notification**](docs/NotificationsApi.md#read_notification) | **POST** /api/v3/notifications/{id}/read_ian | Read notification
*NotificationsApi* | [**read_notifications**](docs/NotificationsApi.md#read_notifications) | **POST** /api/v3/notifications/read_ian | Read all notifications
*NotificationsApi* | [**unread_notification**](docs/NotificationsApi.md#unread_notification) | **POST** /api/v3/notifications/{id}/unread_ian | Unread notification
*NotificationsApi* | [**unread_notifications**](docs/NotificationsApi.md#unread_notifications) | **POST** /api/v3/notifications/unread_ian | Unread all notifications
*NotificationsApi* | [**view_notification**](docs/NotificationsApi.md#view_notification) | **GET** /api/v3/notifications/{id} | Get the notification
*NotificationsApi* | [**view_notification_detail**](docs/NotificationsApi.md#view_notification_detail) | **GET** /api/v3/notifications/{notification_id}/details/{id} | Get a notification detail
*OAuth2Api* | [**get_oauth_application**](docs/OAuth2Api.md#get_oauth_application) | **GET** /api/v3/oauth_applications/{id} | Get the oauth application.
*OAuth2Api* | [**get_oauth_client_credentials**](docs/OAuth2Api.md#get_oauth_client_credentials) | **GET** /api/v3/oauth_client_credentials/{id} | Get the oauth client credentials object.
*PostsApi* | [**view_post**](docs/PostsApi.md#view_post) | **GET** /api/v3/posts/{id} | View Post
*PreviewingApi* | [**preview_markdown_document**](docs/PreviewingApi.md#preview_markdown_document) | **POST** /api/v3/render/markdown | Preview Markdown document
*PreviewingApi* | [**preview_plain_document**](docs/PreviewingApi.md#preview_plain_document) | **POST** /api/v3/render/plain | Preview plain document
*PrincipalsApi* | [**create_placeholder_user**](docs/PrincipalsApi.md#create_placeholder_user) | **POST** /api/v3/placeholder_users | Create placeholder user
*PrincipalsApi* | [**create_user**](docs/PrincipalsApi.md#create_user) | **POST** /api/v3/users | Create User
*PrincipalsApi* | [**delete_placeholder_user**](docs/PrincipalsApi.md#delete_placeholder_user) | **DELETE** /api/v3/placeholder_users/{id} | Delete placeholder user
*PrincipalsApi* | [**delete_user**](docs/PrincipalsApi.md#delete_user) | **DELETE** /api/v3/users/{id} | Delete user
*PrincipalsApi* | [**list_placeholder_users**](docs/PrincipalsApi.md#list_placeholder_users) | **GET** /api/v3/placeholder_users | List placehoder users
*PrincipalsApi* | [**list_principals**](docs/PrincipalsApi.md#list_principals) | **GET** /api/v3/principals | List principals
*PrincipalsApi* | [**list_users**](docs/PrincipalsApi.md#list_users) | **GET** /api/v3/users | List Users
*PrincipalsApi* | [**update_placeholder_user**](docs/PrincipalsApi.md#update_placeholder_user) | **PATCH** /api/v3/placeholder_users/{id} | Update placeholder user
*PrincipalsApi* | [**update_user**](docs/PrincipalsApi.md#update_user) | **PATCH** /api/v3/users/{id} | Update user
*PrincipalsApi* | [**view_placeholder_user**](docs/PrincipalsApi.md#view_placeholder_user) | **GET** /api/v3/placeholder_users/{id} | View placeholder user
*PrincipalsApi* | [**view_user**](docs/PrincipalsApi.md#view_user) | **GET** /api/v3/users/{id} | View user
*PrioritiesApi* | [**list_all_priorities**](docs/PrioritiesApi.md#list_all_priorities) | **GET** /api/v3/priorities | List all Priorities
*PrioritiesApi* | [**view_priority**](docs/PrioritiesApi.md#view_priority) | **GET** /api/v3/priorities/{id} | View Priority
*ProjectsApi* | [**create_project**](docs/ProjectsApi.md#create_project) | **POST** /api/v3/projects | Create project
*ProjectsApi* | [**create_project_copy**](docs/ProjectsApi.md#create_project_copy) | **POST** /api/v3/projects/{id}/copy | Create project copy
*ProjectsApi* | [**delete_project**](docs/ProjectsApi.md#delete_project) | **DELETE** /api/v3/projects/{id} | Delete Project
*ProjectsApi* | [**list_available_parent_project_candidates**](docs/ProjectsApi.md#list_available_parent_project_candidates) | **GET** /api/v3/projects/available_parent_projects | List available parent project candidates
*ProjectsApi* | [**list_projects**](docs/ProjectsApi.md#list_projects) | **GET** /api/v3/projects | List projects
*ProjectsApi* | [**list_projects_with_version**](docs/ProjectsApi.md#list_projects_with_version) | **GET** /api/v3/versions/{id}/projects | List projects having version
*ProjectsApi* | [**project_copy_form**](docs/ProjectsApi.md#project_copy_form) | **POST** /api/v3/projects/{id}/copy/form | Project copy form
*ProjectsApi* | [**project_create_form**](docs/ProjectsApi.md#project_create_form) | **POST** /api/v3/projects/form | Project create form
*ProjectsApi* | [**project_update_form**](docs/ProjectsApi.md#project_update_form) | **POST** /api/v3/projects/{id}/form | Project update form
*ProjectsApi* | [**update_project**](docs/ProjectsApi.md#update_project) | **PATCH** /api/v3/projects/{id} | Update Project
*ProjectsApi* | [**view_project**](docs/ProjectsApi.md#view_project) | **GET** /api/v3/projects/{id} | View project
*ProjectsApi* | [**view_project_schema**](docs/ProjectsApi.md#view_project_schema) | **GET** /api/v3/projects/schema | View project schema
*ProjectsApi* | [**view_project_status**](docs/ProjectsApi.md#view_project_status) | **GET** /api/v3/project_statuses/{id} | View project status
*QueriesApi* | [**available_projects_for_query**](docs/QueriesApi.md#available_projects_for_query) | **GET** /api/v3/queries/available_projects | Available projects for query
*QueriesApi* | [**create_query**](docs/QueriesApi.md#create_query) | **POST** /api/v3/queries | Create query
*QueriesApi* | [**delete_query**](docs/QueriesApi.md#delete_query) | **DELETE** /api/v3/queries/{id} | Delete query
*QueriesApi* | [**edit_query**](docs/QueriesApi.md#edit_query) | **PATCH** /api/v3/queries/{id} | Edit Query
*QueriesApi* | [**list_queries**](docs/QueriesApi.md#list_queries) | **GET** /api/v3/queries | List queries
*QueriesApi* | [**query_create_form**](docs/QueriesApi.md#query_create_form) | **POST** /api/v3/queries/form | Query Create Form
*QueriesApi* | [**query_update_form**](docs/QueriesApi.md#query_update_form) | **POST** /api/v3/queries/{id}/form | Query Update Form
*QueriesApi* | [**star_query**](docs/QueriesApi.md#star_query) | **PATCH** /api/v3/queries/{id}/star | Star query
*QueriesApi* | [**unstar_query**](docs/QueriesApi.md#unstar_query) | **PATCH** /api/v3/queries/{id}/unstar | Unstar query
*QueriesApi* | [**view_default_query**](docs/QueriesApi.md#view_default_query) | **GET** /api/v3/queries/default | View default query
*QueriesApi* | [**view_default_query_for_project**](docs/QueriesApi.md#view_default_query_for_project) | **GET** /api/v3/projects/{id}/queries/default | View default query for project
*QueriesApi* | [**view_query**](docs/QueriesApi.md#view_query) | **GET** /api/v3/queries/{id} | View query
*QueriesApi* | [**view_schema_for_global_queries**](docs/QueriesApi.md#view_schema_for_global_queries) | **GET** /api/v3/queries/schema | View schema for global queries
*QueriesApi* | [**view_schema_for_project_queries**](docs/QueriesApi.md#view_schema_for_project_queries) | **GET** /api/v3/projects/{id}/queries/schema | View schema for project queries
*QueryColumnsApi* | [**view_query_column**](docs/QueryColumnsApi.md#view_query_column) | **GET** /api/v3/queries/columns/{id} | View Query Column
*QueryFilterInstanceSchemaApi* | [**list_query_filter_instance_schemas**](docs/QueryFilterInstanceSchemaApi.md#list_query_filter_instance_schemas) | **GET** /api/v3/queries/filter_instance_schemas | List Query Filter Instance Schemas
*QueryFilterInstanceSchemaApi* | [**list_query_filter_instance_schemas_for_project**](docs/QueryFilterInstanceSchemaApi.md#list_query_filter_instance_schemas_for_project) | **GET** /api/v3/projects/{id}/queries/filter_instance_schemas | List Query Filter Instance Schemas for Project
*QueryFilterInstanceSchemaApi* | [**view_query_filter_instance_schema**](docs/QueryFilterInstanceSchemaApi.md#view_query_filter_instance_schema) | **GET** /api/v3/queries/filter_instance_schemas/{id} | View Query Filter Instance Schema
*QueryFiltersApi* | [**view_query_filter**](docs/QueryFiltersApi.md#view_query_filter) | **GET** /api/v3/queries/filters/{id} | View Query Filter
*QueryOperatorsApi* | [**view_query_operator**](docs/QueryOperatorsApi.md#view_query_operator) | **GET** /api/v3/queries/operators/{id} | View Query Operator
*QuerySortBysApi* | [**view_query_sort_by**](docs/QuerySortBysApi.md#view_query_sort_by) | **GET** /api/v3/queries/sort_bys/{id} | View Query Sort By
*RelationsApi* | [**delete_relation**](docs/RelationsApi.md#delete_relation) | **DELETE** /api/v3/relations/{id} | Delete Relation
*RelationsApi* | [**edit_relation**](docs/RelationsApi.md#edit_relation) | **PATCH** /api/v3/relations/{id} | Edit Relation
*RelationsApi* | [**list_relations**](docs/RelationsApi.md#list_relations) | **GET** /api/v3/relations | List Relations
*RelationsApi* | [**relation_edit_form**](docs/RelationsApi.md#relation_edit_form) | **POST** /api/v3/relations/{id}/form | Relation edit form
*RelationsApi* | [**view_relation**](docs/RelationsApi.md#view_relation) | **GET** /api/v3/relations/{id} | View Relation
*RelationsApi* | [**view_relation_schema**](docs/RelationsApi.md#view_relation_schema) | **GET** /api/v3/relations/schema | View relation schema
*RelationsApi* | [**view_relation_schema_for_type**](docs/RelationsApi.md#view_relation_schema_for_type) | **GET** /api/v3/relations/schema/{type} | View relation schema for type
*RevisionsApi* | [**view_revision**](docs/RevisionsApi.md#view_revision) | **GET** /api/v3/revisions/{id} | View revision
*RolesApi* | [**list_roles**](docs/RolesApi.md#list_roles) | **GET** /api/v3/roles | List roles
*RolesApi* | [**view_role**](docs/RolesApi.md#view_role) | **GET** /api/v3/roles/{id} | View role
*RootApi* | [**view_root**](docs/RootApi.md#view_root) | **GET** /api/v3 | View root
*SchemasApi* | [**view_the_schema**](docs/SchemasApi.md#view_the_schema) | **GET** /api/v3/example/schema | view the schema
*StatusesApi* | [**list_all_statuses**](docs/StatusesApi.md#list_all_statuses) | **GET** /api/v3/statuses | List all Statuses
*StatusesApi* | [**view_status**](docs/StatusesApi.md#view_status) | **GET** /api/v3/statuses/{id} | View Status
*TimeEntriesApi* | [**available_projects_for_time_entries**](docs/TimeEntriesApi.md#available_projects_for_time_entries) | **GET** /api/v3/time_entries/available_projects | Available projects for time entries
*TimeEntriesApi* | [**create_time_entry**](docs/TimeEntriesApi.md#create_time_entry) | **POST** /api/v3/time_entries | Create time entry
*TimeEntriesApi* | [**delete_time_entry**](docs/TimeEntriesApi.md#delete_time_entry) | **DELETE** /api/v3/time_entries/{id} | Delete time entry
*TimeEntriesApi* | [**get_time_entry**](docs/TimeEntriesApi.md#get_time_entry) | **GET** /api/v3/time_entries/{id} | Get time entry
*TimeEntriesApi* | [**list_time_entries**](docs/TimeEntriesApi.md#list_time_entries) | **GET** /api/v3/time_entries | List time entries
*TimeEntriesApi* | [**time_entry_create_form**](docs/TimeEntriesApi.md#time_entry_create_form) | **POST** /api/v3/time_entries/form | Time entry create form
*TimeEntriesApi* | [**time_entry_update_form**](docs/TimeEntriesApi.md#time_entry_update_form) | **POST** /api/v3/time_entries/{id}/form | Time entry update form
*TimeEntriesApi* | [**update_time_entry**](docs/TimeEntriesApi.md#update_time_entry) | **PATCH** /api/v3/time_entries/{id} | update time entry
*TimeEntriesApi* | [**view_time_entry_schema**](docs/TimeEntriesApi.md#view_time_entry_schema) | **GET** /api/v3/time_entries/schema | View time entry schema
*TimeEntryActivitiesApi* | [**get_time_entries_activity**](docs/TimeEntryActivitiesApi.md#get_time_entries_activity) | **GET** /api/v3/time_entries/activity/{id} | View time entries activity
*TypesApi* | [**list_all_types**](docs/TypesApi.md#list_all_types) | **GET** /api/v3/types | List all Types
*TypesApi* | [**list_types_available_in_a_project**](docs/TypesApi.md#list_types_available_in_a_project) | **GET** /api/v3/projects/{id}/types | List types available in a project
*TypesApi* | [**view_type**](docs/TypesApi.md#view_type) | **GET** /api/v3/types/{id} | View Type
*UserPreferencesApi* | [**show_my_preferences**](docs/UserPreferencesApi.md#show_my_preferences) | **GET** /api/v3/my_preferences | Show my preferences
*UserPreferencesApi* | [**update_user_preferences**](docs/UserPreferencesApi.md#update_user_preferences) | **PATCH** /api/v3/my_preferences | Update my preferences
*UsersApi* | [**create_user**](docs/UsersApi.md#create_user) | **POST** /api/v3/users | Create User
*UsersApi* | [**delete_user**](docs/UsersApi.md#delete_user) | **DELETE** /api/v3/users/{id} | Delete user
*UsersApi* | [**list_users**](docs/UsersApi.md#list_users) | **GET** /api/v3/users | List Users
*UsersApi* | [**lock_user**](docs/UsersApi.md#lock_user) | **POST** /api/v3/users/{id}/lock | Lock user
*UsersApi* | [**unlock_user**](docs/UsersApi.md#unlock_user) | **DELETE** /api/v3/users/{id}/lock | Unlock user
*UsersApi* | [**update_user**](docs/UsersApi.md#update_user) | **PATCH** /api/v3/users/{id} | Update user
*UsersApi* | [**user_update_form**](docs/UsersApi.md#user_update_form) | **POST** /api/v3/users/{id}/form | User update form
*UsersApi* | [**view_user**](docs/UsersApi.md#view_user) | **GET** /api/v3/users/{id} | View user
*UsersApi* | [**view_user_schema**](docs/UsersApi.md#view_user_schema) | **GET** /api/v3/users/schema | View user schema
*ValuesPropertyApi* | [**view_notification_detail**](docs/ValuesPropertyApi.md#view_notification_detail) | **GET** /api/v3/notifications/{notification_id}/details/{id} | Get a notification detail
*ValuesPropertyApi* | [**view_values_schema**](docs/ValuesPropertyApi.md#view_values_schema) | **GET** /api/v3/values/schema/{id} | View Values schema
*VersionsApi* | [**available_projects_for_versions**](docs/VersionsApi.md#available_projects_for_versions) | **GET** /api/v3/versions/available_projects | Available projects for versions
*VersionsApi* | [**create_version**](docs/VersionsApi.md#create_version) | **POST** /api/v3/versions | Create version
*VersionsApi* | [**delete_version**](docs/VersionsApi.md#delete_version) | **DELETE** /api/v3/versions/{id} | Delete version
*VersionsApi* | [**list_versions**](docs/VersionsApi.md#list_versions) | **GET** /api/v3/versions | List versions
*VersionsApi* | [**list_versions_available_in_a_project**](docs/VersionsApi.md#list_versions_available_in_a_project) | **GET** /api/v3/projects/{id}/versions | List versions available in a project
*VersionsApi* | [**update_version**](docs/VersionsApi.md#update_version) | **PATCH** /api/v3/versions/{id} | Update Version
*VersionsApi* | [**version_create_form**](docs/VersionsApi.md#version_create_form) | **POST** /api/v3/versions/form | Version create form
*VersionsApi* | [**version_update_form**](docs/VersionsApi.md#version_update_form) | **POST** /api/v3/versions/{id}/form | Version update form
*VersionsApi* | [**view_version**](docs/VersionsApi.md#view_version) | **GET** /api/v3/versions/{id} | View version
*VersionsApi* | [**view_version_schema**](docs/VersionsApi.md#view_version_schema) | **GET** /api/v3/versions/schema | View version schema
*ViewsApi* | [**create_views**](docs/ViewsApi.md#create_views) | **POST** /api/v3/views/{id} | Create view
*ViewsApi* | [**list_views**](docs/ViewsApi.md#list_views) | **GET** /api/v3/views | List views
*ViewsApi* | [**view_view**](docs/ViewsApi.md#view_view) | **GET** /api/v3/views/{id} | View view
*WikiPagesApi* | [**view_wiki_page**](docs/WikiPagesApi.md#view_wiki_page) | **GET** /api/v3/wiki_pages/{id} | View Wiki Page
*WorkPackagesApi* | [**add_watcher**](docs/WorkPackagesApi.md#add_watcher) | **POST** /api/v3/work_packages/{id}/watchers | Add watcher
*WorkPackagesApi* | [**available_projects_for_work_package**](docs/WorkPackagesApi.md#available_projects_for_work_package) | **GET** /api/v3/work_packages/{id}/available_projects | Available projects for work package
*WorkPackagesApi* | [**available_responsibles**](docs/WorkPackagesApi.md#available_responsibles) | **GET** /api/v3/projects/{id}/available_responsibles | Available responsibles
*WorkPackagesApi* | [**available_watchers**](docs/WorkPackagesApi.md#available_watchers) | **GET** /api/v3/work_packages/{id}/available_watchers | Available watchers
*WorkPackagesApi* | [**comment_work_package**](docs/WorkPackagesApi.md#comment_work_package) | **POST** /api/v3/work_packages/{id}/activities | Comment work package
*WorkPackagesApi* | [**create_project_work_package**](docs/WorkPackagesApi.md#create_project_work_package) | **POST** /api/v3/projects/{id}/work_packages | Create work package in project
*WorkPackagesApi* | [**create_relation**](docs/WorkPackagesApi.md#create_relation) | **POST** /api/v3/work_packages/{id}/relations | Create Relation
*WorkPackagesApi* | [**create_work_package**](docs/WorkPackagesApi.md#create_work_package) | **POST** /api/v3/work_packages | Create Work Package
*WorkPackagesApi* | [**create_work_package_file_link**](docs/WorkPackagesApi.md#create_work_package_file_link) | **POST** /api/v3/work_packages/{id}/file_links | Creates file links.
*WorkPackagesApi* | [**delete_work_package**](docs/WorkPackagesApi.md#delete_work_package) | **DELETE** /api/v3/work_packages/{id} | Delete Work Package
*WorkPackagesApi* | [**get_project_work_package_collection**](docs/WorkPackagesApi.md#get_project_work_package_collection) | **GET** /api/v3/projects/{id}/work_packages | Get work packages of project
*WorkPackagesApi* | [**list_available_relation_candidates**](docs/WorkPackagesApi.md#list_available_relation_candidates) | **GET** /api/v3/work_packages/{id}/available_relation_candidates | Available relation candidates
*WorkPackagesApi* | [**list_relations**](docs/WorkPackagesApi.md#list_relations) | **GET** /api/v3/work_packages/{id}/relations | List relations
*WorkPackagesApi* | [**list_watchers**](docs/WorkPackagesApi.md#list_watchers) | **GET** /api/v3/work_packages/{id}/watchers | List watchers
*WorkPackagesApi* | [**list_work_package_activities**](docs/WorkPackagesApi.md#list_work_package_activities) | **GET** /api/v3/work_packages/{id}/activities | List work package activities
*WorkPackagesApi* | [**list_work_package_file_links**](docs/WorkPackagesApi.md#list_work_package_file_links) | **GET** /api/v3/work_packages/{id}/file_links | Gets all file links of a work package
*WorkPackagesApi* | [**list_work_package_schemas**](docs/WorkPackagesApi.md#list_work_package_schemas) | **GET** /api/v3/work_packages/schemas | List Work Package Schemas
*WorkPackagesApi* | [**list_work_packages**](docs/WorkPackagesApi.md#list_work_packages) | **GET** /api/v3/work_packages | List work packages
*WorkPackagesApi* | [**project_available_assignees**](docs/WorkPackagesApi.md#project_available_assignees) | **GET** /api/v3/projects/{id}/available_assignees | Project Available assignees
*WorkPackagesApi* | [**remove_watcher**](docs/WorkPackagesApi.md#remove_watcher) | **DELETE** /api/v3/work_packages/{id}/watchers/{user_id} | Remove watcher
*WorkPackagesApi* | [**revisions**](docs/WorkPackagesApi.md#revisions) | **GET** /api/v3/work_packages/{id}/revisions | Revisions
*WorkPackagesApi* | [**update_work_package**](docs/WorkPackagesApi.md#update_work_package) | **PATCH** /api/v3/work_packages/{id} | Update a Work Package
*WorkPackagesApi* | [**view_work_package**](docs/WorkPackagesApi.md#view_work_package) | **GET** /api/v3/work_packages/{id} | View Work Package
*WorkPackagesApi* | [**view_work_package_schema**](docs/WorkPackagesApi.md#view_work_package_schema) | **GET** /api/v3/work_packages/schemas/{identifier} | View Work Package Schema
*WorkPackagesApi* | [**work_package_available_assignees**](docs/WorkPackagesApi.md#work_package_available_assignees) | **GET** /api/v3/work_packages/{id}/available_assignees | Work Package Available assignees
*WorkPackagesApi* | [**work_package_create_form**](docs/WorkPackagesApi.md#work_package_create_form) | **POST** /api/v3/work_packages/form | Work Package Create Form
*WorkPackagesApi* | [**work_package_create_form_for_project**](docs/WorkPackagesApi.md#work_package_create_form_for_project) | **POST** /api/v3/projects/{id}/work_packages/form | Work Package Create Form For Project
*WorkPackagesApi* | [**work_package_edit_form**](docs/WorkPackagesApi.md#work_package_edit_form) | **POST** /api/v3/work_packages/{id}/form | Work Package Edit Form
*WorkScheduleApi* | [**create_non_working_day**](docs/WorkScheduleApi.md#create_non_working_day) | **POST** /api/v3/days/non_working | Creates a non-working day (NOT IMPLEMENTED)
*WorkScheduleApi* | [**delete_non_working_day**](docs/WorkScheduleApi.md#delete_non_working_day) | **DELETE** /api/v3/days/non_working/{date} | Removes a non-working day (NOT IMPLEMENTED)
*WorkScheduleApi* | [**list_days**](docs/WorkScheduleApi.md#list_days) | **GET** /api/v3/days | Lists days
*WorkScheduleApi* | [**list_non_working_days**](docs/WorkScheduleApi.md#list_non_working_days) | **GET** /api/v3/days/non_working | Lists all non working days
*WorkScheduleApi* | [**list_week_days**](docs/WorkScheduleApi.md#list_week_days) | **GET** /api/v3/days/week | Lists week days
*WorkScheduleApi* | [**update_non_working_day**](docs/WorkScheduleApi.md#update_non_working_day) | **PATCH** /api/v3/days/non_working/{date} | Update a non-working day attributes (NOT IMPLEMENTED)
*WorkScheduleApi* | [**update_week_day**](docs/WorkScheduleApi.md#update_week_day) | **PATCH** /api/v3/days/week/{day} | Update a week day attributes (NOT IMPLEMENTED)
*WorkScheduleApi* | [**update_week_days**](docs/WorkScheduleApi.md#update_week_days) | **PATCH** /api/v3/days/week | Update week days (NOT IMPLEMENTED)
*WorkScheduleApi* | [**view_day**](docs/WorkScheduleApi.md#view_day) | **GET** /api/v3/days/{date} | View day
*WorkScheduleApi* | [**view_non_working_day**](docs/WorkScheduleApi.md#view_non_working_day) | **GET** /api/v3/days/non_working/{date} | View a non-working day
*WorkScheduleApi* | [**view_week_day**](docs/WorkScheduleApi.md#view_week_day) | **GET** /api/v3/days/week/{day} | View a week day


## Documentation For Models

 - [ActivityModel](docs/ActivityModel.md)
 - [ActivityModelComment](docs/ActivityModelComment.md)
 - [AddWatcherRequest](docs/AddWatcherRequest.md)
 - [AttachmentModel](docs/AttachmentModel.md)
 - [AttachmentModelDescription](docs/AttachmentModelDescription.md)
 - [AttachmentModelLinks](docs/AttachmentModelLinks.md)
 - [AttachmentModelLinksAuthor](docs/AttachmentModelLinksAuthor.md)
 - [AttachmentModelLinksContainer](docs/AttachmentModelLinksContainer.md)
 - [AttachmentModelLinksDelete](docs/AttachmentModelLinksDelete.md)
 - [AttachmentModelLinksDownloadLocation](docs/AttachmentModelLinksDownloadLocation.md)
 - [AttachmentModelLinksSelf](docs/AttachmentModelLinksSelf.md)
 - [AttachmentsModel](docs/AttachmentsModel.md)
 - [AttachmentsModelAllOfEmbedded](docs/AttachmentsModelAllOfEmbedded.md)
 - [AttachmentsModelAllOfEmbeddedElementsInner](docs/AttachmentsModelAllOfEmbeddedElementsInner.md)
 - [AttachmentsModelAllOfLinks](docs/AttachmentsModelAllOfLinks.md)
 - [AttachmentsModelAllOfLinksSelf](docs/AttachmentsModelAllOfLinksSelf.md)
 - [BudgetModel](docs/BudgetModel.md)
 - [BudgetModelLinks](docs/BudgetModelLinks.md)
 - [BudgetModelLinksSelf](docs/BudgetModelLinksSelf.md)
 - [CategoryModel](docs/CategoryModel.md)
 - [CategoryModelLinks](docs/CategoryModelLinks.md)
 - [CategoryModelLinksDefaultAssignee](docs/CategoryModelLinksDefaultAssignee.md)
 - [CategoryModelLinksProject](docs/CategoryModelLinksProject.md)
 - [CategoryModelLinksSelf](docs/CategoryModelLinksSelf.md)
 - [CollectionModel](docs/CollectionModel.md)
 - [CollectionModelLinks](docs/CollectionModelLinks.md)
 - [CollectionModelLinksSelf](docs/CollectionModelLinksSelf.md)
 - [CommentWorkPackageRequest](docs/CommentWorkPackageRequest.md)
 - [ConfigurationModel](docs/ConfigurationModel.md)
 - [CreateViewsRequest](docs/CreateViewsRequest.md)
 - [CreateViewsRequestLinks](docs/CreateViewsRequestLinks.md)
 - [CreateViewsRequestLinksQuery](docs/CreateViewsRequestLinksQuery.md)
 - [CustomActionModel](docs/CustomActionModel.md)
 - [CustomActionModelLinks](docs/CustomActionModelLinks.md)
 - [CustomActionModelLinksExecuteImmediately](docs/CustomActionModelLinksExecuteImmediately.md)
 - [CustomActionModelLinksSelf](docs/CustomActionModelLinksSelf.md)
 - [CustomOptionModel](docs/CustomOptionModel.md)
 - [CustomOptionModelLinks](docs/CustomOptionModelLinks.md)
 - [CustomOptionModelLinksSelf](docs/CustomOptionModelLinksSelf.md)
 - [DayCollectionModel](docs/DayCollectionModel.md)
 - [DayCollectionModelAllOfEmbedded](docs/DayCollectionModelAllOfEmbedded.md)
 - [DayCollectionModelAllOfLinks](docs/DayCollectionModelAllOfLinks.md)
 - [DayCollectionModelAllOfLinksSelf](docs/DayCollectionModelAllOfLinksSelf.md)
 - [DayModel](docs/DayModel.md)
 - [DayModelLinks](docs/DayModelLinks.md)
 - [DayModelLinksWeekDay](docs/DayModelLinksWeekDay.md)
 - [DocumentModel](docs/DocumentModel.md)
 - [DocumentModelLinks](docs/DocumentModelLinks.md)
 - [DocumentModelLinksAttachments](docs/DocumentModelLinksAttachments.md)
 - [DocumentModelLinksProject](docs/DocumentModelLinksProject.md)
 - [DocumentModelLinksSelf](docs/DocumentModelLinksSelf.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ErrorResponseEmbedded](docs/ErrorResponseEmbedded.md)
 - [ErrorResponseEmbeddedDetails](docs/ErrorResponseEmbeddedDetails.md)
 - [ExecuteCustomActionRequest](docs/ExecuteCustomActionRequest.md)
 - [ExecuteCustomActionRequestLinks](docs/ExecuteCustomActionRequestLinks.md)
 - [ExecuteCustomActionRequestLinksWorkPackage](docs/ExecuteCustomActionRequestLinksWorkPackage.md)
 - [FileLinkCollectionReadModel](docs/FileLinkCollectionReadModel.md)
 - [FileLinkCollectionReadModelAllOfEmbedded](docs/FileLinkCollectionReadModelAllOfEmbedded.md)
 - [FileLinkCollectionReadModelAllOfLinks](docs/FileLinkCollectionReadModelAllOfLinks.md)
 - [FileLinkCollectionReadModelAllOfLinksSelf](docs/FileLinkCollectionReadModelAllOfLinksSelf.md)
 - [FileLinkCollectionWriteModel](docs/FileLinkCollectionWriteModel.md)
 - [FileLinkCollectionWriteModelAllOfEmbedded](docs/FileLinkCollectionWriteModelAllOfEmbedded.md)
 - [FileLinkOriginDataModel](docs/FileLinkOriginDataModel.md)
 - [FileLinkReadModel](docs/FileLinkReadModel.md)
 - [FileLinkReadModelEmbedded](docs/FileLinkReadModelEmbedded.md)
 - [FileLinkReadModelLinks](docs/FileLinkReadModelLinks.md)
 - [FileLinkReadModelLinksContainer](docs/FileLinkReadModelLinksContainer.md)
 - [FileLinkReadModelLinksCreator](docs/FileLinkReadModelLinksCreator.md)
 - [FileLinkReadModelLinksDelete](docs/FileLinkReadModelLinksDelete.md)
 - [FileLinkReadModelLinksOriginOpen](docs/FileLinkReadModelLinksOriginOpen.md)
 - [FileLinkReadModelLinksOriginOpenLocation](docs/FileLinkReadModelLinksOriginOpenLocation.md)
 - [FileLinkReadModelLinksSelf](docs/FileLinkReadModelLinksSelf.md)
 - [FileLinkReadModelLinksStaticOriginDownload](docs/FileLinkReadModelLinksStaticOriginDownload.md)
 - [FileLinkReadModelLinksStaticOriginOpen](docs/FileLinkReadModelLinksStaticOriginOpen.md)
 - [FileLinkReadModelLinksStaticOriginOpenLocation](docs/FileLinkReadModelLinksStaticOriginOpenLocation.md)
 - [FileLinkReadModelLinksStatus](docs/FileLinkReadModelLinksStatus.md)
 - [FileLinkReadModelLinksStorage](docs/FileLinkReadModelLinksStorage.md)
 - [FileLinkWriteModel](docs/FileLinkWriteModel.md)
 - [FileLinkWriteModelLinks](docs/FileLinkWriteModelLinks.md)
 - [FileLinkWriteModelLinksOneOf](docs/FileLinkWriteModelLinksOneOf.md)
 - [FileLinkWriteModelLinksOneOf1](docs/FileLinkWriteModelLinksOneOf1.md)
 - [FileLinkWriteModelLinksOneOf1StorageUrl](docs/FileLinkWriteModelLinksOneOf1StorageUrl.md)
 - [Formattable](docs/Formattable.md)
 - [GridCollectionModel](docs/GridCollectionModel.md)
 - [GridCollectionModelAllOfEmbedded](docs/GridCollectionModelAllOfEmbedded.md)
 - [GridReadModel](docs/GridReadModel.md)
 - [GridReadModelLinks](docs/GridReadModelLinks.md)
 - [GridReadModelLinksAddAttachment](docs/GridReadModelLinksAddAttachment.md)
 - [GridReadModelLinksAttachments](docs/GridReadModelLinksAttachments.md)
 - [GridReadModelLinksDelete](docs/GridReadModelLinksDelete.md)
 - [GridReadModelLinksScope](docs/GridReadModelLinksScope.md)
 - [GridReadModelLinksSelf](docs/GridReadModelLinksSelf.md)
 - [GridReadModelLinksUpdate](docs/GridReadModelLinksUpdate.md)
 - [GridReadModelLinksUpdateImmediately](docs/GridReadModelLinksUpdateImmediately.md)
 - [GridWidgetModel](docs/GridWidgetModel.md)
 - [GridWriteModel](docs/GridWriteModel.md)
 - [GridWriteModelLinks](docs/GridWriteModelLinks.md)
 - [GroupCollectionModel](docs/GroupCollectionModel.md)
 - [GroupCollectionModelAllOfEmbedded](docs/GroupCollectionModelAllOfEmbedded.md)
 - [GroupCollectionModelAllOfLinks](docs/GroupCollectionModelAllOfLinks.md)
 - [GroupCollectionModelAllOfLinksSelf](docs/GroupCollectionModelAllOfLinksSelf.md)
 - [GroupWriteModel](docs/GroupWriteModel.md)
 - [GroupWriteModelLinks](docs/GroupWriteModelLinks.md)
 - [GroupWriteModelLinksMembersInner](docs/GroupWriteModelLinksMembersInner.md)
 - [HelpTextCollectionModel](docs/HelpTextCollectionModel.md)
 - [HelpTextCollectionModelAllOfEmbedded](docs/HelpTextCollectionModelAllOfEmbedded.md)
 - [HelpTextCollectionModelAllOfLinks](docs/HelpTextCollectionModelAllOfLinks.md)
 - [HelpTextCollectionModelAllOfLinksSelf](docs/HelpTextCollectionModelAllOfLinksSelf.md)
 - [HelpTextModel](docs/HelpTextModel.md)
 - [HelpTextModelLinks](docs/HelpTextModelLinks.md)
 - [HelpTextModelLinksAddAttachment](docs/HelpTextModelLinksAddAttachment.md)
 - [HelpTextModelLinksAttachments](docs/HelpTextModelLinksAttachments.md)
 - [HelpTextModelLinksEditText](docs/HelpTextModelLinksEditText.md)
 - [HelpTextModelLinksSelf](docs/HelpTextModelLinksSelf.md)
 - [Link](docs/Link.md)
 - [MeetingModel](docs/MeetingModel.md)
 - [MeetingModelLinks](docs/MeetingModelLinks.md)
 - [MeetingModelLinksAddAttachment](docs/MeetingModelLinksAddAttachment.md)
 - [MeetingModelLinksAttachments](docs/MeetingModelLinksAttachments.md)
 - [MeetingModelLinksAuthor](docs/MeetingModelLinksAuthor.md)
 - [MeetingModelLinksProject](docs/MeetingModelLinksProject.md)
 - [MeetingModelLinksSelf](docs/MeetingModelLinksSelf.md)
 - [MembershipCollectionModel](docs/MembershipCollectionModel.md)
 - [MembershipCollectionModelAllOfEmbedded](docs/MembershipCollectionModelAllOfEmbedded.md)
 - [MembershipFormModel](docs/MembershipFormModel.md)
 - [MembershipFormModelEmbedded](docs/MembershipFormModelEmbedded.md)
 - [MembershipFormModelEmbeddedValidationError](docs/MembershipFormModelEmbeddedValidationError.md)
 - [MembershipFormModelLinks](docs/MembershipFormModelLinks.md)
 - [MembershipFormModelLinksCommit](docs/MembershipFormModelLinksCommit.md)
 - [MembershipFormModelLinksSelf](docs/MembershipFormModelLinksSelf.md)
 - [MembershipFormModelLinksValidateInner](docs/MembershipFormModelLinksValidateInner.md)
 - [MembershipReadModel](docs/MembershipReadModel.md)
 - [MembershipReadModelEmbedded](docs/MembershipReadModelEmbedded.md)
 - [MembershipReadModelEmbeddedPrincipal](docs/MembershipReadModelEmbeddedPrincipal.md)
 - [MembershipReadModelLinks](docs/MembershipReadModelLinks.md)
 - [MembershipReadModelLinksPrincipal](docs/MembershipReadModelLinksPrincipal.md)
 - [MembershipReadModelLinksProject](docs/MembershipReadModelLinksProject.md)
 - [MembershipReadModelLinksRolesInner](docs/MembershipReadModelLinksRolesInner.md)
 - [MembershipReadModelLinksSchema](docs/MembershipReadModelLinksSchema.md)
 - [MembershipReadModelLinksSelf](docs/MembershipReadModelLinksSelf.md)
 - [MembershipReadModelLinksUpdate](docs/MembershipReadModelLinksUpdate.md)
 - [MembershipReadModelLinksUpdateImmediately](docs/MembershipReadModelLinksUpdateImmediately.md)
 - [MembershipSchemaModel](docs/MembershipSchemaModel.md)
 - [MembershipWriteModel](docs/MembershipWriteModel.md)
 - [MembershipWriteModelLinks](docs/MembershipWriteModelLinks.md)
 - [MembershipWriteModelLinksPrincipal](docs/MembershipWriteModelLinksPrincipal.md)
 - [MembershipWriteModelLinksProject](docs/MembershipWriteModelLinksProject.md)
 - [MembershipWriteModelMeta](docs/MembershipWriteModelMeta.md)
 - [MembershipWriteModelMetaNotificationMessage](docs/MembershipWriteModelMetaNotificationMessage.md)
 - [NewsModel](docs/NewsModel.md)
 - [NewsModelLinks](docs/NewsModelLinks.md)
 - [NewsModelLinksAuthor](docs/NewsModelLinksAuthor.md)
 - [NewsModelLinksProject](docs/NewsModelLinksProject.md)
 - [NewsModelLinksSelf](docs/NewsModelLinksSelf.md)
 - [NonWorkingDayCollectionModel](docs/NonWorkingDayCollectionModel.md)
 - [NonWorkingDayCollectionModelAllOfEmbedded](docs/NonWorkingDayCollectionModelAllOfEmbedded.md)
 - [NonWorkingDayCollectionModelAllOfLinks](docs/NonWorkingDayCollectionModelAllOfLinks.md)
 - [NonWorkingDayCollectionModelAllOfLinksSelf](docs/NonWorkingDayCollectionModelAllOfLinksSelf.md)
 - [NonWorkingDayModel](docs/NonWorkingDayModel.md)
 - [NonWorkingDayModelLinks](docs/NonWorkingDayModelLinks.md)
 - [NonWorkingDayModelLinksSelf](docs/NonWorkingDayModelLinksSelf.md)
 - [NotificationCollectionModel](docs/NotificationCollectionModel.md)
 - [NotificationCollectionModelAllOfEmbedded](docs/NotificationCollectionModelAllOfEmbedded.md)
 - [NotificationCollectionModelAllOfLinks](docs/NotificationCollectionModelAllOfLinks.md)
 - [NotificationCollectionModelAllOfLinksChangeSize](docs/NotificationCollectionModelAllOfLinksChangeSize.md)
 - [NotificationCollectionModelAllOfLinksJumpTo](docs/NotificationCollectionModelAllOfLinksJumpTo.md)
 - [NotificationCollectionModelAllOfLinksSelf](docs/NotificationCollectionModelAllOfLinksSelf.md)
 - [NotificationModel](docs/NotificationModel.md)
 - [NotificationModelDetailsInner](docs/NotificationModelDetailsInner.md)
 - [NotificationModelEmbedded](docs/NotificationModelEmbedded.md)
 - [NotificationModelEmbeddedResource](docs/NotificationModelEmbeddedResource.md)
 - [NotificationModelLinks](docs/NotificationModelLinks.md)
 - [NotificationModelLinksActivity](docs/NotificationModelLinksActivity.md)
 - [NotificationModelLinksActor](docs/NotificationModelLinksActor.md)
 - [NotificationModelLinksDetailsInner](docs/NotificationModelLinksDetailsInner.md)
 - [NotificationModelLinksProject](docs/NotificationModelLinksProject.md)
 - [NotificationModelLinksReadIan](docs/NotificationModelLinksReadIan.md)
 - [NotificationModelLinksResource](docs/NotificationModelLinksResource.md)
 - [NotificationModelLinksSelf](docs/NotificationModelLinksSelf.md)
 - [NotificationModelLinksUnreadIan](docs/NotificationModelLinksUnreadIan.md)
 - [OAuthApplicationReadModel](docs/OAuthApplicationReadModel.md)
 - [OAuthApplicationReadModelLinks](docs/OAuthApplicationReadModelLinks.md)
 - [OAuthApplicationReadModelLinksIntegration](docs/OAuthApplicationReadModelLinksIntegration.md)
 - [OAuthApplicationReadModelLinksOwner](docs/OAuthApplicationReadModelLinksOwner.md)
 - [OAuthApplicationReadModelLinksRedirectUri](docs/OAuthApplicationReadModelLinksRedirectUri.md)
 - [OAuthApplicationReadModelLinksSelf](docs/OAuthApplicationReadModelLinksSelf.md)
 - [OAuthClientCredentialsReadModel](docs/OAuthClientCredentialsReadModel.md)
 - [OAuthClientCredentialsReadModelLinks](docs/OAuthClientCredentialsReadModelLinks.md)
 - [OAuthClientCredentialsReadModelLinksIntegration](docs/OAuthClientCredentialsReadModelLinksIntegration.md)
 - [OAuthClientCredentialsReadModelLinksSelf](docs/OAuthClientCredentialsReadModelLinksSelf.md)
 - [OAuthClientCredentialsWriteModel](docs/OAuthClientCredentialsWriteModel.md)
 - [PaginatedCollectionModel](docs/PaginatedCollectionModel.md)
 - [PaginatedCollectionModelAllOfLinks](docs/PaginatedCollectionModelAllOfLinks.md)
 - [PaginatedCollectionModelAllOfLinksChangeSize](docs/PaginatedCollectionModelAllOfLinksChangeSize.md)
 - [PaginatedCollectionModelAllOfLinksJumpTo](docs/PaginatedCollectionModelAllOfLinksJumpTo.md)
 - [PlaceholderUserCollectionModel](docs/PlaceholderUserCollectionModel.md)
 - [PlaceholderUserCollectionModelAllOfEmbedded](docs/PlaceholderUserCollectionModelAllOfEmbedded.md)
 - [PlaceholderUserCollectionModelAllOfLinks](docs/PlaceholderUserCollectionModelAllOfLinks.md)
 - [PlaceholderUserCollectionModelAllOfLinksSelf](docs/PlaceholderUserCollectionModelAllOfLinksSelf.md)
 - [PlaceholderUserCreateModel](docs/PlaceholderUserCreateModel.md)
 - [PlaceholderUserModel](docs/PlaceholderUserModel.md)
 - [PlaceholderUserModelAllOfLinks](docs/PlaceholderUserModelAllOfLinks.md)
 - [PlaceholderUserModelAllOfLinksDelete](docs/PlaceholderUserModelAllOfLinksDelete.md)
 - [PlaceholderUserModelAllOfLinksShowUser](docs/PlaceholderUserModelAllOfLinksShowUser.md)
 - [PlaceholderUserModelAllOfLinksUpdateImmediately](docs/PlaceholderUserModelAllOfLinksUpdateImmediately.md)
 - [PostModel](docs/PostModel.md)
 - [PostModelLinks](docs/PostModelLinks.md)
 - [PostModelLinksAddAttachment](docs/PostModelLinksAddAttachment.md)
 - [PrincipalCollectionModel](docs/PrincipalCollectionModel.md)
 - [PrincipalCollectionModelAllOfEmbedded](docs/PrincipalCollectionModelAllOfEmbedded.md)
 - [PrincipalCollectionModelAllOfEmbeddedElementsInner](docs/PrincipalCollectionModelAllOfEmbeddedElementsInner.md)
 - [PrincipalCollectionModelAllOfLinks](docs/PrincipalCollectionModelAllOfLinks.md)
 - [PrincipalCollectionModelAllOfLinksSelf](docs/PrincipalCollectionModelAllOfLinksSelf.md)
 - [PrincipalModel](docs/PrincipalModel.md)
 - [PrincipalModelLinks](docs/PrincipalModelLinks.md)
 - [PrincipalModelLinksMemberships](docs/PrincipalModelLinksMemberships.md)
 - [PrincipalModelLinksSelf](docs/PrincipalModelLinksSelf.md)
 - [PriorityModel](docs/PriorityModel.md)
 - [PriorityModelLinks](docs/PriorityModelLinks.md)
 - [PriorityModelLinksSelf](docs/PriorityModelLinksSelf.md)
 - [ProjectCollectionModel](docs/ProjectCollectionModel.md)
 - [ProjectCollectionModelAllOfEmbedded](docs/ProjectCollectionModelAllOfEmbedded.md)
 - [ProjectCollectionModelAllOfLinks](docs/ProjectCollectionModelAllOfLinks.md)
 - [ProjectCollectionModelAllOfLinksRepresentationsInner](docs/ProjectCollectionModelAllOfLinksRepresentationsInner.md)
 - [ProjectCollectionModelAllOfLinksSelf](docs/ProjectCollectionModelAllOfLinksSelf.md)
 - [ProjectModel](docs/ProjectModel.md)
 - [ProjectModelLinks](docs/ProjectModelLinks.md)
 - [ProjectModelLinksAncestorsInner](docs/ProjectModelLinksAncestorsInner.md)
 - [ProjectModelLinksCategories](docs/ProjectModelLinksCategories.md)
 - [ProjectModelLinksCreateWorkPackage](docs/ProjectModelLinksCreateWorkPackage.md)
 - [ProjectModelLinksCreateWorkPackageImmediately](docs/ProjectModelLinksCreateWorkPackageImmediately.md)
 - [ProjectModelLinksDelete](docs/ProjectModelLinksDelete.md)
 - [ProjectModelLinksMemberships](docs/ProjectModelLinksMemberships.md)
 - [ProjectModelLinksParent](docs/ProjectModelLinksParent.md)
 - [ProjectModelLinksProjectStorages](docs/ProjectModelLinksProjectStorages.md)
 - [ProjectModelLinksSelf](docs/ProjectModelLinksSelf.md)
 - [ProjectModelLinksStatus](docs/ProjectModelLinksStatus.md)
 - [ProjectModelLinksStoragesInner](docs/ProjectModelLinksStoragesInner.md)
 - [ProjectModelLinksTypes](docs/ProjectModelLinksTypes.md)
 - [ProjectModelLinksUpdate](docs/ProjectModelLinksUpdate.md)
 - [ProjectModelLinksUpdateImmediately](docs/ProjectModelLinksUpdateImmediately.md)
 - [ProjectModelLinksVersions](docs/ProjectModelLinksVersions.md)
 - [ProjectModelLinksWorkPackages](docs/ProjectModelLinksWorkPackages.md)
 - [ProjectModelStatusExplanation](docs/ProjectModelStatusExplanation.md)
 - [ProjectStorageCollectionModel](docs/ProjectStorageCollectionModel.md)
 - [ProjectStorageCollectionModelAllOfEmbedded](docs/ProjectStorageCollectionModelAllOfEmbedded.md)
 - [ProjectStorageCollectionModelAllOfLinks](docs/ProjectStorageCollectionModelAllOfLinks.md)
 - [ProjectStorageCollectionModelAllOfLinksSelf](docs/ProjectStorageCollectionModelAllOfLinksSelf.md)
 - [ProjectStorageModel](docs/ProjectStorageModel.md)
 - [ProjectStorageModelLinks](docs/ProjectStorageModelLinks.md)
 - [ProjectStorageModelLinksCreator](docs/ProjectStorageModelLinksCreator.md)
 - [ProjectStorageModelLinksOpen](docs/ProjectStorageModelLinksOpen.md)
 - [ProjectStorageModelLinksOpenWithConnectionEnsured](docs/ProjectStorageModelLinksOpenWithConnectionEnsured.md)
 - [ProjectStorageModelLinksProject](docs/ProjectStorageModelLinksProject.md)
 - [ProjectStorageModelLinksProjectFolder](docs/ProjectStorageModelLinksProjectFolder.md)
 - [ProjectStorageModelLinksSelf](docs/ProjectStorageModelLinksSelf.md)
 - [ProjectStorageModelLinksStorage](docs/ProjectStorageModelLinksStorage.md)
 - [QueryColumnModel](docs/QueryColumnModel.md)
 - [QueryCreateForm](docs/QueryCreateForm.md)
 - [QueryFilterInstanceSchemaModel](docs/QueryFilterInstanceSchemaModel.md)
 - [QueryFilterInstanceSchemaModelLinks](docs/QueryFilterInstanceSchemaModelLinks.md)
 - [QueryFilterInstanceSchemaModelLinksFilter](docs/QueryFilterInstanceSchemaModelLinksFilter.md)
 - [QueryFilterInstanceSchemaModelLinksSelf](docs/QueryFilterInstanceSchemaModelLinksSelf.md)
 - [QueryFilterModel](docs/QueryFilterModel.md)
 - [QueryOperatorModel](docs/QueryOperatorModel.md)
 - [QuerySortByModel](docs/QuerySortByModel.md)
 - [QueryUpdateForm](docs/QueryUpdateForm.md)
 - [RelationModel](docs/RelationModel.md)
 - [RelationModelLinks](docs/RelationModelLinks.md)
 - [RelationModelLinksDelete](docs/RelationModelLinksDelete.md)
 - [RelationModelLinksFrom](docs/RelationModelLinksFrom.md)
 - [RelationModelLinksSchema](docs/RelationModelLinksSchema.md)
 - [RelationModelLinksSelf](docs/RelationModelLinksSelf.md)
 - [RelationModelLinksTo](docs/RelationModelLinksTo.md)
 - [RelationModelLinksUpdate](docs/RelationModelLinksUpdate.md)
 - [RelationModelLinksUpdateImmediately](docs/RelationModelLinksUpdateImmediately.md)
 - [RevisionModel](docs/RevisionModel.md)
 - [RevisionModelLinks](docs/RevisionModelLinks.md)
 - [RevisionModelLinksAuthor](docs/RevisionModelLinksAuthor.md)
 - [RevisionModelLinksProject](docs/RevisionModelLinksProject.md)
 - [RevisionModelLinksSelf](docs/RevisionModelLinksSelf.md)
 - [RevisionModelLinksShowRevision](docs/RevisionModelLinksShowRevision.md)
 - [RevisionModelMessage](docs/RevisionModelMessage.md)
 - [RoleModel](docs/RoleModel.md)
 - [RoleModelLinks](docs/RoleModelLinks.md)
 - [RoleModelLinksSelf](docs/RoleModelLinksSelf.md)
 - [RootModel](docs/RootModel.md)
 - [RootModelLinks](docs/RootModelLinks.md)
 - [RootModelLinksConfiguration](docs/RootModelLinksConfiguration.md)
 - [RootModelLinksMemberships](docs/RootModelLinksMemberships.md)
 - [RootModelLinksPriorities](docs/RootModelLinksPriorities.md)
 - [RootModelLinksRelations](docs/RootModelLinksRelations.md)
 - [RootModelLinksSelf](docs/RootModelLinksSelf.md)
 - [RootModelLinksStatuses](docs/RootModelLinksStatuses.md)
 - [RootModelLinksTimeEntries](docs/RootModelLinksTimeEntries.md)
 - [RootModelLinksTypes](docs/RootModelLinksTypes.md)
 - [RootModelLinksUser](docs/RootModelLinksUser.md)
 - [RootModelLinksUserPreferences](docs/RootModelLinksUserPreferences.md)
 - [RootModelLinksWorkPackages](docs/RootModelLinksWorkPackages.md)
 - [SchemaModel](docs/SchemaModel.md)
 - [SchemaModelLinks](docs/SchemaModelLinks.md)
 - [SchemaModelLinksSelf](docs/SchemaModelLinksSelf.md)
 - [SchemaPropertyModel](docs/SchemaPropertyModel.md)
 - [ShowOrValidateFormRequest](docs/ShowOrValidateFormRequest.md)
 - [StatusCollectionModel](docs/StatusCollectionModel.md)
 - [StatusCollectionModelAllOfEmbedded](docs/StatusCollectionModelAllOfEmbedded.md)
 - [StatusCollectionModelAllOfLinks](docs/StatusCollectionModelAllOfLinks.md)
 - [StatusCollectionModelAllOfLinksSelf](docs/StatusCollectionModelAllOfLinksSelf.md)
 - [StatusModel](docs/StatusModel.md)
 - [StatusModelLinks](docs/StatusModelLinks.md)
 - [StatusModelLinksSelf](docs/StatusModelLinksSelf.md)
 - [StorageCollectionModel](docs/StorageCollectionModel.md)
 - [StorageCollectionModelAllOfEmbedded](docs/StorageCollectionModelAllOfEmbedded.md)
 - [StorageCollectionModelAllOfLinks](docs/StorageCollectionModelAllOfLinks.md)
 - [StorageCollectionModelAllOfLinksSelf](docs/StorageCollectionModelAllOfLinksSelf.md)
 - [StorageFileModel](docs/StorageFileModel.md)
 - [StorageFileModelAllOfLinks](docs/StorageFileModelAllOfLinks.md)
 - [StorageFileModelAllOfLinksSelf](docs/StorageFileModelAllOfLinksSelf.md)
 - [StorageFileUploadLinkModel](docs/StorageFileUploadLinkModel.md)
 - [StorageFileUploadLinkModelLinks](docs/StorageFileUploadLinkModelLinks.md)
 - [StorageFileUploadLinkModelLinksDestination](docs/StorageFileUploadLinkModelLinksDestination.md)
 - [StorageFileUploadLinkModelLinksSelf](docs/StorageFileUploadLinkModelLinksSelf.md)
 - [StorageFileUploadPreparationModel](docs/StorageFileUploadPreparationModel.md)
 - [StorageFilesModel](docs/StorageFilesModel.md)
 - [StorageFilesModelParent](docs/StorageFilesModelParent.md)
 - [StorageReadModel](docs/StorageReadModel.md)
 - [StorageReadModelEmbedded](docs/StorageReadModelEmbedded.md)
 - [StorageReadModelLinks](docs/StorageReadModelLinks.md)
 - [StorageReadModelLinksAuthorizationState](docs/StorageReadModelLinksAuthorizationState.md)
 - [StorageReadModelLinksAuthorize](docs/StorageReadModelLinksAuthorize.md)
 - [StorageReadModelLinksOauthApplication](docs/StorageReadModelLinksOauthApplication.md)
 - [StorageReadModelLinksOauthClientCredentials](docs/StorageReadModelLinksOauthClientCredentials.md)
 - [StorageReadModelLinksOpen](docs/StorageReadModelLinksOpen.md)
 - [StorageReadModelLinksOrigin](docs/StorageReadModelLinksOrigin.md)
 - [StorageReadModelLinksSelf](docs/StorageReadModelLinksSelf.md)
 - [StorageReadModelLinksType](docs/StorageReadModelLinksType.md)
 - [StorageWriteModel](docs/StorageWriteModel.md)
 - [StorageWriteModelLinks](docs/StorageWriteModelLinks.md)
 - [StorageWriteModelLinksOrigin](docs/StorageWriteModelLinksOrigin.md)
 - [StorageWriteModelLinksType](docs/StorageWriteModelLinksType.md)
 - [TimeEntryActivityModel](docs/TimeEntryActivityModel.md)
 - [TimeEntryActivityModelEmbedded](docs/TimeEntryActivityModelEmbedded.md)
 - [TimeEntryActivityModelLinks](docs/TimeEntryActivityModelLinks.md)
 - [TimeEntryActivityModelLinksProjectsInner](docs/TimeEntryActivityModelLinksProjectsInner.md)
 - [TimeEntryActivityModelLinksSelf](docs/TimeEntryActivityModelLinksSelf.md)
 - [TimeEntryCollectionModel](docs/TimeEntryCollectionModel.md)
 - [TimeEntryCollectionModelAllOfEmbedded](docs/TimeEntryCollectionModelAllOfEmbedded.md)
 - [TimeEntryCollectionModelAllOfLinks](docs/TimeEntryCollectionModelAllOfLinks.md)
 - [TimeEntryCollectionModelAllOfLinksSelf](docs/TimeEntryCollectionModelAllOfLinksSelf.md)
 - [TimeEntryModel](docs/TimeEntryModel.md)
 - [TimeEntryModelComment](docs/TimeEntryModelComment.md)
 - [TimeEntryModelLinks](docs/TimeEntryModelLinks.md)
 - [TimeEntryModelLinksActivity](docs/TimeEntryModelLinksActivity.md)
 - [TimeEntryModelLinksDelete](docs/TimeEntryModelLinksDelete.md)
 - [TimeEntryModelLinksProject](docs/TimeEntryModelLinksProject.md)
 - [TimeEntryModelLinksSchema](docs/TimeEntryModelLinksSchema.md)
 - [TimeEntryModelLinksSelf](docs/TimeEntryModelLinksSelf.md)
 - [TimeEntryModelLinksUpdate](docs/TimeEntryModelLinksUpdate.md)
 - [TimeEntryModelLinksUpdateImmediately](docs/TimeEntryModelLinksUpdateImmediately.md)
 - [TimeEntryModelLinksUser](docs/TimeEntryModelLinksUser.md)
 - [TimeEntryModelLinksWorkPackage](docs/TimeEntryModelLinksWorkPackage.md)
 - [TypeModel](docs/TypeModel.md)
 - [TypeModelLinks](docs/TypeModelLinks.md)
 - [TypeModelLinksSelf](docs/TypeModelLinksSelf.md)
 - [UpdateActivityRequest](docs/UpdateActivityRequest.md)
 - [UpdateActivityRequestComment](docs/UpdateActivityRequestComment.md)
 - [UpdateUserPreferencesRequest](docs/UpdateUserPreferencesRequest.md)
 - [UserCollectionModel](docs/UserCollectionModel.md)
 - [UserCollectionModelAllOfEmbedded](docs/UserCollectionModelAllOfEmbedded.md)
 - [UserCollectionModelAllOfLinks](docs/UserCollectionModelAllOfLinks.md)
 - [UserCollectionModelAllOfLinksSelf](docs/UserCollectionModelAllOfLinksSelf.md)
 - [UserCreateModel](docs/UserCreateModel.md)
 - [UserModel](docs/UserModel.md)
 - [UserModelAllOfLinks](docs/UserModelAllOfLinks.md)
 - [UserModelAllOfLinksAuthSource](docs/UserModelAllOfLinksAuthSource.md)
 - [UserModelAllOfLinksDelete](docs/UserModelAllOfLinksDelete.md)
 - [UserModelAllOfLinksLock](docs/UserModelAllOfLinksLock.md)
 - [UserModelAllOfLinksShowUser](docs/UserModelAllOfLinksShowUser.md)
 - [UserModelAllOfLinksUnlock](docs/UserModelAllOfLinksUnlock.md)
 - [UserModelAllOfLinksUpdateImmediately](docs/UserModelAllOfLinksUpdateImmediately.md)
 - [ValuesPropertyModel](docs/ValuesPropertyModel.md)
 - [ValuesPropertyModelLinks](docs/ValuesPropertyModelLinks.md)
 - [ValuesPropertyModelLinksSchema](docs/ValuesPropertyModelLinksSchema.md)
 - [ValuesPropertyModelLinksSelf](docs/ValuesPropertyModelLinksSelf.md)
 - [VersionModel](docs/VersionModel.md)
 - [VersionModelLinks](docs/VersionModelLinks.md)
 - [VersionModelLinksAvailableInProjects](docs/VersionModelLinksAvailableInProjects.md)
 - [VersionModelLinksDefiningProject](docs/VersionModelLinksDefiningProject.md)
 - [VersionModelLinksSelf](docs/VersionModelLinksSelf.md)
 - [VersionModelLinksUpdate](docs/VersionModelLinksUpdate.md)
 - [VersionModelLinksUpdateImmediately](docs/VersionModelLinksUpdateImmediately.md)
 - [WatchersModel](docs/WatchersModel.md)
 - [WatchersModelAllOfEmbedded](docs/WatchersModelAllOfEmbedded.md)
 - [WatchersModelAllOfEmbeddedElementsInner](docs/WatchersModelAllOfEmbeddedElementsInner.md)
 - [WatchersModelAllOfLinks](docs/WatchersModelAllOfLinks.md)
 - [WatchersModelAllOfLinksSelf](docs/WatchersModelAllOfLinksSelf.md)
 - [WeekDayCollectionModel](docs/WeekDayCollectionModel.md)
 - [WeekDayCollectionModelAllOfEmbedded](docs/WeekDayCollectionModelAllOfEmbedded.md)
 - [WeekDayCollectionModelAllOfLinks](docs/WeekDayCollectionModelAllOfLinks.md)
 - [WeekDayCollectionModelAllOfLinksSelf](docs/WeekDayCollectionModelAllOfLinksSelf.md)
 - [WeekDayCollectionWriteModel](docs/WeekDayCollectionWriteModel.md)
 - [WeekDayCollectionWriteModelEmbedded](docs/WeekDayCollectionWriteModelEmbedded.md)
 - [WeekDayCollectionWriteModelEmbeddedElementsInner](docs/WeekDayCollectionWriteModelEmbeddedElementsInner.md)
 - [WeekDayModel](docs/WeekDayModel.md)
 - [WeekDaySelfLinkModel](docs/WeekDaySelfLinkModel.md)
 - [WeekDaySelfLinkModelSelf](docs/WeekDaySelfLinkModelSelf.md)
 - [WeekDayWriteModel](docs/WeekDayWriteModel.md)
 - [WikiPageModel](docs/WikiPageModel.md)
 - [WikiPageModelLinks](docs/WikiPageModelLinks.md)
 - [WikiPageModelLinksAddAttachment](docs/WikiPageModelLinksAddAttachment.md)
 - [WorkPackageModel](docs/WorkPackageModel.md)
 - [WorkPackageModelDescription](docs/WorkPackageModelDescription.md)
 - [WorkPackageModelLinks](docs/WorkPackageModelLinks.md)
 - [WorkPackageModelLinksAddAttachment](docs/WorkPackageModelLinksAddAttachment.md)
 - [WorkPackageModelLinksAddComment](docs/WorkPackageModelLinksAddComment.md)
 - [WorkPackageModelLinksAddFileLink](docs/WorkPackageModelLinksAddFileLink.md)
 - [WorkPackageModelLinksAddRelation](docs/WorkPackageModelLinksAddRelation.md)
 - [WorkPackageModelLinksAddWatcher](docs/WorkPackageModelLinksAddWatcher.md)
 - [WorkPackageModelLinksAncestorsInner](docs/WorkPackageModelLinksAncestorsInner.md)
 - [WorkPackageModelLinksAssignee](docs/WorkPackageModelLinksAssignee.md)
 - [WorkPackageModelLinksAttachments](docs/WorkPackageModelLinksAttachments.md)
 - [WorkPackageModelLinksAuthor](docs/WorkPackageModelLinksAuthor.md)
 - [WorkPackageModelLinksAvailableWatchers](docs/WorkPackageModelLinksAvailableWatchers.md)
 - [WorkPackageModelLinksBudget](docs/WorkPackageModelLinksBudget.md)
 - [WorkPackageModelLinksCategory](docs/WorkPackageModelLinksCategory.md)
 - [WorkPackageModelLinksChildrenInner](docs/WorkPackageModelLinksChildrenInner.md)
 - [WorkPackageModelLinksCustomActionsInner](docs/WorkPackageModelLinksCustomActionsInner.md)
 - [WorkPackageModelLinksFileLinks](docs/WorkPackageModelLinksFileLinks.md)
 - [WorkPackageModelLinksParent](docs/WorkPackageModelLinksParent.md)
 - [WorkPackageModelLinksPreviewMarkup](docs/WorkPackageModelLinksPreviewMarkup.md)
 - [WorkPackageModelLinksPriority](docs/WorkPackageModelLinksPriority.md)
 - [WorkPackageModelLinksProject](docs/WorkPackageModelLinksProject.md)
 - [WorkPackageModelLinksRelations](docs/WorkPackageModelLinksRelations.md)
 - [WorkPackageModelLinksRemoveWatcher](docs/WorkPackageModelLinksRemoveWatcher.md)
 - [WorkPackageModelLinksResponsible](docs/WorkPackageModelLinksResponsible.md)
 - [WorkPackageModelLinksRevisions](docs/WorkPackageModelLinksRevisions.md)
 - [WorkPackageModelLinksSchema](docs/WorkPackageModelLinksSchema.md)
 - [WorkPackageModelLinksSelf](docs/WorkPackageModelLinksSelf.md)
 - [WorkPackageModelLinksStatus](docs/WorkPackageModelLinksStatus.md)
 - [WorkPackageModelLinksTimeEntries](docs/WorkPackageModelLinksTimeEntries.md)
 - [WorkPackageModelLinksType](docs/WorkPackageModelLinksType.md)
 - [WorkPackageModelLinksUnwatch](docs/WorkPackageModelLinksUnwatch.md)
 - [WorkPackageModelLinksUpdate](docs/WorkPackageModelLinksUpdate.md)
 - [WorkPackageModelLinksUpdateImmediately](docs/WorkPackageModelLinksUpdateImmediately.md)
 - [WorkPackageModelLinksVersion](docs/WorkPackageModelLinksVersion.md)
 - [WorkPackageModelLinksWatch](docs/WorkPackageModelLinksWatch.md)
 - [WorkPackageModelLinksWatchers](docs/WorkPackageModelLinksWatchers.md)
 - [WorkPackagePatchModel](docs/WorkPackagePatchModel.md)
 - [WorkPackagePatchModelLinks](docs/WorkPackagePatchModelLinks.md)
 - [WorkPackagesModel](docs/WorkPackagesModel.md)
 - [WorkPackagesModelAllOfEmbedded](docs/WorkPackagesModelAllOfEmbedded.md)
 - [WorkPackagesModelAllOfLinks](docs/WorkPackagesModelAllOfLinks.md)
 - [WorkPackagesModelAllOfLinksSelf](docs/WorkPackagesModelAllOfLinksSelf.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



