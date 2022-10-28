# DatabasesCreateClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference a database cluster. | [optional][readonly]
**name** | **String** | A unique, human-readable name referring to a database cluster. | 
**engine** | **String** | A slug representing the database engine used for the cluster. The possible values are: \"pg\" for PostgreSQL, \"mysql\" for MySQL, \"redis\" for Redis, and \"mongodb\" for MongoDB. | 
**version** | Option<**String**> | A string representing the version of the database engine in use for the cluster. | [optional]
**num_nodes** | **i32** | The number of nodes in the database cluster. | 
**size** | **String** | The slug identifier representing the size of the nodes in the database cluster. | 
**region** | **String** | The slug identifier for the region where the database cluster is located. | 
**status** | Option<**String**> | A string representing the current status of the database cluster. | [optional][readonly]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the database cluster was created. | [optional][readonly]
**private_network_uuid** | Option<**String**> | A string specifying the UUID of the VPC to which the database cluster will be assigned. If excluded, the cluster when creating a new database cluster, it will be assigned to your account's default VPC for the region. | [optional]
**tags** | Option<**Vec<String>**> | An array of tags that have been applied to the database cluster. | [optional]
**db_names** | Option<**Vec<String>**> | An array of strings containing the names of databases created in the database cluster. | [optional][readonly]
**connection** | Option<[**crate::models::DatabaseClusterConnection**](database_cluster_connection.md)> |  | [optional]
**private_connection** | Option<[**crate::models::DatabaseClusterConnection**](database_cluster_connection.md)> |  | [optional]
**users** | Option<[**Vec<crate::models::DatabaseUser>**](database_user.md)> |  | [optional][readonly]
**maintenance_window** | Option<[**crate::models::DatabaseClusterMaintenanceWindow**](database_cluster_maintenance_window.md)> |  | [optional]
**project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the project that the database cluster is assigned to. If excluded when creating a new database cluster, it will be assigned to your default project. | [optional]
**rules** | Option<[**Vec<crate::models::FirewallRule>**](firewall_rule.md)> |  | [optional]
**backup_restore** | Option<[**crate::models::DatabaseBackup**](database_backup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


