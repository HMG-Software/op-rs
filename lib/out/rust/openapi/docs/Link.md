# Link

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**href** | Option<**String**> | URL to the referenced resource (might be relative) | 
**title** | Option<**String**> | Representative label for the resource | [optional]
**templated** | Option<**bool**> | If true the href contains parts that need to be replaced by the client | [optional][default to false]
**method** | Option<**String**> | The HTTP verb to use when requesting the resource | [optional][default to GET]
**payload** | Option<[**serde_json::Value**](.md)> | The payload to send in the request to achieve the desired result | [optional]
**identifier** | Option<**String**> | An optional unique identifier to the link object | [optional]
**r#type** | Option<**String**> | The MIME-Type of the returned resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


