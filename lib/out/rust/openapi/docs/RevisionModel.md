# RevisionModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Revision's id, assigned by OpenProject | [optional][readonly]
**identifier** | **String** | The raw SCM identifier of the revision (e.g. full SHA hash) | [readonly]
**formatted_identifier** | **String** | The SCM identifier of the revision, formatted (e.g. shortened unambiguous SHA hash). May be identical to identifier in many cases | [readonly]
**author_name** | **String** | The name of the author that committed this revision. Note that this name is retrieved from the repository and does not identify a user in OpenProject. | [readonly]
**message** | [**models::RevisionModelMessage**](RevisionModel_message.md) |  | 
**created_at** | **String** | The time this revision was committed to the repository | 
**_links** | Option<[**models::RevisionModelLinks**](RevisionModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


