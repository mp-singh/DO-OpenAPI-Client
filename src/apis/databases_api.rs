/*
 * DigitalOcean API
 *
 * # Introduction  The DigitalOcean API allows you to manage Droplets and resources within the DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.  All of the functionality that you are familiar with in the DigitalOcean control panel is also available through the API, allowing you to script the complex actions that your situation requires.  The API documentation will start with a general overview about the design and technology that has been implemented, followed by reference information about specific endpoints.  ## Requests  Any tool that is fluent in HTTP can communicate with the API simply by requesting the correct URI. Requests should be made using the HTTPS protocol so that traffic is encrypted. The interface responds to different methods depending on the action required.  |Method|Usage| |--- |--- | |GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.| |DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.| |PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.| |PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is is a set of instructions on how to modify a resource updating only specific attributes.| |POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.| |HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|   ## HTTP Statuses  Along with the HTTP methods that the API responds to, it will also return standard HTTP statuses, including error codes.  In the event of a problem, the status will contain the error code, while the body of the response will usually contain additional information about the problem that was encountered.  In general, if the status returned is in the 200 range, it indicates that the request was fulfilled successfully and that no error was encountered.  Return codes in the 400 range typically indicate that there was an issue with the request that was sent. Among other things, this could mean that you did not authenticate correctly, that you are requesting an action that you do not have authorization for, that the object you are requesting does not exist, or that your request is malformed.  If you receive a status in the 500 range, this generally indicates a server-side problem. This means that we are having an issue on our end and cannot fulfill your request currently.  400 and 500 level error responses will include a JSON object in their body, including the following attributes:  |Name|Type|Description| |--- |--- |--- | |id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"| |message|string|A message providing additional information about the error, including details to help resolve it when possible.| |request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|  ### Example Error Response  ```     HTTP/1.1 403 Forbidden     {       \"id\":       \"forbidden\",       \"message\":  \"You do not have access for the attempted action.\"     } ```  ## Responses  When a request is successful, a response body will typically be sent back in the form of a JSON object. An exception to this is when a DELETE request is processed, which will result in a successful HTTP 204 status and an empty response body.  Inside of this JSON object, the resource root that was the target of the request will be set as the key. This will be the singular form of the word if the request operated on a single object, and the plural form of the word if a collection was processed.  For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you will get back an object with a key called \"`droplet`\". However, if you send the GET request to the general collection at `/v2/droplets`, you will get back an object with a key called \"`droplets`\".  The value of these keys will generally be a JSON object for a request on a single object and an array of objects for a request on a collection of objects.  ### Response for a Single Object  ```     {         \"droplet\": {             \"name\": \"example.com\"             . . .         }     } ```  ### Response for an Object Collection  ```     {         \"droplets\": [             {                 \"name\": \"example.com\"                 . . .             },             {                 \"name\": \"second.com\"                 . . .             }         ]     } ```  ## Meta  In addition to the main resource root, the response may also contain a `meta` object. This object contains information about the response itself.  The `meta` object contains a `total` key that is set to the total number of objects returned by the request. This has implications on the `links` object and pagination.  The `meta` object will only be displayed when it has a value. Currently, the `meta` object will have a value when a request is made on a collection (like `droplets` or `domains`).   ### Sample Meta Object  ```     {         . . .         \"meta\": {             \"total\": 43         }         . . .     } ```  ## Links & Pagination  The `links` object is returned as part of the response body when pagination is enabled. By default, 20 objects are returned per page. If the response contains 20 objects or fewer, no `links` object will be returned. If the response contains more than 20 objects, the first 20 will be returned along with the `links` object.  You can request a different pagination limit or force pagination by appending `?per_page=` to the request with the number of items you would like per page. For instance, to show only two results per page, you could add `?per_page=2` to the end of your query. The maximum number of results per page is 200.  The `links` object contains a `pages` object. The `pages` object, in turn, contains keys indicating the relationship of additional pages. The values of these are the URLs of the associated pages. The keys will be one of the following:  *   **first**: The URI of the first page of results. *   **prev**: The URI of the previous sequential page of results. *   **next**: The URI of the next sequential page of results. *   **last**: The URI of the last page of results.  The `pages` object will only include the links that make sense. So for the first page of results, no `first` or `prev` links will ever be set. This convention holds true in other situations where a link would not make sense.  ### Sample Links Object  ```     {         . . .         \"links\": {             \"pages\": {                 \"last\": \"https://api.digitalocean.com/v2/images?page=2\",                 \"next\": \"https://api.digitalocean.com/v2/images?page=2\"             }         }         . . .     } ```  ## Rate Limit  Requests through the API are rate limited per OAuth token. Current rate limits:  *   5,000 requests per hour *   250 requests per minute (5% of the hourly total)  Once you exceed either limit, you will be rate limited until the next cycle starts. Space out any requests that you would otherwise issue in bursts for the best results.  The rate limiting information is contained within the response headers of each request. The relevant headers are:  *   **RateLimit-Limit**: The number of requests that can be made per hour. *   **RateLimit-Remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire. *   **RateLimit-Reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.  As long as the `RateLimit-Remaining` count is above zero, you will be able to make additional requests.  The way that a request expires and is removed from the current limit count is important to understand. Rather than counting all of the requests for an hour and resetting the `RateLimit-Remaining` value at the end of the hour, each request instead has its own timer.  This means that each request contributes toward the `RateLimit-Remaining` count for one complete hour after the request is made. When that request's timer runs out, it is no longer counted towards the request limit.  This has implications on the meaning of the `RateLimit-Reset` header as well. Because the entire rate limit is not reset at one time, the value of this header is set to the time when the _oldest_ request will expire.  Keep this in mind if you see your `RateLimit-Reset` value change, but not move an entire hour into the future.  If the `RateLimit-Remaining` reaches zero, subsequent requests will receive a 429 error code until the request reset has been reached. You can see the format of the response in the examples.  **Note:** The following endpoints have special rate limit requirements that are independent of the limits defined above.  *   Only 12 `POST` requests to the `/v2/floating_ips` endpoint to create Floating IPs can be made per 60 seconds. *   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds.  ### Sample Rate Limit Headers  ```     . . .     RateLimit-Limit: 1200     RateLimit-Remaining: 1193     RateLimit-Reset: 1402425459     . . . ```  ### Sample Rate Exceeded Response  ```     429 Too Many Requests     {             id: \"too_many_requests\",             message: \"API Rate limit exceeded.\"     } ```  ## Curl Examples  Throughout this document, some example API requests will be given using the `curl` command. This will allow us to demonstrate the various endpoints in a simple, textual format.      These examples assume that you are using a Linux or macOS command line. To run these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:  * For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1) to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).  * For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2) to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).  * WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine. Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10),  then follow this API documentation normally.  The names of account-specific references (like Droplet IDs, for instance) will be represented by variables. For instance, a Droplet ID may be represented by a variable called `$DROPLET_ID`. You can set the associated variables in your environment if you wish to use the examples without modification.  The first variable that you should set to get started is your OAuth authorization token. The next section will go over the details of this, but you can set an environmental variable for it now.  Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications) section of the DigitalOcean control panel. Use an existing token if you have saved one, or generate a new token with the \"Generate new token\" button. Copy the generated token and use it to set and export the TOKEN variable in your environment as the example shows.  You may also wish to set some other variables now or as you go along. For example, you may wish to set the `DROPLET_ID` variable to one of your Droplet IDs since this will be used frequently in the API.  If you are following along, make sure you use a Droplet ID that you control so that your commands will execute correctly.  If you need access to the headers of a response through `curl`, you can pass the `-i` flag to display the header information along with the body. If you are only interested in the header, you can instead pass the `-I` flag, which will exclude the response body entirely.   ### Set and Export your OAuth Token  ``` export DIGITALOCEAN_TOKEN=your_token_here ```  ### Set and Export a Variable  ``` export DROPLET_ID=1111111 ```  ## Parameters  There are two different ways to pass parameters in a request with the API.  When passing parameters to create or update an object, parameters should be passed as a JSON object containing the appropriate attribute names and values as key-value pairs. When you use this format, you should specify that you are sending a JSON object in the header. This is done by setting the `Content-Type` header to `application/json`. This ensures that your request is interpreted correctly.  When passing parameters to filter a response on GET requests, parameters can be passed using standard query attributes. In this case, the parameters would be embedded into the URI itself by appending a `?` to the end of the URI and then setting each attribute with an equal sign. Attributes can be separated with a `&`. Tools like `curl` can create the appropriate URI when given parameters and values; this can also be done using the `-F` flag and then passing the key and value as an argument. The argument should take the form of a quoted string with the attribute being set to a value with an equal sign.  ### Pass Parameters as a JSON Object  ```     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\         -H \"Content-Type: application/json\" \\         -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\         -X POST \"https://api.digitalocean.com/v2/domains\" ```  ### Pass Filter Parameters as a Query String  ```      curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\          -X GET \\          \"https://api.digitalocean.com/v2/images?private=true\" ```  ## Cross Origin Resource Sharing  In order to make requests to the API from other domains, the API implements Cross Origin Resource Sharing (CORS) support.  CORS support is generally used to create AJAX requests outside of the domain that the request originated from. This is necessary to implement projects like control panels utilizing the API. This tells the browser that it can send requests to an outside domain.  The procedure that the browser initiates in order to perform these actions (other than GET requests) begins by sending a \"preflight\" request. This sets the `Origin` header and uses the `OPTIONS` method. The server will reply back with the methods it allows and some of the limits it imposes. The client then sends the actual request if it falls within the allowed constraints.  This process is usually done in the background by the browser, but you can use curl to emulate this process using the example provided. The headers that will be set to show the constraints are:  *   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header. *   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods. *   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain. *   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent. *   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.  You should not need to be concerned with the details of these headers, because the browser will typically do all of the work for you. 
 *
 * The version of the OpenAPI document: 2.0
 * Contact: api-engineering@digitalocean.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`databases_add`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesAddError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_add_connection_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesAddConnectionPoolError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_add_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesAddUserError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_create_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesCreateClusterError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_create_replica`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesCreateReplicaError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_connection_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteConnectionPoolError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_online_migration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteOnlineMigrationError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteUserError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_destroy_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDestroyClusterError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_destroy_replica`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDestroyReplicaError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_ca`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetCaError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetClusterError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetConfigError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_connection_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetConnectionPoolError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_eviction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetEvictionPolicyError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_migration_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetMigrationStatusError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_replica`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetReplicaError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_sql_mode`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetSqlModeError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetUserError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_backups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListBackupsError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_clusters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListClustersError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_connection_pools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListConnectionPoolsError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_firewall_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListFirewallRulesError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_options`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListOptionsError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_replicas`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListReplicasError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListUsersError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_patch_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesPatchConfigError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_reset_auth`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesResetAuthError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_cluster_size`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateClusterSizeError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_eviction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateEvictionPolicyError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_firewall_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateFirewallRulesError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_maintenance_window`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateMaintenanceWindowError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_online_migration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateOnlineMigrationError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_region`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateRegionError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_sql_mode`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateSqlModeError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// To add a new database to an existing cluster, send a POST request to `/v2/databases/$DATABASE_ID/dbs`.  Note: Database management is not supported for Redis clusters.  The response will be a JSON object with a key called `db`. The value of this will be an object that contains the standard attributes associated with a database. 
pub async fn databases_add(configuration: &configuration::Configuration, database_cluster_uuid: &str, database: crate::models::Database) -> Result<crate::models::DatabasesAdd201Response, Error<DatabasesAddError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/dbs", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&database);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesAddError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// For PostgreSQL database clusters, connection pools can be used to allow a database to share its idle connections. The popular PostgreSQL connection pooling utility PgBouncer is used to provide this service. [See here for more information](https://www.digitalocean.com/docs/databases/postgresql/how-to/manage-connection-pools/) about how and why to use PgBouncer connection pooling including details about the available transaction modes.  To add a new connection pool to a PostgreSQL database cluster, send a POST request to `/v2/databases/$DATABASE_ID/pools` specifying a name for the pool, the user to connect with, the database to connect to, as well as its desired size and transaction mode. 
pub async fn databases_add_connection_pool(configuration: &configuration::Configuration, database_cluster_uuid: &str, connection_pool: crate::models::ConnectionPool) -> Result<crate::models::DatabasesAddConnectionPool201Response, Error<DatabasesAddConnectionPoolError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&connection_pool);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesAddConnectionPoolError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To add a new database user, send a POST request to `/v2/databases/$DATABASE_ID/users` with the desired username.  Note: User management is not supported for Redis clusters.  When adding a user to a MySQL cluster, additional options can be configured in the `mysql_settings` object.  The response will be a JSON object with a key called `user`. The value of this will be an object that contains the standard attributes associated with a database user including its randomly generated password. 
pub async fn databases_add_user(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_user: crate::models::DatabaseUser) -> Result<crate::models::DatabasesAddUser201Response, Error<DatabasesAddUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&database_user);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesAddUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To create a database cluster, send a POST request to `/v2/databases`. The response will be a JSON object with a key called `database`. The value of this will be an object that contains the standard attributes associated with a database cluster. The initial value of the database cluster's `status` attribute will be `creating`. When the cluster is ready to receive traffic, this will transition to `online`. The embedded `connection` and `private_connection` objects will contain the information needed to access the database cluster. DigitalOcean managed PostgreSQL and MySQL database clusters take automated daily backups. To create a new database cluster based on a backup of an existing cluster, send a POST request to `/v2/databases`. In addition to the standard database cluster attributes, the JSON body must include a key named `backup_restore` with the name of the original database cluster and the timestamp of the backup to be restored. Creating a database from a backup is the same as forking a database in the control panel. Note: Backups are not supported for Redis clusters.
pub async fn databases_create_cluster(configuration: &configuration::Configuration, databases_create_cluster_request: crate::models::DatabasesCreateClusterRequest) -> Result<crate::models::DatabasesCreateCluster201Response, Error<DatabasesCreateClusterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&databases_create_cluster_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesCreateClusterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To create a read-only replica for a PostgreSQL or MySQL database cluster, send a POST request to `/v2/databases/$DATABASE_ID/replicas` specifying the name it should be given, the size of the node to be used, and the region where it will be located. **Note**: Read-only replicas are not supported for Redis clusters. The response will be a JSON object with a key called `replica`. The value of this will be an object that contains the standard attributes associated with a database replica. The initial value of the read-only replica's `status` attribute will be `forking`. When the replica is ready to receive traffic, this will transition to `active`.
pub async fn databases_create_replica(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_create_replica_request: Option<crate::models::DatabasesCreateReplicaRequest>) -> Result<crate::models::DatabasesCreateReplica201Response, Error<DatabasesCreateReplicaError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&databases_create_replica_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesCreateReplicaError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To delete a specific database, send a DELETE request to `/v2/databases/$DATABASE_ID/dbs/$DB_NAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.  Note: Database management is not supported for Redis clusters. 
pub async fn databases_delete(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_name: &str) -> Result<(), Error<DatabasesDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/dbs/{database_name}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), database_name=crate::apis::urlencode(database_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To delete a specific connection pool for a PostgreSQL database cluster, send a DELETE request to `/v2/databases/$DATABASE_ID/pools/$POOL_NAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 
pub async fn databases_delete_connection_pool(configuration: &configuration::Configuration, database_cluster_uuid: &str, pool_name: &str) -> Result<(), Error<DatabasesDeleteConnectionPoolError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools/{pool_name}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), pool_name=crate::apis::urlencode(pool_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesDeleteConnectionPoolError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To stop an online migration, send a DELETE request to `/v2/databases/$DATABASE_ID/online-migration/$MIGRATION_ID`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 
pub async fn databases_delete_online_migration(configuration: &configuration::Configuration, database_cluster_uuid: &str, migration_id: &str) -> Result<(), Error<DatabasesDeleteOnlineMigrationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/online-migration/{migration_id}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), migration_id=crate::apis::urlencode(migration_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesDeleteOnlineMigrationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To remove a specific database user, send a DELETE request to `/v2/databases/$DATABASE_ID/users/$USERNAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.  Note: User management is not supported for Redis clusters. 
pub async fn databases_delete_user(configuration: &configuration::Configuration, database_cluster_uuid: &str, username: &str) -> Result<(), Error<DatabasesDeleteUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users/{username}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), username=crate::apis::urlencode(username));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesDeleteUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To destroy a specific database, send a DELETE request to `/v2/databases/$DATABASE_ID`. A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.
pub async fn databases_destroy_cluster(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<(), Error<DatabasesDestroyClusterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesDestroyClusterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To destroy a specific read-only replica, send a DELETE request to `/v2/databases/$DATABASE_ID/replicas/$REPLICA_NAME`. **Note**: Read-only replicas are not supported for Redis clusters. A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.
pub async fn databases_destroy_replica(configuration: &configuration::Configuration, database_cluster_uuid: &str, replica_name: &str) -> Result<(), Error<DatabasesDestroyReplicaError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas/{replica_name}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), replica_name=crate::apis::urlencode(replica_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesDestroyReplicaError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To show information about an existing database cluster, send a GET request to `/v2/databases/$DATABASE_ID/dbs/$DB_NAME`.  Note: Database management is not supported for Redis clusters.  The response will be a JSON object with a `db` key. This will be set to an object containing the standard database attributes. 
pub async fn databases_get(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_name: &str) -> Result<crate::models::DatabasesAdd201Response, Error<DatabasesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/dbs/{database_name}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), database_name=crate::apis::urlencode(database_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To retrieve the public certificate used to secure the connection to the database cluster send a GET request to `/v2/databases/$DATABASE_ID/ca`.  The response will be a JSON object with a `ca` key. This will be set to an object containing the base64 encoding of the public key certificate. 
pub async fn databases_get_ca(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesGetCa200Response, Error<DatabasesGetCaError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/ca", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetCaError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To show information about an existing database cluster, send a GET request to `/v2/databases/$DATABASE_ID`. The response will be a JSON object with a database key. This will be set to an object containing the standard database cluster attributes. The embedded connection and private_connection objects will contain the information needed to access the database cluster. The embedded maintenance_window object will contain information about any scheduled maintenance for the database cluster.
pub async fn databases_get_cluster(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesCreateCluster201Response, Error<DatabasesGetClusterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetClusterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Shows configuration parameters for an existing database cluster by sending a GET request to `/v2/databases/$DATABASE_ID/config`. The response is a JSON object with a `config` key, which is set to an object containing any database configuration parameters. 
pub async fn databases_get_config(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesGetConfig200Response, Error<DatabasesGetConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/config", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To show information about an existing connection pool for a PostgreSQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/pools/$POOL_NAME`. The response will be a JSON object with a `pool` key.
pub async fn databases_get_connection_pool(configuration: &configuration::Configuration, database_cluster_uuid: &str, pool_name: &str) -> Result<crate::models::DatabasesAddConnectionPool201Response, Error<DatabasesGetConnectionPoolError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools/{pool_name}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), pool_name=crate::apis::urlencode(pool_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetConnectionPoolError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To retrieve the configured eviction policy for an existing Redis cluster, send a GET request to `/v2/databases/$DATABASE_ID/eviction_policy`. The response will be a JSON object with an `eviction_policy` key. This will be set to a string representing the eviction policy.
pub async fn databases_get_eviction_policy(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesGetEvictionPolicy200Response, Error<DatabasesGetEvictionPolicyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/eviction_policy", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetEvictionPolicyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To retrieve the status of an online migration, send a GET request to `/v2/databases/$DATABASE_ID/online-migration`. If a migration has completed, a 200 OK status is returned with no response body.
pub async fn databases_get_migration_status(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::OnlineMigration, Error<DatabasesGetMigrationStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/online-migration", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetMigrationStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To show information about an existing database replica, send a GET request to `/v2/databases/$DATABASE_ID/replicas/$REPLICA_NAME`. **Note**: Read-only replicas are not supported for Redis clusters. The response will be a JSON object with a `replica key`. This will be set to an object containing the standard database replica attributes.
pub async fn databases_get_replica(configuration: &configuration::Configuration, database_cluster_uuid: &str, replica_name: &str) -> Result<crate::models::DatabasesCreateReplica201Response, Error<DatabasesGetReplicaError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas/{replica_name}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), replica_name=crate::apis::urlencode(replica_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetReplicaError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To retrieve the configured SQL modes for an existing MySQL cluster, send a GET request to `/v2/databases/$DATABASE_ID/sql_mode`. The response will be a JSON object with a `sql_mode` key. This will be set to a string representing the configured SQL modes.
pub async fn databases_get_sql_mode(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::SqlMode, Error<DatabasesGetSqlModeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/sql_mode", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetSqlModeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To show information about an existing database user, send a GET request to `/v2/databases/$DATABASE_ID/users/$USERNAME`.  Note: User management is not supported for Redis clusters.  The response will be a JSON object with a `user` key. This will be set to an object containing the standard database user attributes.  For MySQL clusters, additional options will be contained in the mysql_settings object. 
pub async fn databases_get_user(configuration: &configuration::Configuration, database_cluster_uuid: &str, username: &str) -> Result<crate::models::DatabasesAddUser201Response, Error<DatabasesGetUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users/{username}", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), username=crate::apis::urlencode(username));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesGetUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To list all of the databases in a clusters, send a GET request to `/v2/databases/$DATABASE_ID/dbs`.  The result will be a JSON object with a `dbs` key. This will be set to an array of database objects, each of which will contain the standard database attributes.  Note: Database management is not supported for Redis clusters. 
pub async fn databases_list(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesList200Response, Error<DatabasesListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/dbs", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To list all of the available backups of a PostgreSQL or MySQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/backups`. **Note**: Backups are not supported for Redis clusters. The result will be a JSON object with a `backups key`. This will be set to an array of backup objects, each of which will contain the size of the backup and the timestamp at which it was created.
pub async fn databases_list_backups(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesListBackups200Response, Error<DatabasesListBackupsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/backups", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesListBackupsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To list all of the database clusters available on your account, send a GET request to `/v2/databases`. To limit the results to database clusters with a specific tag, include the `tag_name` query parameter set to the name of the tag. For example, `/v2/databases?tag_name=$TAG_NAME`. The result will be a JSON object with a `databases` key. This will be set to an array of database objects, each of which will contain the standard database attributes. The embedded `connection` and `private_connection` objects will contain the information needed to access the database cluster: The embedded `maintenance_window` object will contain information about any scheduled maintenance for the database cluster.
pub async fn databases_list_clusters(configuration: &configuration::Configuration, tag_name: Option<&str>) -> Result<crate::models::DatabasesListClusters200Response, Error<DatabasesListClustersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = tag_name {
        local_var_req_builder = local_var_req_builder.query(&[("tag_name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesListClustersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To list all of the connection pools available to a PostgreSQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/pools`. The result will be a JSON object with a `pools` key. This will be set to an array of connection pool objects.
pub async fn databases_list_connection_pools(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::ConnectionPools, Error<DatabasesListConnectionPoolsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesListConnectionPoolsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To list all of a database cluster's firewall rules (known as \"trusted sources\" in the control panel), send a GET request to `/v2/databases/$DATABASE_ID/firewall`. The result will be a JSON object with a `rules` key.
pub async fn databases_list_firewall_rules(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesListFirewallRules200Response, Error<DatabasesListFirewallRulesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/firewall", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesListFirewallRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To list all of the options available for the offered database engines, send a GET request to `/v2/databases/options`. The result will be a JSON object with an `options` key.
pub async fn databases_list_options(configuration: &configuration::Configuration, ) -> Result<crate::models::Options, Error<DatabasesListOptionsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/options", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesListOptionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To list all of the read-only replicas associated with a database cluster, send a GET request to `/v2/databases/$DATABASE_ID/replicas`. **Note**: Read-only replicas are not supported for Redis clusters. The result will be a JSON object with a `replicas` key. This will be set to an array of database replica objects, each of which will contain the standard database replica attributes.
pub async fn databases_list_replicas(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesListReplicas200Response, Error<DatabasesListReplicasError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesListReplicasError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To list all of the users for your database cluster, send a GET request to `/v2/databases/$DATABASE_ID/users`.  Note: User management is not supported for Redis clusters.  The result will be a JSON object with a `users` key. This will be set to an array of database user objects, each of which will contain the standard database user attributes.  For MySQL clusters, additional options will be contained in the mysql_settings object. 
pub async fn databases_list_users(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<crate::models::DatabasesListUsers200Response, Error<DatabasesListUsersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesListUsersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To update the configuration for an existing database cluster, send a PATCH request to `/v2/databases/$DATABASE_ID/config`. 
pub async fn databases_patch_config(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_config: crate::models::DatabaseConfig) -> Result<(), Error<DatabasesPatchConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/config", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&database_config);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesPatchConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To reset the password for a database user, send a POST request to `/v2/databases/$DATABASE_ID/users/$USERNAME/reset_auth`.  For `mysql` databases, the authentication method can be specifying by including a key in the JSON body called `mysql_settings` with the `auth_plugin` value specified.  The response will be a JSON object with a `user` key. This will be set to an object containing the standard database user attributes. 
pub async fn databases_reset_auth(configuration: &configuration::Configuration, database_cluster_uuid: &str, username: &str, databases_reset_auth_request: crate::models::DatabasesResetAuthRequest) -> Result<crate::models::DatabasesAddUser201Response, Error<DatabasesResetAuthError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users/{username}/reset_auth", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid), username=crate::apis::urlencode(username));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&databases_reset_auth_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesResetAuthError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To resize a database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/resize`. The body of the request must specify both the size and num_nodes attributes. A successful request will receive a 202 Accepted status code with no body in response. Querying the database cluster will show that its status attribute will now be set to resizing. This will transition back to online when the resize operation has completed.
pub async fn databases_update_cluster_size(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_cluster_resize: crate::models::DatabaseClusterResize) -> Result<(), Error<DatabasesUpdateClusterSizeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/resize", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&database_cluster_resize);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesUpdateClusterSizeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To configure an eviction policy for an existing Redis cluster, send a PUT request to `/v2/databases/$DATABASE_ID/eviction_policy` specifying the desired policy.
pub async fn databases_update_eviction_policy(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_update_eviction_policy_request: crate::models::DatabasesUpdateEvictionPolicyRequest) -> Result<(), Error<DatabasesUpdateEvictionPolicyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/eviction_policy", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&databases_update_eviction_policy_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesUpdateEvictionPolicyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To update a database cluster's firewall rules (known as \"trusted sources\" in the control panel), send a PUT request to `/v2/databases/$DATABASE_ID/firewall` specifying which resources should be able to open connections to the database. You may limit connections to specific Droplets, Kubernetes clusters, or IP addresses. When a tag is provided, any Droplet or Kubernetes node with that tag applied to it will have access. The firewall is limited to 100 rules (or trusted sources). When possible, we recommend [placing your databases into a VPC network](https://www.digitalocean.com/docs/networking/vpc/) to limit access to them instead of using a firewall. A successful
pub async fn databases_update_firewall_rules(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_update_firewall_rules_request: crate::models::DatabasesUpdateFirewallRulesRequest) -> Result<(), Error<DatabasesUpdateFirewallRulesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/firewall", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&databases_update_firewall_rules_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesUpdateFirewallRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To configure the window when automatic maintenance should be performed for a database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/maintenance`. A successful request will receive a 204 No Content status code with no body in response.
pub async fn databases_update_maintenance_window(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_maintenance_window: Option<crate::models::DatabaseMaintenanceWindow>) -> Result<(), Error<DatabasesUpdateMaintenanceWindowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/maintenance", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&database_maintenance_window);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesUpdateMaintenanceWindowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To start an online migration, send a PUT request to `/v2/databases/$DATABASE_ID/online-migration` endpoint. Migrating a cluster establishes a connection with an existing cluster and replicates its contents to the target cluster. Online migration is only available for MySQL, PostgreSQL, and Redis clusters.
pub async fn databases_update_online_migration(configuration: &configuration::Configuration, database_cluster_uuid: &str, source_database: crate::models::SourceDatabase) -> Result<crate::models::OnlineMigration, Error<DatabasesUpdateOnlineMigrationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/online-migration", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&source_database);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DatabasesUpdateOnlineMigrationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To migrate a database cluster to a new region, send a `PUT` request to `/v2/databases/$DATABASE_ID/migrate`. The body of the request must specify a `region` attribute.  A successful request will receive a 202 Accepted status code with no body in response. Querying the database cluster will show that its `status` attribute will now be set to `migrating`. This will transition back to `online` when the migration has completed. 
pub async fn databases_update_region(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_update_region_request: crate::models::DatabasesUpdateRegionRequest) -> Result<(), Error<DatabasesUpdateRegionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/migrate", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&databases_update_region_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesUpdateRegionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To configure the SQL modes for an existing MySQL cluster, send a PUT request to `/v2/databases/$DATABASE_ID/sql_mode` specifying the desired modes. See the official MySQL 8 documentation for a [full list of supported SQL modes](https://dev.mysql.com/doc/refman/8.0/en/sql-mode.html#sql-mode-full). A successful request will receive a 204 No Content status code with no body in response.
pub async fn databases_update_sql_mode(configuration: &configuration::Configuration, database_cluster_uuid: &str, sql_mode: crate::models::SqlMode) -> Result<(), Error<DatabasesUpdateSqlModeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/databases/{database_cluster_uuid}/sql_mode", local_var_configuration.base_path, database_cluster_uuid=crate::apis::urlencode(database_cluster_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&sql_mode);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DatabasesUpdateSqlModeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

