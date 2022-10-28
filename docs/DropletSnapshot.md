# DropletSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The unique identifier for the snapshot or backup. | 
**name** | **String** | A human-readable name for the snapshot. | 
**created_at** | **String** | A time value given in ISO8601 combined date and time format that represents when the snapshot was created. | 
**regions** | **Vec<String>** | An array of the regions that the snapshot is available in. The regions are represented by their identifying slug values. | 
**min_disk_size** | **i32** | The minimum size in GB required for a volume or Droplet to use this snapshot. | 
**size_gigabytes** | **f32** | The billable size of the snapshot in gigabytes. | 
**r#type** | **String** | Describes the kind of image. It may be one of `snapshot` or `backup`. This specifies whether an image is a user-generated Droplet snapshot or automatically created Droplet backup. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


