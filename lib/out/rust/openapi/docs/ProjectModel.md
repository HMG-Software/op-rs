# ProjectModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> |  | [optional]
**id** | Option<**i32**> | Projects' id | [optional]
**identifier** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**active** | Option<**bool**> | Indicates whether the project is currently active or already archived | [optional]
**status_explanation** | Option<[**models::ProjectModelStatusExplanation**](ProjectModel_statusExplanation.md)> |  | [optional]
**public** | Option<**bool**> | Indicates whether the project is accessible for everybody | [optional]
**description** | Option<[**models::Formattable**](Formattable.md)> |  | [optional]
**created_at** | Option<**String**> | Time of creation | [optional]
**updated_at** | Option<**String**> | Time of the most recent change to the project | [optional]
**_links** | Option<[**models::ProjectModelLinks**](ProjectModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


