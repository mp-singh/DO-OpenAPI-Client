# FloatingIpsActionPost201ResponseAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | A unique numeric ID that can be used to identify and reference an action. | [optional]
**status** | Option<**String**> | The current status of the action. This can be \"in-progress\", \"completed\", or \"errored\". | [optional][default to InProgress]
**r#type** | Option<**String**> | This is the type of action that the object represents. For example, this could be \"transfer\" to represent the state of an image transfer action. | [optional]
**started_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the action was initiated. | [optional]
**completed_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the action was completed. | [optional]
**resource_id** | Option<**i32**> | A unique identifier for the resource that the action is associated with. | [optional]
**resource_type** | Option<**String**> | The type of resource that the action is associated with. | [optional]
**region** | Option<[**crate::models::Region**](region.md)> |  | [optional]
**region_slug** | Option<**String**> |  | [optional]
**project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The UUID of the project to which the reserved IP currently belongs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


