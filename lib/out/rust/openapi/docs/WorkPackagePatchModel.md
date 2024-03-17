# WorkPackagePatchModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lock_version** | **i32** | The version of the item as used for optimistic locking | 
**subject** | Option<**String**> | Work package subject | [optional]
**description** | Option<[**models::WorkPackageModelDescription**](Work_PackageModel_description.md)> |  | [optional]
**schedule_manually** | Option<**bool**> | If false (default) schedule automatically. | [optional]
**start_date** | Option<[**String**](string.md)> | Scheduled beginning of a work package | [optional]
**due_date** | Option<[**String**](string.md)> | Scheduled end of a work package | [optional]
**date** | Option<[**String**](string.md)> | Date on which a milestone is achieved | [optional]
**estimated_time** | Option<**String**> | Time a work package likely needs to be completed excluding its descendants | [optional]
**ignore_non_working_days** | Option<**bool**> | **(NOT IMPLEMENTED)** When scheduling, whether or not to ignore the non working days being defined. A work package with the flag set to true will be allowed to be scheduled to a non working day. | [optional][readonly]
**spent_time** | Option<**String**> | The time booked for this work package by users working on it  # Conditions  **Permission** view time entries | [optional][readonly]
**percentage_done** | Option<**i32**> | Amount of total completion for a work package | [optional]
**created_at** | Option<**String**> | Time of creation | [optional][readonly]
**updated_at** | Option<**String**> | Time of the most recent change to the work package | [optional][readonly]
**_links** | Option<[**models::WorkPackagePatchModelLinks**](WorkPackagePatchModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


