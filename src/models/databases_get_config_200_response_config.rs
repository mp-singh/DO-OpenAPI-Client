/*
 * DigitalOcean API
 *
 * # Introduction  The DigitalOcean API allows you to manage Droplets and resources within the DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.  All of the functionality that you are familiar with in the DigitalOcean control panel is also available through the API, allowing you to script the complex actions that your situation requires.  The API documentation will start with a general overview about the design and technology that has been implemented, followed by reference information about specific endpoints.  ## Requests  Any tool that is fluent in HTTP can communicate with the API simply by requesting the correct URI. Requests should be made using the HTTPS protocol so that traffic is encrypted. The interface responds to different methods depending on the action required.  |Method|Usage| |--- |--- | |GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.| |DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.| |PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.| |PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is is a set of instructions on how to modify a resource updating only specific attributes.| |POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.| |HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|   ## HTTP Statuses  Along with the HTTP methods that the API responds to, it will also return standard HTTP statuses, including error codes.  In the event of a problem, the status will contain the error code, while the body of the response will usually contain additional information about the problem that was encountered.  In general, if the status returned is in the 200 range, it indicates that the request was fulfilled successfully and that no error was encountered.  Return codes in the 400 range typically indicate that there was an issue with the request that was sent. Among other things, this could mean that you did not authenticate correctly, that you are requesting an action that you do not have authorization for, that the object you are requesting does not exist, or that your request is malformed.  If you receive a status in the 500 range, this generally indicates a server-side problem. This means that we are having an issue on our end and cannot fulfill your request currently.  400 and 500 level error responses will include a JSON object in their body, including the following attributes:  |Name|Type|Description| |--- |--- |--- | |id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"| |message|string|A message providing additional information about the error, including details to help resolve it when possible.| |request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|  ### Example Error Response  ```     HTTP/1.1 403 Forbidden     {       \"id\":       \"forbidden\",       \"message\":  \"You do not have access for the attempted action.\"     } ```  ## Responses  When a request is successful, a response body will typically be sent back in the form of a JSON object. An exception to this is when a DELETE request is processed, which will result in a successful HTTP 204 status and an empty response body.  Inside of this JSON object, the resource root that was the target of the request will be set as the key. This will be the singular form of the word if the request operated on a single object, and the plural form of the word if a collection was processed.  For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you will get back an object with a key called \"`droplet`\". However, if you send the GET request to the general collection at `/v2/droplets`, you will get back an object with a key called \"`droplets`\".  The value of these keys will generally be a JSON object for a request on a single object and an array of objects for a request on a collection of objects.  ### Response for a Single Object  ```     {         \"droplet\": {             \"name\": \"example.com\"             . . .         }     } ```  ### Response for an Object Collection  ```     {         \"droplets\": [             {                 \"name\": \"example.com\"                 . . .             },             {                 \"name\": \"second.com\"                 . . .             }         ]     } ```  ## Meta  In addition to the main resource root, the response may also contain a `meta` object. This object contains information about the response itself.  The `meta` object contains a `total` key that is set to the total number of objects returned by the request. This has implications on the `links` object and pagination.  The `meta` object will only be displayed when it has a value. Currently, the `meta` object will have a value when a request is made on a collection (like `droplets` or `domains`).   ### Sample Meta Object  ```     {         . . .         \"meta\": {             \"total\": 43         }         . . .     } ```  ## Links & Pagination  The `links` object is returned as part of the response body when pagination is enabled. By default, 20 objects are returned per page. If the response contains 20 objects or fewer, no `links` object will be returned. If the response contains more than 20 objects, the first 20 will be returned along with the `links` object.  You can request a different pagination limit or force pagination by appending `?per_page=` to the request with the number of items you would like per page. For instance, to show only two results per page, you could add `?per_page=2` to the end of your query. The maximum number of results per page is 200.  The `links` object contains a `pages` object. The `pages` object, in turn, contains keys indicating the relationship of additional pages. The values of these are the URLs of the associated pages. The keys will be one of the following:  *   **first**: The URI of the first page of results. *   **prev**: The URI of the previous sequential page of results. *   **next**: The URI of the next sequential page of results. *   **last**: The URI of the last page of results.  The `pages` object will only include the links that make sense. So for the first page of results, no `first` or `prev` links will ever be set. This convention holds true in other situations where a link would not make sense.  ### Sample Links Object  ```     {         . . .         \"links\": {             \"pages\": {                 \"last\": \"https://api.digitalocean.com/v2/images?page=2\",                 \"next\": \"https://api.digitalocean.com/v2/images?page=2\"             }         }         . . .     } ```  ## Rate Limit  Requests through the API are rate limited per OAuth token. Current rate limits:  *   5,000 requests per hour *   250 requests per minute (5% of the hourly total)  Once you exceed either limit, you will be rate limited until the next cycle starts. Space out any requests that you would otherwise issue in bursts for the best results.  The rate limiting information is contained within the response headers of each request. The relevant headers are:  *   **RateLimit-Limit**: The number of requests that can be made per hour. *   **RateLimit-Remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire. *   **RateLimit-Reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.  As long as the `RateLimit-Remaining` count is above zero, you will be able to make additional requests.  The way that a request expires and is removed from the current limit count is important to understand. Rather than counting all of the requests for an hour and resetting the `RateLimit-Remaining` value at the end of the hour, each request instead has its own timer.  This means that each request contributes toward the `RateLimit-Remaining` count for one complete hour after the request is made. When that request's timer runs out, it is no longer counted towards the request limit.  This has implications on the meaning of the `RateLimit-Reset` header as well. Because the entire rate limit is not reset at one time, the value of this header is set to the time when the _oldest_ request will expire.  Keep this in mind if you see your `RateLimit-Reset` value change, but not move an entire hour into the future.  If the `RateLimit-Remaining` reaches zero, subsequent requests will receive a 429 error code until the request reset has been reached. You can see the format of the response in the examples.  **Note:** The following endpoints have special rate limit requirements that are independent of the limits defined above.  *   Only 12 `POST` requests to the `/v2/floating_ips` endpoint to create Floating IPs can be made per 60 seconds. *   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds.  ### Sample Rate Limit Headers  ```     . . .     RateLimit-Limit: 1200     RateLimit-Remaining: 1193     RateLimit-Reset: 1402425459     . . . ```  ### Sample Rate Exceeded Response  ```     429 Too Many Requests     {             id: \"too_many_requests\",             message: \"API Rate limit exceeded.\"     } ```  ## Curl Examples  Throughout this document, some example API requests will be given using the `curl` command. This will allow us to demonstrate the various endpoints in a simple, textual format.      These examples assume that you are using a Linux or macOS command line. To run these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:  * For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1) to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).  * For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2) to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).  * WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine. Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10),  then follow this API documentation normally.  The names of account-specific references (like Droplet IDs, for instance) will be represented by variables. For instance, a Droplet ID may be represented by a variable called `$DROPLET_ID`. You can set the associated variables in your environment if you wish to use the examples without modification.  The first variable that you should set to get started is your OAuth authorization token. The next section will go over the details of this, but you can set an environmental variable for it now.  Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications) section of the DigitalOcean control panel. Use an existing token if you have saved one, or generate a new token with the \"Generate new token\" button. Copy the generated token and use it to set and export the TOKEN variable in your environment as the example shows.  You may also wish to set some other variables now or as you go along. For example, you may wish to set the `DROPLET_ID` variable to one of your Droplet IDs since this will be used frequently in the API.  If you are following along, make sure you use a Droplet ID that you control so that your commands will execute correctly.  If you need access to the headers of a response through `curl`, you can pass the `-i` flag to display the header information along with the body. If you are only interested in the header, you can instead pass the `-I` flag, which will exclude the response body entirely.   ### Set and Export your OAuth Token  ``` export DIGITALOCEAN_TOKEN=your_token_here ```  ### Set and Export a Variable  ``` export DROPLET_ID=1111111 ```  ## Parameters  There are two different ways to pass parameters in a request with the API.  When passing parameters to create or update an object, parameters should be passed as a JSON object containing the appropriate attribute names and values as key-value pairs. When you use this format, you should specify that you are sending a JSON object in the header. This is done by setting the `Content-Type` header to `application/json`. This ensures that your request is interpreted correctly.  When passing parameters to filter a response on GET requests, parameters can be passed using standard query attributes. In this case, the parameters would be embedded into the URI itself by appending a `?` to the end of the URI and then setting each attribute with an equal sign. Attributes can be separated with a `&`. Tools like `curl` can create the appropriate URI when given parameters and values; this can also be done using the `-F` flag and then passing the key and value as an argument. The argument should take the form of a quoted string with the attribute being set to a value with an equal sign.  ### Pass Parameters as a JSON Object  ```     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\         -H \"Content-Type: application/json\" \\         -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\         -X POST \"https://api.digitalocean.com/v2/domains\" ```  ### Pass Filter Parameters as a Query String  ```      curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\          -X GET \\          \"https://api.digitalocean.com/v2/images?private=true\" ```  ## Cross Origin Resource Sharing  In order to make requests to the API from other domains, the API implements Cross Origin Resource Sharing (CORS) support.  CORS support is generally used to create AJAX requests outside of the domain that the request originated from. This is necessary to implement projects like control panels utilizing the API. This tells the browser that it can send requests to an outside domain.  The procedure that the browser initiates in order to perform these actions (other than GET requests) begins by sending a \"preflight\" request. This sets the `Origin` header and uses the `OPTIONS` method. The server will reply back with the methods it allows and some of the limits it imposes. The client then sends the actual request if it falls within the allowed constraints.  This process is usually done in the background by the browser, but you can use curl to emulate this process using the example provided. The headers that will be set to show the constraints are:  *   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header. *   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods. *   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain. *   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent. *   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.  You should not need to be concerned with the details of these headers, because the browser will typically do all of the work for you. 
 *
 * The version of the OpenAPI document: 2.0
 * Contact: api-engineering@digitalocean.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatabasesGetConfig200ResponseConfig {
    /// The hour of day (in UTC) when backup for the service starts. New backup only starts if previous backup has already completed.
    #[serde(rename = "backup_hour", skip_serializing_if = "Option::is_none")]
    pub backup_hour: Option<i32>,
    /// The minute of the backup hour when backup for the service starts. New backup is only started if previous backup has already completed.
    #[serde(rename = "backup_minute", skip_serializing_if = "Option::is_none")]
    pub backup_minute: Option<i32>,
    /// Global SQL mode. If empty, uses MySQL server defaults. Must only include uppercase alphabetic characters, underscores, and commas.
    #[serde(rename = "sql_mode", skip_serializing_if = "Option::is_none")]
    pub sql_mode: Option<String>,
    /// The number of seconds that the mysqld server waits for a connect packet before responding with bad handshake.
    #[serde(rename = "connect_timeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<i32>,
    /// Default server time zone, in the form of an offset from UTC (from -12:00 to +12:00), a time zone name (EST), or 'SYSTEM' to use the MySQL server default.
    #[serde(rename = "default_time_zone", skip_serializing_if = "Option::is_none")]
    pub default_time_zone: Option<String>,
    /// The maximum permitted result length, in bytes, for the GROUP_CONCAT() function.
    #[serde(rename = "group_concat_max_len", skip_serializing_if = "Option::is_none")]
    pub group_concat_max_len: Option<i32>,
    /// The time, in seconds, before cached statistics expire.
    #[serde(rename = "information_schema_stats_expiry", skip_serializing_if = "Option::is_none")]
    pub information_schema_stats_expiry: Option<i32>,
    /// The minimum length of words that an InnoDB FULLTEXT index stores.
    #[serde(rename = "innodb_ft_min_token_size", skip_serializing_if = "Option::is_none")]
    pub innodb_ft_min_token_size: Option<i32>,
    /// The InnoDB FULLTEXT index stopword list for all InnoDB tables.
    #[serde(rename = "innodb_ft_server_stopword_table", skip_serializing_if = "Option::is_none")]
    pub innodb_ft_server_stopword_table: Option<String>,
    /// The time, in seconds, that an InnoDB transaction waits for a row lock. before giving up.
    #[serde(rename = "innodb_lock_wait_timeout", skip_serializing_if = "Option::is_none")]
    pub innodb_lock_wait_timeout: Option<i32>,
    /// The size of the buffer, in bytes, that InnoDB uses to write to the log files. on disk.
    #[serde(rename = "innodb_log_buffer_size", skip_serializing_if = "Option::is_none")]
    pub innodb_log_buffer_size: Option<i32>,
    /// The upper limit, in bytes, of the size of the temporary log files used during online DDL operations for InnoDB tables.
    #[serde(rename = "innodb_online_alter_log_max_size", skip_serializing_if = "Option::is_none")]
    pub innodb_online_alter_log_max_size: Option<i32>,
    /// When enabled, records information about all deadlocks in InnoDB user transactions  in the error log. Disabled by default.
    #[serde(rename = "innodb_print_all_deadlocks", skip_serializing_if = "Option::is_none")]
    pub innodb_print_all_deadlocks: Option<bool>,
    /// When enabled, transaction timeouts cause InnoDB to abort and roll back the entire transaction.
    #[serde(rename = "innodb_rollback_on_timeout", skip_serializing_if = "Option::is_none")]
    pub innodb_rollback_on_timeout: Option<bool>,
    /// The time, in seconds, the server waits for activity on an interactive. connection before closing it.
    #[serde(rename = "interactive_timeout", skip_serializing_if = "Option::is_none")]
    pub interactive_timeout: Option<i32>,
    /// The storage engine for in-memory internal temporary tables.
    #[serde(rename = "internal_tmp_mem_storage_engine", skip_serializing_if = "Option::is_none")]
    pub internal_tmp_mem_storage_engine: Option<InternalTmpMemStorageEngine>,
    /// The time, in seconds, to wait for more data from an existing connection. aborting the read.
    #[serde(rename = "net_read_timeout", skip_serializing_if = "Option::is_none")]
    pub net_read_timeout: Option<i32>,
    /// The number of seconds to wait for a block to be written to a connection before aborting the write.
    #[serde(rename = "net_write_timeout", skip_serializing_if = "Option::is_none")]
    pub net_write_timeout: Option<i32>,
    /// Require primary key to be defined for new tables or old tables modified with ALTER TABLE and fail if missing. It is recommended to always have primary keys because various functionality may break if any large table is missing them.
    #[serde(rename = "sql_require_primary_key", skip_serializing_if = "Option::is_none")]
    pub sql_require_primary_key: Option<bool>,
    /// The number of seconds the server waits for activity on a noninteractive connection before closing it.
    #[serde(rename = "wait_timeout", skip_serializing_if = "Option::is_none")]
    pub wait_timeout: Option<i32>,
    /// The size of the largest message, in bytes, that can be received by the server. Default is 67108864 (64M).
    #[serde(rename = "max_allowed_packet", skip_serializing_if = "Option::is_none")]
    pub max_allowed_packet: Option<i32>,
    /// The maximum size, in bytes, of internal in-memory tables. Also set tmp_table_size. Default is 16777216 (16M)
    #[serde(rename = "max_heap_table_size", skip_serializing_if = "Option::is_none")]
    pub max_heap_table_size: Option<i32>,
    /// The sort buffer size, in bytes, for ORDER BY optimization. Default is 262144. (256K).
    #[serde(rename = "sort_buffer_size", skip_serializing_if = "Option::is_none")]
    pub sort_buffer_size: Option<i32>,
    /// The maximum size, in bytes, of internal in-memory tables. Also set max_heap_table_size. Default is 16777216 (16M).
    #[serde(rename = "tmp_table_size", skip_serializing_if = "Option::is_none")]
    pub tmp_table_size: Option<i32>,
    /// When enabled, captures slow queries. When disabled, also truncates the mysql.slow_log table. Default is false.
    #[serde(rename = "slow_query_log", skip_serializing_if = "Option::is_none")]
    pub slow_query_log: Option<bool>,
    /// The time, in seconds, for a query to take to execute before  being captured by slow_query_logs. Default is 10 seconds.
    #[serde(rename = "long_query_time", skip_serializing_if = "Option::is_none")]
    pub long_query_time: Option<f32>,
    /// The minimum amount of time, in seconds, to keep binlog entries before deletion.  This may be extended for services that require binlog entries for longer than the default, for example if using the MySQL Debezium Kafka connector.
    #[serde(rename = "binlog_retention_period", skip_serializing_if = "Option::is_none")]
    pub binlog_retention_period: Option<f32>,
    /// Specifies the maximum age (in transactions) that a table's pg_class.relfrozenxid field can attain before a VACUUM operation is forced to prevent transaction ID wraparound within the table. Note that the system will launch autovacuum processes to prevent wraparound even when autovacuum is otherwise disabled. This parameter will cause the server to be restarted.
    #[serde(rename = "autovacuum_freeze_max_age", skip_serializing_if = "Option::is_none")]
    pub autovacuum_freeze_max_age: Option<i32>,
    /// Specifies the maximum number of autovacuum processes (other than the autovacuum launcher) that may be running at any one time. The default is three. This parameter can only be set at server start.
    #[serde(rename = "autovacuum_max_workers", skip_serializing_if = "Option::is_none")]
    pub autovacuum_max_workers: Option<i32>,
    /// Specifies the minimum delay, in seconds, between autovacuum runs on any given database. The default is one minute.
    #[serde(rename = "autovacuum_naptime", skip_serializing_if = "Option::is_none")]
    pub autovacuum_naptime: Option<i32>,
    /// Specifies the minimum number of updated or deleted tuples needed to trigger a VACUUM in any one table. The default is 50 tuples.
    #[serde(rename = "autovacuum_vacuum_threshold", skip_serializing_if = "Option::is_none")]
    pub autovacuum_vacuum_threshold: Option<i32>,
    /// Specifies the minimum number of inserted, updated, or deleted tuples needed to trigger an ANALYZE in any one table. The default is 50 tuples.
    #[serde(rename = "autovacuum_analyze_threshold", skip_serializing_if = "Option::is_none")]
    pub autovacuum_analyze_threshold: Option<i32>,
    /// Specifies a fraction, in a decimal value, of the table size to add to autovacuum_vacuum_threshold when deciding whether to trigger a VACUUM. The default is 0.2 (20% of table size).
    #[serde(rename = "autovacuum_vacuum_scale_factor", skip_serializing_if = "Option::is_none")]
    pub autovacuum_vacuum_scale_factor: Option<f32>,
    /// Specifies a fraction, in a decimal value, of the table size to add to autovacuum_analyze_threshold when deciding whether to trigger an ANALYZE. The default is 0.2 (20% of table size).
    #[serde(rename = "autovacuum_analyze_scale_factor", skip_serializing_if = "Option::is_none")]
    pub autovacuum_analyze_scale_factor: Option<f32>,
    /// Specifies the cost delay value, in milliseconds, that will be used in automatic VACUUM operations. If -1, uses the regular vacuum_cost_delay value, which is 20 milliseconds.
    #[serde(rename = "autovacuum_vacuum_cost_delay", skip_serializing_if = "Option::is_none")]
    pub autovacuum_vacuum_cost_delay: Option<i32>,
    /// Specifies the cost limit value that will be used in automatic VACUUM operations. If -1 is specified (which is the default), the regular vacuum_cost_limit value will be used.
    #[serde(rename = "autovacuum_vacuum_cost_limit", skip_serializing_if = "Option::is_none")]
    pub autovacuum_vacuum_cost_limit: Option<i32>,
    /// Specifies the delay, in milliseconds, between activity rounds for the background writer. Default is 200 ms.
    #[serde(rename = "bgwriter_delay", skip_serializing_if = "Option::is_none")]
    pub bgwriter_delay: Option<i32>,
    /// The amount of kilobytes that need to be written by the background writer before attempting to force the OS to issue these writes to underlying storage. Specified in kilobytes, default is 512.  Setting of 0 disables forced writeback.
    #[serde(rename = "bgwriter_flush_after", skip_serializing_if = "Option::is_none")]
    pub bgwriter_flush_after: Option<i32>,
    /// The maximum number of buffers that the background writer can write. Setting this to zero disables background writing. Default is 100.
    #[serde(rename = "bgwriter_lru_maxpages", skip_serializing_if = "Option::is_none")]
    pub bgwriter_lru_maxpages: Option<i32>,
    /// The average recent need for new buffers is multiplied by bgwriter_lru_multiplier to arrive at an estimate of the number that will be needed during the next round, (up to bgwriter_lru_maxpages). 1.0 represents a “just in time” policy of writing exactly the number of buffers predicted to be needed. Larger values provide some cushion against spikes in demand, while smaller values intentionally leave writes to be done by server processes. The default is 2.0.
    #[serde(rename = "bgwriter_lru_multiplier", skip_serializing_if = "Option::is_none")]
    pub bgwriter_lru_multiplier: Option<f32>,
    /// The amount of time, in milliseconds, to wait on a lock before checking to see if there is a deadlock condition.
    #[serde(rename = "deadlock_timeout", skip_serializing_if = "Option::is_none")]
    pub deadlock_timeout: Option<i32>,
    /// Specifies the default TOAST compression method for values of compressible columns (the default is lz4).
    #[serde(rename = "default_toast_compression", skip_serializing_if = "Option::is_none")]
    pub default_toast_compression: Option<DefaultToastCompression>,
    /// Time out sessions with open transactions after this number of milliseconds
    #[serde(rename = "idle_in_transaction_session_timeout", skip_serializing_if = "Option::is_none")]
    pub idle_in_transaction_session_timeout: Option<i32>,
    /// Activates, in a boolean, the system-wide use of Just-in-Time Compilation (JIT).
    #[serde(rename = "jit", skip_serializing_if = "Option::is_none")]
    pub jit: Option<bool>,
    /// Causes each action executed by autovacuum to be logged if it ran for at least the specified number of milliseconds. Setting this to zero logs all autovacuum actions. Minus-one (the default) disables logging autovacuum actions.
    #[serde(rename = "log_autovacuum_min_duration", skip_serializing_if = "Option::is_none")]
    pub log_autovacuum_min_duration: Option<i32>,
    /// Controls the amount of detail written in the server log for each message that is logged.
    #[serde(rename = "log_error_verbosity", skip_serializing_if = "Option::is_none")]
    pub log_error_verbosity: Option<LogErrorVerbosity>,
    /// Selects one of the available log-formats. These can support popular log analyzers like pgbadger, pganalyze, etc.
    #[serde(rename = "log_line_prefix", skip_serializing_if = "Option::is_none")]
    pub log_line_prefix: Option<LogLinePrefix>,
    /// Log statements that take more than this number of milliseconds to run. If -1, disables.
    #[serde(rename = "log_min_duration_statement", skip_serializing_if = "Option::is_none")]
    pub log_min_duration_statement: Option<i32>,
    /// PostgreSQL maximum number of files that can be open per process.
    #[serde(rename = "max_files_per_process", skip_serializing_if = "Option::is_none")]
    pub max_files_per_process: Option<i32>,
    /// PostgreSQL maximum prepared transactions.
    #[serde(rename = "max_prepared_transactions", skip_serializing_if = "Option::is_none")]
    pub max_prepared_transactions: Option<i32>,
    /// PostgreSQL maximum predicate locks per transaction.
    #[serde(rename = "max_pred_locks_per_transaction", skip_serializing_if = "Option::is_none")]
    pub max_pred_locks_per_transaction: Option<i32>,
    /// PostgreSQL maximum locks per transaction.
    #[serde(rename = "max_locks_per_transaction", skip_serializing_if = "Option::is_none")]
    pub max_locks_per_transaction: Option<i32>,
    /// Maximum depth of the stack in bytes.
    #[serde(rename = "max_stack_depth", skip_serializing_if = "Option::is_none")]
    pub max_stack_depth: Option<i32>,
    /// Max standby archive delay in milliseconds.
    #[serde(rename = "max_standby_archive_delay", skip_serializing_if = "Option::is_none")]
    pub max_standby_archive_delay: Option<i32>,
    /// Max standby streaming delay in milliseconds.
    #[serde(rename = "max_standby_streaming_delay", skip_serializing_if = "Option::is_none")]
    pub max_standby_streaming_delay: Option<i32>,
    /// PostgreSQL maximum replication slots.
    #[serde(rename = "max_replication_slots", skip_serializing_if = "Option::is_none")]
    pub max_replication_slots: Option<i32>,
    /// PostgreSQL maximum logical replication workers (taken from the pool of max_parallel_workers).
    #[serde(rename = "max_logical_replication_workers", skip_serializing_if = "Option::is_none")]
    pub max_logical_replication_workers: Option<i32>,
    /// Sets the maximum number of workers that the system can support for parallel queries.
    #[serde(rename = "max_parallel_workers", skip_serializing_if = "Option::is_none")]
    pub max_parallel_workers: Option<i32>,
    /// Sets the maximum number of workers that can be started by a single Gather or Gather Merge node.
    #[serde(rename = "max_parallel_workers_per_gather", skip_serializing_if = "Option::is_none")]
    pub max_parallel_workers_per_gather: Option<i32>,
    /// Sets the maximum number of background processes that the system can support.
    #[serde(rename = "max_worker_processes", skip_serializing_if = "Option::is_none")]
    pub max_worker_processes: Option<i32>,
    /// Controls which role to use for pg_partman's scheduled background tasks. Must consist of alpha-numeric characters, dots, underscores, or dashes. May not start with dash or dot. Maximum of 64 characters.
    #[serde(rename = "pg_partman_bgw.role", skip_serializing_if = "Option::is_none")]
    pub pg_partman_bgw_period_role: Option<String>,
    /// Sets the time interval to run pg_partman's scheduled tasks.
    #[serde(rename = "pg_partman_bgw.interval", skip_serializing_if = "Option::is_none")]
    pub pg_partman_bgw_period_interval: Option<i32>,
    /// Controls which statements are counted. Specify 'top' to track top-level statements (those issued directly by clients), 'all' to also track nested statements (such as statements invoked within functions), or 'none' to disable statement statistics collection. The default value is top.
    #[serde(rename = "pg_stat_statements.track", skip_serializing_if = "Option::is_none")]
    pub pg_stat_statements_period_track: Option<PgStatStatementsPeriodTrack>,
    /// PostgreSQL temporary file limit in KiB. If -1, sets to unlimited.
    #[serde(rename = "temp_file_limit", skip_serializing_if = "Option::is_none")]
    pub temp_file_limit: Option<i32>,
    /// PostgreSQL service timezone
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Specifies the number of bytes reserved to track the currently executing command for each active session.
    #[serde(rename = "track_activity_query_size", skip_serializing_if = "Option::is_none")]
    pub track_activity_query_size: Option<i32>,
    /// Record commit time of transactions.
    #[serde(rename = "track_commit_timestamp", skip_serializing_if = "Option::is_none")]
    pub track_commit_timestamp: Option<TrackCommitTimestamp>,
    /// Enables tracking of function call counts and time used.
    #[serde(rename = "track_functions", skip_serializing_if = "Option::is_none")]
    pub track_functions: Option<TrackFunctions>,
    /// Enables timing of database I/O calls. This parameter is off by default, because it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms.
    #[serde(rename = "track_io_timing", skip_serializing_if = "Option::is_none")]
    pub track_io_timing: Option<TrackIoTiming>,
    /// PostgreSQL maximum WAL senders
    #[serde(rename = "max_wal_senders", skip_serializing_if = "Option::is_none")]
    pub max_wal_senders: Option<i32>,
    /// Terminate replication connections that are inactive for longer than this amount of time, in milliseconds. Setting this value to zero disables the timeout. Must be either 0 or between 5000 and 10800000.
    #[serde(rename = "wal_sender_timeout", skip_serializing_if = "Option::is_none")]
    pub wal_sender_timeout: Option<i32>,
    /// WAL flush interval in milliseconds. Note that setting this value to lower than the default 200ms may negatively impact performance
    #[serde(rename = "wal_writer_delay", skip_serializing_if = "Option::is_none")]
    pub wal_writer_delay: Option<i32>,
    /// Percentage of total RAM that the database server uses for shared memory buffers.  Valid range is 20-60 (float), which corresponds to 20% - 60%.  This setting adjusts the shared_buffers configuration value.
    #[serde(rename = "shared_buffers_percentage", skip_serializing_if = "Option::is_none")]
    pub shared_buffers_percentage: Option<f32>,
    #[serde(rename = "pgbouncer", skip_serializing_if = "Option::is_none")]
    pub pgbouncer: Option<Box<crate::models::Pgbouncer>>,
    /// The maximum amount of memory, in MB, used by a query operation (such as a sort or hash table) before writing to temporary disk files. Default is 1MB + 0.075% of total RAM (up to 32MB).
    #[serde(rename = "work_mem", skip_serializing_if = "Option::is_none")]
    pub work_mem: Option<i32>,
    #[serde(rename = "timescaledb", skip_serializing_if = "Option::is_none")]
    pub timescaledb: Option<Box<crate::models::Timescaledb>>,
    /// Synchronous replication type. Note that the service plan also needs to support synchronous replication.
    #[serde(rename = "synchronous_replication", skip_serializing_if = "Option::is_none")]
    pub synchronous_replication: Option<SynchronousReplication>,
    /// Enable the pg_stat_monitor extension. <b>Enabling this extension will cause the cluster to be restarted.</b> When this extension is enabled, pg_stat_statements results for utility commands are unreliable.
    #[serde(rename = "stat_monitor_enable", skip_serializing_if = "Option::is_none")]
    pub stat_monitor_enable: Option<bool>,
    #[serde(rename = "redis_maxmemory_policy", skip_serializing_if = "Option::is_none")]
    pub redis_maxmemory_policy: Option<crate::models::EvictionPolicyModel>,
    /// Set output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan.
    #[serde(rename = "redis_pubsub_client_output_buffer_limit", skip_serializing_if = "Option::is_none")]
    pub redis_pubsub_client_output_buffer_limit: Option<i32>,
    /// Set number of redis databases. Changing this will cause a restart of redis service.
    #[serde(rename = "redis_number_of_databases", skip_serializing_if = "Option::is_none")]
    pub redis_number_of_databases: Option<i32>,
    /// Redis IO thread count
    #[serde(rename = "redis_io_threads", skip_serializing_if = "Option::is_none")]
    pub redis_io_threads: Option<i32>,
    /// Counter logarithm factor for volatile-lfu and allkeys-lfu maxmemory-policies
    #[serde(rename = "redis_lfu_log_factor", skip_serializing_if = "Option::is_none")]
    pub redis_lfu_log_factor: Option<i32>,
    /// LFU maxmemory-policy counter decay time in minutes
    #[serde(rename = "redis_lfu_decay_time", skip_serializing_if = "Option::is_none")]
    pub redis_lfu_decay_time: Option<i32>,
    /// Require SSL to access Redis
    #[serde(rename = "redis_ssl", skip_serializing_if = "Option::is_none")]
    pub redis_ssl: Option<bool>,
    /// Redis idle connection timeout in seconds
    #[serde(rename = "redis_timeout", skip_serializing_if = "Option::is_none")]
    pub redis_timeout: Option<i32>,
    /// Set notify-keyspace-events option
    #[serde(rename = "redis_notify_keyspace_events", skip_serializing_if = "Option::is_none")]
    pub redis_notify_keyspace_events: Option<String>,
    /// When persistence is 'rdb', Redis does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked.
    #[serde(rename = "redis_persistence", skip_serializing_if = "Option::is_none")]
    pub redis_persistence: Option<RedisPersistence>,
    /// Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Redis configuration acl-pubsub-default.
    #[serde(rename = "redis_acl_channels_default", skip_serializing_if = "Option::is_none")]
    pub redis_acl_channels_default: Option<RedisAclChannelsDefault>,
}

impl DatabasesGetConfig200ResponseConfig {
    pub fn new() -> DatabasesGetConfig200ResponseConfig {
        DatabasesGetConfig200ResponseConfig {
            backup_hour: None,
            backup_minute: None,
            sql_mode: None,
            connect_timeout: None,
            default_time_zone: None,
            group_concat_max_len: None,
            information_schema_stats_expiry: None,
            innodb_ft_min_token_size: None,
            innodb_ft_server_stopword_table: None,
            innodb_lock_wait_timeout: None,
            innodb_log_buffer_size: None,
            innodb_online_alter_log_max_size: None,
            innodb_print_all_deadlocks: None,
            innodb_rollback_on_timeout: None,
            interactive_timeout: None,
            internal_tmp_mem_storage_engine: None,
            net_read_timeout: None,
            net_write_timeout: None,
            sql_require_primary_key: None,
            wait_timeout: None,
            max_allowed_packet: None,
            max_heap_table_size: None,
            sort_buffer_size: None,
            tmp_table_size: None,
            slow_query_log: None,
            long_query_time: None,
            binlog_retention_period: None,
            autovacuum_freeze_max_age: None,
            autovacuum_max_workers: None,
            autovacuum_naptime: None,
            autovacuum_vacuum_threshold: None,
            autovacuum_analyze_threshold: None,
            autovacuum_vacuum_scale_factor: None,
            autovacuum_analyze_scale_factor: None,
            autovacuum_vacuum_cost_delay: None,
            autovacuum_vacuum_cost_limit: None,
            bgwriter_delay: None,
            bgwriter_flush_after: None,
            bgwriter_lru_maxpages: None,
            bgwriter_lru_multiplier: None,
            deadlock_timeout: None,
            default_toast_compression: None,
            idle_in_transaction_session_timeout: None,
            jit: None,
            log_autovacuum_min_duration: None,
            log_error_verbosity: None,
            log_line_prefix: None,
            log_min_duration_statement: None,
            max_files_per_process: None,
            max_prepared_transactions: None,
            max_pred_locks_per_transaction: None,
            max_locks_per_transaction: None,
            max_stack_depth: None,
            max_standby_archive_delay: None,
            max_standby_streaming_delay: None,
            max_replication_slots: None,
            max_logical_replication_workers: None,
            max_parallel_workers: None,
            max_parallel_workers_per_gather: None,
            max_worker_processes: None,
            pg_partman_bgw_period_role: None,
            pg_partman_bgw_period_interval: None,
            pg_stat_statements_period_track: None,
            temp_file_limit: None,
            timezone: None,
            track_activity_query_size: None,
            track_commit_timestamp: None,
            track_functions: None,
            track_io_timing: None,
            max_wal_senders: None,
            wal_sender_timeout: None,
            wal_writer_delay: None,
            shared_buffers_percentage: None,
            pgbouncer: None,
            work_mem: None,
            timescaledb: None,
            synchronous_replication: None,
            stat_monitor_enable: None,
            redis_maxmemory_policy: None,
            redis_pubsub_client_output_buffer_limit: None,
            redis_number_of_databases: None,
            redis_io_threads: None,
            redis_lfu_log_factor: None,
            redis_lfu_decay_time: None,
            redis_ssl: None,
            redis_timeout: None,
            redis_notify_keyspace_events: None,
            redis_persistence: None,
            redis_acl_channels_default: None,
        }
    }
}

/// The storage engine for in-memory internal temporary tables.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InternalTmpMemStorageEngine {
    #[serde(rename = "TempTable")]
    TempTable,
    #[serde(rename = "MEMORY")]
    Memory,
}

impl Default for InternalTmpMemStorageEngine {
    fn default() -> InternalTmpMemStorageEngine {
        Self::TempTable
    }
}
/// Specifies the default TOAST compression method for values of compressible columns (the default is lz4).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultToastCompression {
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "pglz")]
    Pglz,
}

impl Default for DefaultToastCompression {
    fn default() -> DefaultToastCompression {
        Self::Lz4
    }
}
/// Controls the amount of detail written in the server log for each message that is logged.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogErrorVerbosity {
    #[serde(rename = "TERSE")]
    Terse,
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "VERBOSE")]
    Verbose,
}

impl Default for LogErrorVerbosity {
    fn default() -> LogErrorVerbosity {
        Self::Terse
    }
}
/// Selects one of the available log-formats. These can support popular log analyzers like pgbadger, pganalyze, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogLinePrefix {
    #[serde(rename = "pid=%p,user=%u,db=%d,app=%a,client=%h")]
    PidEqualPercentPCommaUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentH,
    #[serde(rename = "%m [%p] %q[user=%u,db=%d,app=%a]")]
    PercentMLeftSquareBracketPercentPRightSquareBracketPercentQLeftSquareBracketUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentARightSquareBracket,
    #[serde(rename = "%t [%p]: [%l-1] user=%u,db=%d,app=%a,client=%h")]
    PercentTLeftSquareBracketPercentPRightSquareBracketColonLeftSquareBracketPercentL1RightSquareBracketUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentH,
}

impl Default for LogLinePrefix {
    fn default() -> LogLinePrefix {
        Self::PidEqualPercentPCommaUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentH
    }
}
/// Controls which statements are counted. Specify 'top' to track top-level statements (those issued directly by clients), 'all' to also track nested statements (such as statements invoked within functions), or 'none' to disable statement statistics collection. The default value is top.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PgStatStatementsPeriodTrack {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "none")]
    None,
}

impl Default for PgStatStatementsPeriodTrack {
    fn default() -> PgStatStatementsPeriodTrack {
        Self::All
    }
}
/// Record commit time of transactions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackCommitTimestamp {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
}

impl Default for TrackCommitTimestamp {
    fn default() -> TrackCommitTimestamp {
        Self::Off
    }
}
/// Enables tracking of function call counts and time used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackFunctions {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "none")]
    None,
}

impl Default for TrackFunctions {
    fn default() -> TrackFunctions {
        Self::All
    }
}
/// Enables timing of database I/O calls. This parameter is off by default, because it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackIoTiming {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
}

impl Default for TrackIoTiming {
    fn default() -> TrackIoTiming {
        Self::Off
    }
}
/// Synchronous replication type. Note that the service plan also needs to support synchronous replication.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SynchronousReplication {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "quorum")]
    Quorum,
}

impl Default for SynchronousReplication {
    fn default() -> SynchronousReplication {
        Self::Off
    }
}
/// When persistence is 'rdb', Redis does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RedisPersistence {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "rdb")]
    Rdb,
}

impl Default for RedisPersistence {
    fn default() -> RedisPersistence {
        Self::Off
    }
}
/// Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Redis configuration acl-pubsub-default.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RedisAclChannelsDefault {
    #[serde(rename = "allchannels")]
    Allchannels,
    #[serde(rename = "resetchannels")]
    Resetchannels,
}

impl Default for RedisAclChannelsDefault {
    fn default() -> RedisAclChannelsDefault {
        Self::Allchannels
    }
}

