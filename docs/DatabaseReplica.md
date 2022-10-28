# DatabaseReplica

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference a database replica. | [optional][readonly]
**name** | **String** | The name to give the read-only replicating | 
**region** | Option<**String**> | A slug identifier for the region where the read-only replica will be located. If excluded, the replica will be placed in the same region as the cluster. | [optional]
**size** | Option<**String**> | A slug identifier representing the size of the node for the read-only replica. The size of the replica must be at least as large as the node size for the database cluster from which it is replicating. | [optional]
**status** | Option<**String**> | A string representing the current status of the database cluster. | [optional][readonly]
**tags** | Option<**Vec<String>**> | A flat array of tag names as strings to apply to the read-only replica after it is created. Tag names can either be existing or new tags. | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the database cluster was created. | [optional][readonly]
**private_network_uuid** | Option<**String**> | A string specifying the UUID of the VPC to which the read-only replica will be assigned. If excluded, the replica will be assigned to your account's default VPC for the region. | [optional]
**connection** | Option<[**crate::models::DatabaseReplicaConnection**](database_replica_connection.md)> |  | [optional]
**private_connection** | Option<[**crate::models::DatabaseReplicaConnection**](database_replica_connection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


