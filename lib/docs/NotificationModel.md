# NotificationModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> |  | [optional]
**id** | Option<**i32**> | Notification id | [optional]
**reason** | Option<**String**> | The reason for the notification | [optional]
**read_ian** | Option<**bool**> | Whether the notification is marked as read | [optional]
**details** | Option<[**Vec<models::NotificationModelDetailsInner>**](NotificationModel_details_inner.md)> | A list of objects including detailed information about the notification. | [optional]
**created_at** | Option<**String**> | The time the notification was created at | [optional]
**updated_at** | Option<**String**> | The time the notification was last updated | [optional]
**_embedded** | Option<[**models::NotificationModelEmbedded**](NotificationModel__embedded.md)> |  | [optional]
**_links** | Option<[**models::NotificationModelLinks**](NotificationModel__links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


