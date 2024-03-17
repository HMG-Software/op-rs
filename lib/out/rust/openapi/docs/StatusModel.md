# StatusModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> |  | [optional]
**id** | Option<**i32**> | Status id | [optional][readonly]
**name** | Option<**String**> | Status name | [optional][readonly]
**position** | Option<**i32**> | Sort index of the status | [optional][readonly]
**is_default** | Option<**bool**> |  | [optional][readonly]
**is_closed** | Option<**bool**> | are tickets of this status considered closed? | [optional][readonly]
**is_readonly** | Option<**bool**> | are tickets of this status read only? | [optional][readonly]
**default_done_ratio** | Option<**i32**> | The percentageDone being applied when changing to this status | [optional]
**_links** | Option<[**models::StatusModelLinks**](StatusModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


