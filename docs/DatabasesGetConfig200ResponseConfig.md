# DatabasesGetConfig200ResponseConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_hour** | Option<**i32**> | The hour of day (in UTC) when backup for the service starts. New backup only starts if previous backup has already completed. | [optional]
**backup_minute** | Option<**i32**> | The minute of the backup hour when backup for the service starts. New backup is only started if previous backup has already completed. | [optional]
**sql_mode** | Option<**String**> | Global SQL mode. If empty, uses MySQL server defaults. Must only include uppercase alphabetic characters, underscores, and commas. | [optional]
**connect_timeout** | Option<**i32**> | The number of seconds that the mysqld server waits for a connect packet before responding with bad handshake. | [optional]
**default_time_zone** | Option<**String**> | Default server time zone, in the form of an offset from UTC (from -12:00 to +12:00), a time zone name (EST), or 'SYSTEM' to use the MySQL server default. | [optional]
**group_concat_max_len** | Option<**i32**> | The maximum permitted result length, in bytes, for the GROUP_CONCAT() function. | [optional]
**information_schema_stats_expiry** | Option<**i32**> | The time, in seconds, before cached statistics expire. | [optional]
**innodb_ft_min_token_size** | Option<**i32**> | The minimum length of words that an InnoDB FULLTEXT index stores. | [optional]
**innodb_ft_server_stopword_table** | Option<**String**> | The InnoDB FULLTEXT index stopword list for all InnoDB tables. | [optional]
**innodb_lock_wait_timeout** | Option<**i32**> | The time, in seconds, that an InnoDB transaction waits for a row lock. before giving up. | [optional]
**innodb_log_buffer_size** | Option<**i32**> | The size of the buffer, in bytes, that InnoDB uses to write to the log files. on disk. | [optional]
**innodb_online_alter_log_max_size** | Option<**i32**> | The upper limit, in bytes, of the size of the temporary log files used during online DDL operations for InnoDB tables. | [optional]
**innodb_print_all_deadlocks** | Option<**bool**> | When enabled, records information about all deadlocks in InnoDB user transactions  in the error log. Disabled by default. | [optional]
**innodb_rollback_on_timeout** | Option<**bool**> | When enabled, transaction timeouts cause InnoDB to abort and roll back the entire transaction. | [optional]
**interactive_timeout** | Option<**i32**> | The time, in seconds, the server waits for activity on an interactive. connection before closing it. | [optional]
**internal_tmp_mem_storage_engine** | Option<**String**> | The storage engine for in-memory internal temporary tables. | [optional]
**net_read_timeout** | Option<**i32**> | The time, in seconds, to wait for more data from an existing connection. aborting the read. | [optional]
**net_write_timeout** | Option<**i32**> | The number of seconds to wait for a block to be written to a connection before aborting the write. | [optional]
**sql_require_primary_key** | Option<**bool**> | Require primary key to be defined for new tables or old tables modified with ALTER TABLE and fail if missing. It is recommended to always have primary keys because various functionality may break if any large table is missing them. | [optional]
**wait_timeout** | Option<**i32**> | The number of seconds the server waits for activity on a noninteractive connection before closing it. | [optional]
**max_allowed_packet** | Option<**i32**> | The size of the largest message, in bytes, that can be received by the server. Default is 67108864 (64M). | [optional]
**max_heap_table_size** | Option<**i32**> | The maximum size, in bytes, of internal in-memory tables. Also set tmp_table_size. Default is 16777216 (16M) | [optional]
**sort_buffer_size** | Option<**i32**> | The sort buffer size, in bytes, for ORDER BY optimization. Default is 262144. (256K). | [optional]
**tmp_table_size** | Option<**i32**> | The maximum size, in bytes, of internal in-memory tables. Also set max_heap_table_size. Default is 16777216 (16M). | [optional]
**slow_query_log** | Option<**bool**> | When enabled, captures slow queries. When disabled, also truncates the mysql.slow_log table. Default is false. | [optional]
**long_query_time** | Option<**f32**> | The time, in seconds, for a query to take to execute before  being captured by slow_query_logs. Default is 10 seconds. | [optional]
**binlog_retention_period** | Option<**f32**> | The minimum amount of time, in seconds, to keep binlog entries before deletion.  This may be extended for services that require binlog entries for longer than the default, for example if using the MySQL Debezium Kafka connector. | [optional]
**autovacuum_freeze_max_age** | Option<**i32**> | Specifies the maximum age (in transactions) that a table's pg_class.relfrozenxid field can attain before a VACUUM operation is forced to prevent transaction ID wraparound within the table. Note that the system will launch autovacuum processes to prevent wraparound even when autovacuum is otherwise disabled. This parameter will cause the server to be restarted. | [optional]
**autovacuum_max_workers** | Option<**i32**> | Specifies the maximum number of autovacuum processes (other than the autovacuum launcher) that may be running at any one time. The default is three. This parameter can only be set at server start. | [optional]
**autovacuum_naptime** | Option<**i32**> | Specifies the minimum delay, in seconds, between autovacuum runs on any given database. The default is one minute. | [optional]
**autovacuum_vacuum_threshold** | Option<**i32**> | Specifies the minimum number of updated or deleted tuples needed to trigger a VACUUM in any one table. The default is 50 tuples. | [optional]
**autovacuum_analyze_threshold** | Option<**i32**> | Specifies the minimum number of inserted, updated, or deleted tuples needed to trigger an ANALYZE in any one table. The default is 50 tuples. | [optional]
**autovacuum_vacuum_scale_factor** | Option<**f32**> | Specifies a fraction, in a decimal value, of the table size to add to autovacuum_vacuum_threshold when deciding whether to trigger a VACUUM. The default is 0.2 (20% of table size). | [optional]
**autovacuum_analyze_scale_factor** | Option<**f32**> | Specifies a fraction, in a decimal value, of the table size to add to autovacuum_analyze_threshold when deciding whether to trigger an ANALYZE. The default is 0.2 (20% of table size). | [optional]
**autovacuum_vacuum_cost_delay** | Option<**i32**> | Specifies the cost delay value, in milliseconds, that will be used in automatic VACUUM operations. If -1, uses the regular vacuum_cost_delay value, which is 20 milliseconds. | [optional]
**autovacuum_vacuum_cost_limit** | Option<**i32**> | Specifies the cost limit value that will be used in automatic VACUUM operations. If -1 is specified (which is the default), the regular vacuum_cost_limit value will be used. | [optional]
**bgwriter_delay** | Option<**i32**> | Specifies the delay, in milliseconds, between activity rounds for the background writer. Default is 200 ms. | [optional]
**bgwriter_flush_after** | Option<**i32**> | The amount of kilobytes that need to be written by the background writer before attempting to force the OS to issue these writes to underlying storage. Specified in kilobytes, default is 512.  Setting of 0 disables forced writeback. | [optional]
**bgwriter_lru_maxpages** | Option<**i32**> | The maximum number of buffers that the background writer can write. Setting this to zero disables background writing. Default is 100. | [optional]
**bgwriter_lru_multiplier** | Option<**f32**> | The average recent need for new buffers is multiplied by bgwriter_lru_multiplier to arrive at an estimate of the number that will be needed during the next round, (up to bgwriter_lru_maxpages). 1.0 represents a “just in time” policy of writing exactly the number of buffers predicted to be needed. Larger values provide some cushion against spikes in demand, while smaller values intentionally leave writes to be done by server processes. The default is 2.0. | [optional]
**deadlock_timeout** | Option<**i32**> | The amount of time, in milliseconds, to wait on a lock before checking to see if there is a deadlock condition. | [optional]
**default_toast_compression** | Option<**String**> | Specifies the default TOAST compression method for values of compressible columns (the default is lz4). | [optional]
**idle_in_transaction_session_timeout** | Option<**i32**> | Time out sessions with open transactions after this number of milliseconds | [optional]
**jit** | Option<**bool**> | Activates, in a boolean, the system-wide use of Just-in-Time Compilation (JIT). | [optional]
**log_autovacuum_min_duration** | Option<**i32**> | Causes each action executed by autovacuum to be logged if it ran for at least the specified number of milliseconds. Setting this to zero logs all autovacuum actions. Minus-one (the default) disables logging autovacuum actions. | [optional]
**log_error_verbosity** | Option<**String**> | Controls the amount of detail written in the server log for each message that is logged. | [optional]
**log_line_prefix** | Option<**String**> | Selects one of the available log-formats. These can support popular log analyzers like pgbadger, pganalyze, etc. | [optional]
**log_min_duration_statement** | Option<**i32**> | Log statements that take more than this number of milliseconds to run. If -1, disables. | [optional]
**max_files_per_process** | Option<**i32**> | PostgreSQL maximum number of files that can be open per process. | [optional]
**max_prepared_transactions** | Option<**i32**> | PostgreSQL maximum prepared transactions. | [optional]
**max_pred_locks_per_transaction** | Option<**i32**> | PostgreSQL maximum predicate locks per transaction. | [optional]
**max_locks_per_transaction** | Option<**i32**> | PostgreSQL maximum locks per transaction. | [optional]
**max_stack_depth** | Option<**i32**> | Maximum depth of the stack in bytes. | [optional]
**max_standby_archive_delay** | Option<**i32**> | Max standby archive delay in milliseconds. | [optional]
**max_standby_streaming_delay** | Option<**i32**> | Max standby streaming delay in milliseconds. | [optional]
**max_replication_slots** | Option<**i32**> | PostgreSQL maximum replication slots. | [optional]
**max_logical_replication_workers** | Option<**i32**> | PostgreSQL maximum logical replication workers (taken from the pool of max_parallel_workers). | [optional]
**max_parallel_workers** | Option<**i32**> | Sets the maximum number of workers that the system can support for parallel queries. | [optional]
**max_parallel_workers_per_gather** | Option<**i32**> | Sets the maximum number of workers that can be started by a single Gather or Gather Merge node. | [optional]
**max_worker_processes** | Option<**i32**> | Sets the maximum number of background processes that the system can support. | [optional]
**pg_partman_bgw_period_role** | Option<**String**> | Controls which role to use for pg_partman's scheduled background tasks. Must consist of alpha-numeric characters, dots, underscores, or dashes. May not start with dash or dot. Maximum of 64 characters. | [optional]
**pg_partman_bgw_period_interval** | Option<**i32**> | Sets the time interval to run pg_partman's scheduled tasks. | [optional]
**pg_stat_statements_period_track** | Option<**String**> | Controls which statements are counted. Specify 'top' to track top-level statements (those issued directly by clients), 'all' to also track nested statements (such as statements invoked within functions), or 'none' to disable statement statistics collection. The default value is top. | [optional]
**temp_file_limit** | Option<**i32**> | PostgreSQL temporary file limit in KiB. If -1, sets to unlimited. | [optional]
**timezone** | Option<**String**> | PostgreSQL service timezone | [optional]
**track_activity_query_size** | Option<**i32**> | Specifies the number of bytes reserved to track the currently executing command for each active session. | [optional]
**track_commit_timestamp** | Option<**String**> | Record commit time of transactions. | [optional]
**track_functions** | Option<**String**> | Enables tracking of function call counts and time used. | [optional]
**track_io_timing** | Option<**String**> | Enables timing of database I/O calls. This parameter is off by default, because it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms. | [optional]
**max_wal_senders** | Option<**i32**> | PostgreSQL maximum WAL senders | [optional]
**wal_sender_timeout** | Option<**i32**> | Terminate replication connections that are inactive for longer than this amount of time, in milliseconds. Setting this value to zero disables the timeout. Must be either 0 or between 5000 and 10800000. | [optional]
**wal_writer_delay** | Option<**i32**> | WAL flush interval in milliseconds. Note that setting this value to lower than the default 200ms may negatively impact performance | [optional]
**shared_buffers_percentage** | Option<**f32**> | Percentage of total RAM that the database server uses for shared memory buffers.  Valid range is 20-60 (float), which corresponds to 20% - 60%.  This setting adjusts the shared_buffers configuration value. | [optional]
**pgbouncer** | Option<[**crate::models::Pgbouncer**](pgbouncer.md)> |  | [optional]
**work_mem** | Option<**i32**> | The maximum amount of memory, in MB, used by a query operation (such as a sort or hash table) before writing to temporary disk files. Default is 1MB + 0.075% of total RAM (up to 32MB). | [optional]
**timescaledb** | Option<[**crate::models::Timescaledb**](timescaledb.md)> |  | [optional]
**synchronous_replication** | Option<**String**> | Synchronous replication type. Note that the service plan also needs to support synchronous replication. | [optional]
**stat_monitor_enable** | Option<**bool**> | Enable the pg_stat_monitor extension. <b>Enabling this extension will cause the cluster to be restarted.</b> When this extension is enabled, pg_stat_statements results for utility commands are unreliable. | [optional]
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

