# GridWidgetModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** |  | 
**id** | Option<**i32**> | The grid widget's unique identifier. Can be null, if a new widget is created within a grid. | 
**identifier** | **String** | An alternative, human legible, and unique identifier. | 
**start_row** | **i32** | The index of the starting row of the widget. The row is inclusive. | 
**end_row** | **i32** | The index of the ending row of the widget. The row is exclusive. | 
**start_column** | **i32** | The index of the starting column of the widget. The column is inclusive. | 
**end_column** | **i32** | The index of the ending column of the widget. The column is exclusive. | 
**options** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


