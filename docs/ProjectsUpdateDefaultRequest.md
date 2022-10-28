# ProjectsUpdateDefaultRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The unique universal identifier of this project. | [optional][readonly]
**owner_uuid** | Option<**String**> | The unique universal identifier of the project owner. | [optional][readonly]
**owner_id** | Option<**i32**> | The integer id of the project owner. | [optional][readonly]
**name** | **String** | The human-readable name for the project. The maximum length is 175 characters and the name must be unique. | 
**description** | **String** | The description of the project. The maximum length is 255 characters. | 
**purpose** | **String** | The purpose of the project. The maximum length is 255 characters. It can have one of the following values:  - Just trying out DigitalOcean - Class project / Educational purposes - Website or blog - Web Application - Service or API - Mobile Application - Machine learning / AI / Data processing - IoT - Operational / Developer tooling  If another value for purpose is specified, for example, \"your custom purpose\", your purpose will be stored as `Other: your custom purpose`.  | 
**environment** | **String** | The environment of the project's resources. | 
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the project was created. | [optional][readonly]
**updated_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the project was updated. | [optional][readonly]
**is_default** | **bool** | If true, all resources will be added to this project if no project is specified. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


