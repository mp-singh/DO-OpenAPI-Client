# Droplet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | A unique identifier for each Droplet instance. This is automatically generated upon Droplet creation. | 
**name** | **String** | The human-readable name set for the Droplet instance. | 
**memory** | **i32** | Memory of the Droplet in megabytes. | 
**vcpus** | **i32** | The number of virtual CPUs. | 
**disk** | **i32** | The size of the Droplet's disk in gigabytes. | 
**locked** | **bool** | A boolean value indicating whether the Droplet has been locked, preventing actions by users. | 
**status** | **String** | A status string indicating the state of the Droplet instance. This may be \"new\", \"active\", \"off\", or \"archive\". | 
**kernel** | Option<[**crate::models::Kernel**](kernel.md)> |  | [optional]
**created_at** | **String** | A time value given in ISO8601 combined date and time format that represents when the Droplet was created. | 
**features** | **Vec<String>** | An array of features enabled on this Droplet. | 
**backup_ids** | **Vec<i32>** | An array of backup IDs of any backups that have been taken of the Droplet instance.  Droplet backups are enabled at the time of the instance creation. | 
**next_backup_window** | Option<[**crate::models::DropletNextBackupWindow**](droplet_next_backup_window.md)> |  | 
**snapshot_ids** | **Vec<i32>** | An array of snapshot IDs of any snapshots created from the Droplet instance. | 
**image** | [**crate::models::Image**](image.md) |  | 
**volume_ids** | **Vec<String>** | A flat array including the unique identifier for each Block Storage volume attached to the Droplet. | 
**size** | [**crate::models::Size**](size.md) |  | 
**size_slug** | **String** | The unique slug identifier for the size of this Droplet. | 
**networks** | [**crate::models::DropletNetworks**](droplet_networks.md) |  | 
**region** | [**crate::models::Region**](region.md) |  | 
**tags** | **Vec<String>** | An array of Tags the Droplet has been tagged with. | 
**vpc_uuid** | Option<**String**> | A string specifying the UUID of the VPC to which the Droplet is assigned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


