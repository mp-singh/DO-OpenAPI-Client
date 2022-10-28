# FloatingIp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip** | Option<**String**> | The public IP address of the floating IP. It also serves as its identifier. | [optional]
**region** | Option<[**crate::models::FloatingIpRegion**](floating_ip_region.md)> |  | [optional]
**droplet** | Option<[**crate::models::FloatingIpDroplet**](floating_ip_droplet.md)> |  | [optional]
**locked** | Option<**bool**> | A boolean value indicating whether or not the floating IP has pending actions preventing new ones from being submitted. | [optional]
**project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The UUID of the project to which the reserved IP currently belongs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


