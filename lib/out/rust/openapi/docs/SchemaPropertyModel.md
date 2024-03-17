# SchemaPropertyModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The resource type for this property. | 
**name** | **String** | The name of the property. | 
**required** | **bool** | Indicates, if the property is required for submitting a request of this schema. | 
**has_default** | **bool** | Indicates, if the property has a default. | 
**writable** | **bool** | Indicates, if the property is writable when sending a request of this schema. | 
**object** | Option<[**serde_json::Value**](.md)> | Additional options for the property. | [optional]
**location** | Option<**String**> | Defines the json path where the property is located in the payload. | [optional][default to ]
**_links** | Option<[**serde_json::Value**](.md)> | Useful links for this property (e.g. an endpoint to fetch allowed values) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


