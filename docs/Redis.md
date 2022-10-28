# Redis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**redis_maxmemory_policy** | Option<[**crate::models::EvictionPolicyModel**](eviction_policy_model.md)> |  | [optional]
**redis_pubsub_client_output_buffer_limit** | Option<**i32**> | Set output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan. | [optional]
**redis_number_of_databases** | Option<**i32**> | Set number of redis databases. Changing this will cause a restart of redis service. | [optional]
**redis_io_threads** | Option<**i32**> | Redis IO thread count | [optional]
**redis_lfu_log_factor** | Option<**i32**> | Counter logarithm factor for volatile-lfu and allkeys-lfu maxmemory-policies | [optional][default to 10]
**redis_lfu_decay_time** | Option<**i32**> | LFU maxmemory-policy counter decay time in minutes | [optional][default to 1]
**redis_ssl** | Option<**bool**> | Require SSL to access Redis | [optional][default to true]
**redis_timeout** | Option<**i32**> | Redis idle connection timeout in seconds | [optional][default to 300]
**redis_notify_keyspace_events** | Option<**String**> | Set notify-keyspace-events option | [optional][default to ]
**redis_persistence** | Option<**String**> | When persistence is 'rdb', Redis does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked. | [optional]
**redis_acl_channels_default** | Option<**String**> | Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Redis configuration acl-pubsub-default. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


