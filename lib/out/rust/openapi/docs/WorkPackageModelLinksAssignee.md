# WorkPackageModelLinksAssignee

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**href** | Option<[**serde_json::Value**](.md)> | URL to the referenced resource (might be relative) | 
**title** | Option<[**serde_json::Value**](.md)> | Representative label for the resource | [optional]
**templated** | Option<[**serde_json::Value**](.md)> | If true the href contains parts that need to be replaced by the client | [optional][default to false]
**method** | Option<[**serde_json::Value**](.md)> | The HTTP verb to use when requesting the resource | [optional][default to GET]
**payload** | Option<[**serde_json::Value**](.md)> | The payload to send in the request to achieve the desired result | [optional]
**identifier** | Option<[**serde_json::Value**](.md)> | An optional unique identifier to the link object | [optional]
**r#type** | Option<[**serde_json::Value**](.md)> | The MIME-Type of the returned resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


