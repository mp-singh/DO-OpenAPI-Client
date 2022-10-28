# UptimeCheckCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A human-friendly display name. | 
**r#type** | Option<**String**> | The type of health check to perform. | [optional]
**target** | **String** | The endpoint to perform healthchecks on. | 
**regions** | Option<**Vec<String>**> | An array containing the selected regions to perform healthchecks from. | [optional]
**enabled** | Option<**bool**> | A boolean value indicating whether the check is enabled/disabled. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


