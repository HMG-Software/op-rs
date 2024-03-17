# WorkPackageModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Work package id | [optional][readonly]
**lock_version** | Option<**i32**> | The version of the item as used for optimistic locking | [optional][readonly]
**subject** | **String** | Work package subject | 
**_type** | Option<**String**> |  | [optional][readonly]
**description** | Option<[**models::WorkPackageModelDescription**](Work_PackageModel_description.md)> |  | [optional]
**schedule_manually** | Option<**bool**> | If false (default) schedule automatically. | [optional]
**readonly** | Option<**bool**> | If true, the work package is in a readonly status so with the exception of the status, no other property can be altered. | [optional]
**start_date** | Option<[**String**](string.md)> | Scheduled beginning of a work package | [optional]
**due_date** | Option<[**String**](string.md)> | Scheduled end of a work package | [optional]
**date** | Option<[**String**](string.md)> | Date on which a milestone is achieved | [optional]
**derived_start_date** | Option<[**String**](string.md)> | Similar to start date but is not set by a client but rather deduced by the work packages' descendants. If manual scheduleManually is active, the two dates can deviate. | [optional][readonly]
**derived_due_date** | Option<[**String**](string.md)> | Similar to due date but is not set by a client but rather deduced by the work packages' descendants. If manual scheduleManually is active, the two dates can deviate. | [optional][readonly]
**duration** | Option<**String**> | **(NOT IMPLEMENTED)** The amount of time in hours the work package needs to be completed. Not available for milestone type of work packages. | [optional][readonly]
**estimated_time** | Option<**String**> | Time a work package likely needs to be completed excluding its descendants | [optional]
**derived_estimated_time** | Option<**String**> | Time a work package likely needs to be completed including its descendants | [optional][readonly]
**ignore_non_working_days** | Option<**bool**> | **(NOT IMPLEMENTED)** When scheduling, whether or not to ignore the non working days being defined. A work package with the flag set to true will be allowed to be scheduled to a non working day. | [optional][readonly]
**spent_time** | Option<**String**> | The time booked for this work package by users working on it  # Conditions  **Permission** view time entries | [optional][readonly]
**percentage_done** | Option<**i32**> | Amount of total completion for a work package | [optional]
**derived_percentage_done** | Option<**i32**> | Amount of total completion for a work package derived from itself and its descendant work packages | [optional][readonly]
**created_at** | Option<**String**> | Time of creation | [optional][readonly]
**updated_at** | Option<**String**> | Time of the most recent change to the work package | [optional][readonly]
**_links** | [**models::WorkPackageModelLinks**](Work_PackageModel__links.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


