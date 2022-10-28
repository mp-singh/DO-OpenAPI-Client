# DropletActionsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of action to initiate for the Droplet. | 
**image** | Option<[**crate::models::OneOfLessThanStringCommaIntegerGreaterThan**](oneOf<string,integer>.md)> | The image ID of a public or private image or the slug identifier for a public image. The Droplet will be rebuilt using this image as its base. | [optional]
**disk** | Option<**bool**> | When `true`, the Droplet's disk will be resized in addition to its RAM and CPU. This is a permanent change and cannot be reversed as a Droplet's disk size cannot be decreased. | [optional]
**size** | Option<**String**> | The slug identifier for the size to which you wish to resize the Droplet. | [optional]
**name** | Option<**String**> | The name to give the new snapshot of the Droplet. | [optional]
**kernel** | Option<**i32**> | A unique number used to identify and reference a specific kernel. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


