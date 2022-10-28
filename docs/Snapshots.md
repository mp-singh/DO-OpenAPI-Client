# Snapshots

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for the snapshot. | 
**name** | **String** | A human-readable name for the snapshot. | 
**created_at** | **String** | A time value given in ISO8601 combined date and time format that represents when the snapshot was created. | 
**regions** | **Vec<String>** | An array of the regions that the snapshot is available in. The regions are represented by their identifying slug values. | 
**min_disk_size** | **i32** | The minimum size in GB required for a volume or Droplet to use this snapshot. | 
**size_gigabytes** | **f32** | The billable size of the snapshot in gigabytes. | 
**resource_id** | **String** | The unique identifier for the resource that the snapshot originated from. | 
**resource_type** | **String** | The type of resource that the snapshot originated from. | 
**tags** | Option<**Vec<String>**> | An array of Tags the snapshot has been tagged with. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


