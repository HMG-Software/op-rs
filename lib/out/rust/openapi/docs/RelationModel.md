# RelationModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Relation ID | [optional][readonly]
**name** | Option<**String**> | The internationalized name of this kind of relation | [optional]
**r#type** | Option<**String**> | Which kind of relation (blocks, precedes, etc.) | [optional]
**reverse_type** | Option<**String**> | The kind of relation from the other WP's perspective | [optional][readonly]
**description** | Option<**String**> | Short text further describing the relation | [optional]
**delay_star** | Option<**i32**> | The delay in days between closing of `from` and start of `to` | [optional]
**_links** | Option<[**models::RelationModelLinks**](RelationModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


