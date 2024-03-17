# GridReadModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** |  | 
**id** | **i32** | Grid's id | 
**row_count** | **i32** | The number of rows the grid has | 
**column_count** | **i32** | The number of columns the grid has | 
**widgets** | [**Vec<models::GridWidgetModel>**](GridWidgetModel.md) | The set of `GridWidget`s selected for the grid.  # Conditions  - The widgets must not overlap. | 
**created_at** | Option<**String**> | The time the grid was created. | [optional]
**updated_at** | Option<**String**> | The time the grid was last updated. | [optional]
**_links** | [**models::GridReadModelLinks**](GridReadModel__links.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


