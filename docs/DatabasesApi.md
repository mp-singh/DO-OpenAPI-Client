# \DatabasesApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**databases_add**](DatabasesApi.md#databases_add) | **POST** /v2/databases/{database_cluster_uuid}/dbs | Add a New Database
[**databases_add_connection_pool**](DatabasesApi.md#databases_add_connection_pool) | **POST** /v2/databases/{database_cluster_uuid}/pools | Add a New Connection Pool (PostgreSQL)
[**databases_add_user**](DatabasesApi.md#databases_add_user) | **POST** /v2/databases/{database_cluster_uuid}/users | Add a Database User
[**databases_create_cluster**](DatabasesApi.md#databases_create_cluster) | **POST** /v2/databases | Create a New Database Cluster
[**databases_create_replica**](DatabasesApi.md#databases_create_replica) | **POST** /v2/databases/{database_cluster_uuid}/replicas | Create a Read-only Replica
[**databases_delete**](DatabasesApi.md#databases_delete) | **DELETE** /v2/databases/{database_cluster_uuid}/dbs/{database_name} | Delete a Database
[**databases_delete_connection_pool**](DatabasesApi.md#databases_delete_connection_pool) | **DELETE** /v2/databases/{database_cluster_uuid}/pools/{pool_name} | Delete a Connection Pool (PostgreSQL)
[**databases_delete_online_migration**](DatabasesApi.md#databases_delete_online_migration) | **DELETE** /v2/databases/{database_cluster_uuid}/online-migration/{migration_id} | Stop an Online Migration
[**databases_delete_user**](DatabasesApi.md#databases_delete_user) | **DELETE** /v2/databases/{database_cluster_uuid}/users/{username} | Remove a Database User
[**databases_destroy_cluster**](DatabasesApi.md#databases_destroy_cluster) | **DELETE** /v2/databases/{database_cluster_uuid} | Destroy a Database Cluster
[**databases_destroy_replica**](DatabasesApi.md#databases_destroy_replica) | **DELETE** /v2/databases/{database_cluster_uuid}/replicas/{replica_name} | Destroy a Read-only Replica
[**databases_get**](DatabasesApi.md#databases_get) | **GET** /v2/databases/{database_cluster_uuid}/dbs/{database_name} | Retrieve an Existing Database
[**databases_get_ca**](DatabasesApi.md#databases_get_ca) | **GET** /v2/databases/{database_cluster_uuid}/ca | Retrieve the Public Certificate
[**databases_get_cluster**](DatabasesApi.md#databases_get_cluster) | **GET** /v2/databases/{database_cluster_uuid} | Retrieve an Existing Database Cluster
[**databases_get_config**](DatabasesApi.md#databases_get_config) | **GET** /v2/databases/{database_cluster_uuid}/config | Retrieve an Existing Database Cluster Configuration
[**databases_get_connection_pool**](DatabasesApi.md#databases_get_connection_pool) | **GET** /v2/databases/{database_cluster_uuid}/pools/{pool_name} | Retrieve Existing Connection Pool (PostgreSQL)
[**databases_get_eviction_policy**](DatabasesApi.md#databases_get_eviction_policy) | **GET** /v2/databases/{database_cluster_uuid}/eviction_policy | Retrieve the Eviction Policy for a Redis Cluster
[**databases_get_migration_status**](DatabasesApi.md#databases_get_migration_status) | **GET** /v2/databases/{database_cluster_uuid}/online-migration | Retrieve the Status of an Online Migration
[**databases_get_replica**](DatabasesApi.md#databases_get_replica) | **GET** /v2/databases/{database_cluster_uuid}/replicas/{replica_name} | Retrieve an Existing Read-only Replica
[**databases_get_sql_mode**](DatabasesApi.md#databases_get_sql_mode) | **GET** /v2/databases/{database_cluster_uuid}/sql_mode | Retrieve the SQL Modes for a MySQL Cluster
[**databases_get_user**](DatabasesApi.md#databases_get_user) | **GET** /v2/databases/{database_cluster_uuid}/users/{username} | Retrieve an Existing Database User
[**databases_list**](DatabasesApi.md#databases_list) | **GET** /v2/databases/{database_cluster_uuid}/dbs | List All Databases
[**databases_list_backups**](DatabasesApi.md#databases_list_backups) | **GET** /v2/databases/{database_cluster_uuid}/backups | List Backups for a Database Cluster
[**databases_list_clusters**](DatabasesApi.md#databases_list_clusters) | **GET** /v2/databases | List All Database Clusters
[**databases_list_connection_pools**](DatabasesApi.md#databases_list_connection_pools) | **GET** /v2/databases/{database_cluster_uuid}/pools | List Connection Pools (PostgreSQL)
[**databases_list_firewall_rules**](DatabasesApi.md#databases_list_firewall_rules) | **GET** /v2/databases/{database_cluster_uuid}/firewall | List Firewall Rules (Trusted Sources) for a Database Cluster
[**databases_list_options**](DatabasesApi.md#databases_list_options) | **GET** /v2/databases/options | List Database Options
[**databases_list_replicas**](DatabasesApi.md#databases_list_replicas) | **GET** /v2/databases/{database_cluster_uuid}/replicas | List All Read-only Replicas
[**databases_list_users**](DatabasesApi.md#databases_list_users) | **GET** /v2/databases/{database_cluster_uuid}/users | List all Database Users
[**databases_patch_config**](DatabasesApi.md#databases_patch_config) | **PATCH** /v2/databases/{database_cluster_uuid}/config | Update the Database Configuration for an Existing Database
[**databases_reset_auth**](DatabasesApi.md#databases_reset_auth) | **POST** /v2/databases/{database_cluster_uuid}/users/{username}/reset_auth | Reset a Database User's Password or Authentication Method
[**databases_update_cluster_size**](DatabasesApi.md#databases_update_cluster_size) | **PUT** /v2/databases/{database_cluster_uuid}/resize | Resize a Database Cluster
[**databases_update_eviction_policy**](DatabasesApi.md#databases_update_eviction_policy) | **PUT** /v2/databases/{database_cluster_uuid}/eviction_policy | Configure the Eviction Policy for a Redis Cluster
[**databases_update_firewall_rules**](DatabasesApi.md#databases_update_firewall_rules) | **PUT** /v2/databases/{database_cluster_uuid}/firewall | Update Firewall Rules (Trusted Sources) for a Database
[**databases_update_maintenance_window**](DatabasesApi.md#databases_update_maintenance_window) | **PUT** /v2/databases/{database_cluster_uuid}/maintenance | Configure a Database Cluster's Maintenance Window
[**databases_update_online_migration**](DatabasesApi.md#databases_update_online_migration) | **PUT** /v2/databases/{database_cluster_uuid}/online-migration | Start an Online Migration
[**databases_update_region**](DatabasesApi.md#databases_update_region) | **PUT** /v2/databases/{database_cluster_uuid}/migrate | Migrate a Database Cluster to a New Region
[**databases_update_sql_mode**](DatabasesApi.md#databases_update_sql_mode) | **PUT** /v2/databases/{database_cluster_uuid}/sql_mode | Update SQL Mode for a Cluster



## databases_add

> crate::models::DatabasesAdd201Response databases_add(database_cluster_uuid, database)
Add a New Database

To add a new database to an existing cluster, send a POST request to `/v2/databases/$DATABASE_ID/dbs`.  Note: Database management is not supported for Redis clusters.  The response will be a JSON object with a key called `db`. The value of this will be an object that contains the standard attributes associated with a database. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**database** | [**Database**](Database.md) |  | [required] |

### Return type

[**crate::models::DatabasesAdd201Response**](databases_add_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_add_connection_pool

> crate::models::DatabasesAddConnectionPool201Response databases_add_connection_pool(database_cluster_uuid, connection_pool)
Add a New Connection Pool (PostgreSQL)

For PostgreSQL database clusters, connection pools can be used to allow a database to share its idle connections. The popular PostgreSQL connection pooling utility PgBouncer is used to provide this service. [See here for more information](https://www.digitalocean.com/docs/databases/postgresql/how-to/manage-connection-pools/) about how and why to use PgBouncer connection pooling including details about the available transaction modes.  To add a new connection pool to a PostgreSQL database cluster, send a POST request to `/v2/databases/$DATABASE_ID/pools` specifying a name for the pool, the user to connect with, the database to connect to, as well as its desired size and transaction mode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**connection_pool** | [**ConnectionPool**](ConnectionPool.md) |  | [required] |

### Return type

[**crate::models::DatabasesAddConnectionPool201Response**](databases_add_connectionPool_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_add_user

> crate::models::DatabasesAddUser201Response databases_add_user(database_cluster_uuid, database_user)
Add a Database User

To add a new database user, send a POST request to `/v2/databases/$DATABASE_ID/users` with the desired username.  Note: User management is not supported for Redis clusters.  When adding a user to a MySQL cluster, additional options can be configured in the `mysql_settings` object.  The response will be a JSON object with a key called `user`. The value of this will be an object that contains the standard attributes associated with a database user including its randomly generated password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**database_user** | [**DatabaseUser**](DatabaseUser.md) |  | [required] |

### Return type

[**crate::models::DatabasesAddUser201Response**](databases_add_user_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_cluster

> crate::models::DatabasesCreateCluster201Response databases_create_cluster(databases_create_cluster_request)
Create a New Database Cluster

To create a database cluster, send a POST request to `/v2/databases`. The response will be a JSON object with a key called `database`. The value of this will be an object that contains the standard attributes associated with a database cluster. The initial value of the database cluster's `status` attribute will be `creating`. When the cluster is ready to receive traffic, this will transition to `online`. The embedded `connection` and `private_connection` objects will contain the information needed to access the database cluster. DigitalOcean managed PostgreSQL and MySQL database clusters take automated daily backups. To create a new database cluster based on a backup of an existing cluster, send a POST request to `/v2/databases`. In addition to the standard database cluster attributes, the JSON body must include a key named `backup_restore` with the name of the original database cluster and the timestamp of the backup to be restored. Creating a database from a backup is the same as forking a database in the control panel. Note: Backups are not supported for Redis clusters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**databases_create_cluster_request** | [**DatabasesCreateClusterRequest**](DatabasesCreateClusterRequest.md) |  | [required] |

### Return type

[**crate::models::DatabasesCreateCluster201Response**](databases_create_cluster_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_replica

> crate::models::DatabasesCreateReplica201Response databases_create_replica(database_cluster_uuid, databases_create_replica_request)
Create a Read-only Replica

To create a read-only replica for a PostgreSQL or MySQL database cluster, send a POST request to `/v2/databases/$DATABASE_ID/replicas` specifying the name it should be given, the size of the node to be used, and the region where it will be located. **Note**: Read-only replicas are not supported for Redis clusters. The response will be a JSON object with a key called `replica`. The value of this will be an object that contains the standard attributes associated with a database replica. The initial value of the read-only replica's `status` attribute will be `forking`. When the replica is ready to receive traffic, this will transition to `active`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**databases_create_replica_request** | Option<[**DatabasesCreateReplicaRequest**](DatabasesCreateReplicaRequest.md)> |  |  |

### Return type

[**crate::models::DatabasesCreateReplica201Response**](databases_create_replica_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete

> databases_delete(database_cluster_uuid, database_name)
Delete a Database

To delete a specific database, send a DELETE request to `/v2/databases/$DATABASE_ID/dbs/$DB_NAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.  Note: Database management is not supported for Redis clusters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**database_name** | **String** | The name of the database. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete_connection_pool

> databases_delete_connection_pool(database_cluster_uuid, pool_name)
Delete a Connection Pool (PostgreSQL)

To delete a specific connection pool for a PostgreSQL database cluster, send a DELETE request to `/v2/databases/$DATABASE_ID/pools/$POOL_NAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**pool_name** | **String** | The name used to identify the connection pool. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete_online_migration

> databases_delete_online_migration(database_cluster_uuid, migration_id)
Stop an Online Migration

To stop an online migration, send a DELETE request to `/v2/databases/$DATABASE_ID/online-migration/$MIGRATION_ID`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**migration_id** | **String** | A unique identifier assigned to the online migration. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete_user

> databases_delete_user(database_cluster_uuid, username)
Remove a Database User

To remove a specific database user, send a DELETE request to `/v2/databases/$DATABASE_ID/users/$USERNAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.  Note: User management is not supported for Redis clusters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**username** | **String** | The name of the database user. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_destroy_cluster

> databases_destroy_cluster(database_cluster_uuid)
Destroy a Database Cluster

To destroy a specific database, send a DELETE request to `/v2/databases/$DATABASE_ID`. A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_destroy_replica

> databases_destroy_replica(database_cluster_uuid, replica_name)
Destroy a Read-only Replica

To destroy a specific read-only replica, send a DELETE request to `/v2/databases/$DATABASE_ID/replicas/$REPLICA_NAME`. **Note**: Read-only replicas are not supported for Redis clusters. A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**replica_name** | **String** | The name of the database replica. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get

> crate::models::DatabasesAdd201Response databases_get(database_cluster_uuid, database_name)
Retrieve an Existing Database

To show information about an existing database cluster, send a GET request to `/v2/databases/$DATABASE_ID/dbs/$DB_NAME`.  Note: Database management is not supported for Redis clusters.  The response will be a JSON object with a `db` key. This will be set to an object containing the standard database attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**database_name** | **String** | The name of the database. | [required] |

### Return type

[**crate::models::DatabasesAdd201Response**](databases_add_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_ca

> crate::models::DatabasesGetCa200Response databases_get_ca(database_cluster_uuid)
Retrieve the Public Certificate

To retrieve the public certificate used to secure the connection to the database cluster send a GET request to `/v2/databases/$DATABASE_ID/ca`.  The response will be a JSON object with a `ca` key. This will be set to an object containing the base64 encoding of the public key certificate. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesGetCa200Response**](databases_get_ca_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_cluster

> crate::models::DatabasesCreateCluster201Response databases_get_cluster(database_cluster_uuid)
Retrieve an Existing Database Cluster

To show information about an existing database cluster, send a GET request to `/v2/databases/$DATABASE_ID`. The response will be a JSON object with a database key. This will be set to an object containing the standard database cluster attributes. The embedded connection and private_connection objects will contain the information needed to access the database cluster. The embedded maintenance_window object will contain information about any scheduled maintenance for the database cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesCreateCluster201Response**](databases_create_cluster_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_config

> crate::models::DatabasesGetConfig200Response databases_get_config(database_cluster_uuid)
Retrieve an Existing Database Cluster Configuration

Shows configuration parameters for an existing database cluster by sending a GET request to `/v2/databases/$DATABASE_ID/config`. The response is a JSON object with a `config` key, which is set to an object containing any database configuration parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesGetConfig200Response**](databases_get_config_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_connection_pool

> crate::models::DatabasesAddConnectionPool201Response databases_get_connection_pool(database_cluster_uuid, pool_name)
Retrieve Existing Connection Pool (PostgreSQL)

To show information about an existing connection pool for a PostgreSQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/pools/$POOL_NAME`. The response will be a JSON object with a `pool` key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**pool_name** | **String** | The name used to identify the connection pool. | [required] |

### Return type

[**crate::models::DatabasesAddConnectionPool201Response**](databases_add_connectionPool_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_eviction_policy

> crate::models::DatabasesGetEvictionPolicy200Response databases_get_eviction_policy(database_cluster_uuid)
Retrieve the Eviction Policy for a Redis Cluster

To retrieve the configured eviction policy for an existing Redis cluster, send a GET request to `/v2/databases/$DATABASE_ID/eviction_policy`. The response will be a JSON object with an `eviction_policy` key. This will be set to a string representing the eviction policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesGetEvictionPolicy200Response**](databases_get_evictionPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_migration_status

> crate::models::OnlineMigration databases_get_migration_status(database_cluster_uuid)
Retrieve the Status of an Online Migration

To retrieve the status of an online migration, send a GET request to `/v2/databases/$DATABASE_ID/online-migration`. If a migration has completed, a 200 OK status is returned with no response body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::OnlineMigration**](online_migration.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_replica

> crate::models::DatabasesCreateReplica201Response databases_get_replica(database_cluster_uuid, replica_name)
Retrieve an Existing Read-only Replica

To show information about an existing database replica, send a GET request to `/v2/databases/$DATABASE_ID/replicas/$REPLICA_NAME`. **Note**: Read-only replicas are not supported for Redis clusters. The response will be a JSON object with a `replica key`. This will be set to an object containing the standard database replica attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**replica_name** | **String** | The name of the database replica. | [required] |

### Return type

[**crate::models::DatabasesCreateReplica201Response**](databases_create_replica_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_sql_mode

> crate::models::SqlMode databases_get_sql_mode(database_cluster_uuid)
Retrieve the SQL Modes for a MySQL Cluster

To retrieve the configured SQL modes for an existing MySQL cluster, send a GET request to `/v2/databases/$DATABASE_ID/sql_mode`. The response will be a JSON object with a `sql_mode` key. This will be set to a string representing the configured SQL modes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::SqlMode**](sql_mode.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_user

> crate::models::DatabasesAddUser201Response databases_get_user(database_cluster_uuid, username)
Retrieve an Existing Database User

To show information about an existing database user, send a GET request to `/v2/databases/$DATABASE_ID/users/$USERNAME`.  Note: User management is not supported for Redis clusters.  The response will be a JSON object with a `user` key. This will be set to an object containing the standard database user attributes.  For MySQL clusters, additional options will be contained in the mysql_settings object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**username** | **String** | The name of the database user. | [required] |

### Return type

[**crate::models::DatabasesAddUser201Response**](databases_add_user_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list

> crate::models::DatabasesList200Response databases_list(database_cluster_uuid)
List All Databases

To list all of the databases in a clusters, send a GET request to `/v2/databases/$DATABASE_ID/dbs`.  The result will be a JSON object with a `dbs` key. This will be set to an array of database objects, each of which will contain the standard database attributes.  Note: Database management is not supported for Redis clusters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesList200Response**](databases_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_backups

> crate::models::DatabasesListBackups200Response databases_list_backups(database_cluster_uuid)
List Backups for a Database Cluster

To list all of the available backups of a PostgreSQL or MySQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/backups`. **Note**: Backups are not supported for Redis clusters. The result will be a JSON object with a `backups key`. This will be set to an array of backup objects, each of which will contain the size of the backup and the timestamp at which it was created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesListBackups200Response**](databases_list_backups_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_clusters

> crate::models::DatabasesListClusters200Response databases_list_clusters(tag_name)
List All Database Clusters

To list all of the database clusters available on your account, send a GET request to `/v2/databases`. To limit the results to database clusters with a specific tag, include the `tag_name` query parameter set to the name of the tag. For example, `/v2/databases?tag_name=$TAG_NAME`. The result will be a JSON object with a `databases` key. This will be set to an array of database objects, each of which will contain the standard database attributes. The embedded `connection` and `private_connection` objects will contain the information needed to access the database cluster: The embedded `maintenance_window` object will contain information about any scheduled maintenance for the database cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_name** | Option<**String**> | Limits the results to database clusters with a specific tag. |  |

### Return type

[**crate::models::DatabasesListClusters200Response**](databases_list_clusters_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_connection_pools

> crate::models::ConnectionPools databases_list_connection_pools(database_cluster_uuid)
List Connection Pools (PostgreSQL)

To list all of the connection pools available to a PostgreSQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/pools`. The result will be a JSON object with a `pools` key. This will be set to an array of connection pool objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::ConnectionPools**](connection_pools.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_firewall_rules

> crate::models::DatabasesListFirewallRules200Response databases_list_firewall_rules(database_cluster_uuid)
List Firewall Rules (Trusted Sources) for a Database Cluster

To list all of a database cluster's firewall rules (known as \"trusted sources\" in the control panel), send a GET request to `/v2/databases/$DATABASE_ID/firewall`. The result will be a JSON object with a `rules` key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesListFirewallRules200Response**](databases_list_firewall_rules_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_options

> crate::models::Options databases_list_options()
List Database Options

To list all of the options available for the offered database engines, send a GET request to `/v2/databases/options`. The result will be a JSON object with an `options` key.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Options**](options.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_replicas

> crate::models::DatabasesListReplicas200Response databases_list_replicas(database_cluster_uuid)
List All Read-only Replicas

To list all of the read-only replicas associated with a database cluster, send a GET request to `/v2/databases/$DATABASE_ID/replicas`. **Note**: Read-only replicas are not supported for Redis clusters. The result will be a JSON object with a `replicas` key. This will be set to an array of database replica objects, each of which will contain the standard database replica attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesListReplicas200Response**](databases_list_replicas_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_users

> crate::models::DatabasesListUsers200Response databases_list_users(database_cluster_uuid)
List all Database Users

To list all of the users for your database cluster, send a GET request to `/v2/databases/$DATABASE_ID/users`.  Note: User management is not supported for Redis clusters.  The result will be a JSON object with a `users` key. This will be set to an array of database user objects, each of which will contain the standard database user attributes.  For MySQL clusters, additional options will be contained in the mysql_settings object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |

### Return type

[**crate::models::DatabasesListUsers200Response**](databases_list_users_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_patch_config

> databases_patch_config(database_cluster_uuid, database_config)
Update the Database Configuration for an Existing Database

To update the configuration for an existing database cluster, send a PATCH request to `/v2/databases/$DATABASE_ID/config`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**database_config** | [**DatabaseConfig**](DatabaseConfig.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_reset_auth

> crate::models::DatabasesAddUser201Response databases_reset_auth(database_cluster_uuid, username, databases_reset_auth_request)
Reset a Database User's Password or Authentication Method

To reset the password for a database user, send a POST request to `/v2/databases/$DATABASE_ID/users/$USERNAME/reset_auth`.  For `mysql` databases, the authentication method can be specifying by including a key in the JSON body called `mysql_settings` with the `auth_plugin` value specified.  The response will be a JSON object with a `user` key. This will be set to an object containing the standard database user attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**username** | **String** | The name of the database user. | [required] |
**databases_reset_auth_request** | [**DatabasesResetAuthRequest**](DatabasesResetAuthRequest.md) |  | [required] |

### Return type

[**crate::models::DatabasesAddUser201Response**](databases_add_user_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_cluster_size

> databases_update_cluster_size(database_cluster_uuid, database_cluster_resize)
Resize a Database Cluster

To resize a database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/resize`. The body of the request must specify both the size and num_nodes attributes. A successful request will receive a 202 Accepted status code with no body in response. Querying the database cluster will show that its status attribute will now be set to resizing. This will transition back to online when the resize operation has completed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**database_cluster_resize** | [**DatabaseClusterResize**](DatabaseClusterResize.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_eviction_policy

> databases_update_eviction_policy(database_cluster_uuid, databases_update_eviction_policy_request)
Configure the Eviction Policy for a Redis Cluster

To configure an eviction policy for an existing Redis cluster, send a PUT request to `/v2/databases/$DATABASE_ID/eviction_policy` specifying the desired policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**databases_update_eviction_policy_request** | [**DatabasesUpdateEvictionPolicyRequest**](DatabasesUpdateEvictionPolicyRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_firewall_rules

> databases_update_firewall_rules(database_cluster_uuid, databases_update_firewall_rules_request)
Update Firewall Rules (Trusted Sources) for a Database

To update a database cluster's firewall rules (known as \"trusted sources\" in the control panel), send a PUT request to `/v2/databases/$DATABASE_ID/firewall` specifying which resources should be able to open connections to the database. You may limit connections to specific Droplets, Kubernetes clusters, or IP addresses. When a tag is provided, any Droplet or Kubernetes node with that tag applied to it will have access. The firewall is limited to 100 rules (or trusted sources). When possible, we recommend [placing your databases into a VPC network](https://www.digitalocean.com/docs/networking/vpc/) to limit access to them instead of using a firewall. A successful

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**databases_update_firewall_rules_request** | [**DatabasesUpdateFirewallRulesRequest**](DatabasesUpdateFirewallRulesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_maintenance_window

> databases_update_maintenance_window(database_cluster_uuid, database_maintenance_window)
Configure a Database Cluster's Maintenance Window

To configure the window when automatic maintenance should be performed for a database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/maintenance`. A successful request will receive a 204 No Content status code with no body in response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**database_maintenance_window** | Option<[**DatabaseMaintenanceWindow**](DatabaseMaintenanceWindow.md)> |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_online_migration

> crate::models::OnlineMigration databases_update_online_migration(database_cluster_uuid, source_database)
Start an Online Migration

To start an online migration, send a PUT request to `/v2/databases/$DATABASE_ID/online-migration` endpoint. Migrating a cluster establishes a connection with an existing cluster and replicates its contents to the target cluster. Online migration is only available for MySQL, PostgreSQL, and Redis clusters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**source_database** | [**SourceDatabase**](SourceDatabase.md) |  | [required] |

### Return type

[**crate::models::OnlineMigration**](online_migration.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_region

> databases_update_region(database_cluster_uuid, databases_update_region_request)
Migrate a Database Cluster to a New Region

To migrate a database cluster to a new region, send a `PUT` request to `/v2/databases/$DATABASE_ID/migrate`. The body of the request must specify a `region` attribute.  A successful request will receive a 202 Accepted status code with no body in response. Querying the database cluster will show that its `status` attribute will now be set to `migrating`. This will transition back to `online` when the migration has completed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**databases_update_region_request** | [**DatabasesUpdateRegionRequest**](DatabasesUpdateRegionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_sql_mode

> databases_update_sql_mode(database_cluster_uuid, sql_mode)
Update SQL Mode for a Cluster

To configure the SQL modes for an existing MySQL cluster, send a PUT request to `/v2/databases/$DATABASE_ID/sql_mode` specifying the desired modes. See the official MySQL 8 documentation for a [full list of supported SQL modes](https://dev.mysql.com/doc/refman/8.0/en/sql-mode.html#sql-mode-full). A successful request will receive a 204 No Content status code with no body in response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_cluster_uuid** | **uuid::Uuid** | A unique identifier for a database cluster. | [required] |
**sql_mode** | [**SqlMode**](SqlMode.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

