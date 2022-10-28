# Rust API client for openapi

# Introduction

The DigitalOcean API allows you to manage Droplets and resources within the
DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.

All of the functionality that you are familiar with in the DigitalOcean
control panel is also available through the API, allowing you to script the
complex actions that your situation requires.

The API documentation will start with a general overview about the design
and technology that has been implemented, followed by reference information
about specific endpoints.

## Requests

Any tool that is fluent in HTTP can communicate with the API simply by
requesting the correct URI. Requests should be made using the HTTPS protocol
so that traffic is encrypted. The interface responds to different methods
depending on the action required.

|Method|Usage|
|--- |--- |
|GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.|
|DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.|
|PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.|
|PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is is a set of instructions on how to modify a resource updating only specific attributes.|
|POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.|
|HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|


## HTTP Statuses

Along with the HTTP methods that the API responds to, it will also return
standard HTTP statuses, including error codes.

In the event of a problem, the status will contain the error code, while the
body of the response will usually contain additional information about the
problem that was encountered.

In general, if the status returned is in the 200 range, it indicates that
the request was fulfilled successfully and that no error was encountered.

Return codes in the 400 range typically indicate that there was an issue
with the request that was sent. Among other things, this could mean that you
did not authenticate correctly, that you are requesting an action that you
do not have authorization for, that the object you are requesting does not
exist, or that your request is malformed.

If you receive a status in the 500 range, this generally indicates a
server-side problem. This means that we are having an issue on our end and
cannot fulfill your request currently.

400 and 500 level error responses will include a JSON object in their body,
including the following attributes:

|Name|Type|Description|
|--- |--- |--- |
|id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"|
|message|string|A message providing additional information about the error, including details to help resolve it when possible.|
|request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|

### Example Error Response

```
    HTTP/1.1 403 Forbidden
    {
      \"id\":       \"forbidden\",
      \"message\":  \"You do not have access for the attempted action.\"
    }
```

## Responses

When a request is successful, a response body will typically be sent back in
the form of a JSON object. An exception to this is when a DELETE request is
processed, which will result in a successful HTTP 204 status and an empty
response body.

Inside of this JSON object, the resource root that was the target of the
request will be set as the key. This will be the singular form of the word
if the request operated on a single object, and the plural form of the word
if a collection was processed.

For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you
will get back an object with a key called \"`droplet`\". However, if you send
the GET request to the general collection at `/v2/droplets`, you will get
back an object with a key called \"`droplets`\".

The value of these keys will generally be a JSON object for a request on a
single object and an array of objects for a request on a collection of
objects.

### Response for a Single Object

```
    {
        \"droplet\": {
            \"name\": \"example.com\"
            . . .
        }
    }
```

### Response for an Object Collection

```
    {
        \"droplets\": [
            {
                \"name\": \"example.com\"
                . . .
            },
            {
                \"name\": \"second.com\"
                . . .
            }
        ]
    }
```

## Meta

In addition to the main resource root, the response may also contain a
`meta` object. This object contains information about the response itself.

The `meta` object contains a `total` key that is set to the total number of
objects returned by the request. This has implications on the `links` object
and pagination.

The `meta` object will only be displayed when it has a value. Currently, the
`meta` object will have a value when a request is made on a collection (like
`droplets` or `domains`).


### Sample Meta Object

```
    {
        . . .
        \"meta\": {
            \"total\": 43
        }
        . . .
    }
```

## Links & Pagination

The `links` object is returned as part of the response body when pagination
is enabled. By default, 20 objects are returned per page. If the response
contains 20 objects or fewer, no `links` object will be returned. If the
response contains more than 20 objects, the first 20 will be returned along
with the `links` object.

You can request a different pagination limit or force pagination by
appending `?per_page=` to the request with the number of items you would
like per page. For instance, to show only two results per page, you could
add `?per_page=2` to the end of your query. The maximum number of results
per page is 200.

The `links` object contains a `pages` object. The `pages` object, in turn,
contains keys indicating the relationship of additional pages. The values of
these are the URLs of the associated pages. The keys will be one of the
following:

*   **first**: The URI of the first page of results.
*   **prev**: The URI of the previous sequential page of results.
*   **next**: The URI of the next sequential page of results.
*   **last**: The URI of the last page of results.

The `pages` object will only include the links that make sense. So for the
first page of results, no `first` or `prev` links will ever be set. This
convention holds true in other situations where a link would not make sense.

### Sample Links Object

```
    {
        . . .
        \"links\": {
            \"pages\": {
                \"last\": \"https://api.digitalocean.com/v2/images?page=2\",
                \"next\": \"https://api.digitalocean.com/v2/images?page=2\"
            }
        }
        . . .
    }
```

## Rate Limit

Requests through the API are rate limited per OAuth token. Current rate limits:

*   5,000 requests per hour
*   250 requests per minute (5% of the hourly total)

Once you exceed either limit, you will be rate limited until the next cycle
starts. Space out any requests that you would otherwise issue in bursts for
the best results.

The rate limiting information is contained within the response headers of
each request. The relevant headers are:

*   **RateLimit-Limit**: The number of requests that can be made per hour.
*   **RateLimit-Remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire.
*   **RateLimit-Reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.

As long as the `RateLimit-Remaining` count is above zero, you will be able
to make additional requests.

The way that a request expires and is removed from the current limit count
is important to understand. Rather than counting all of the requests for an
hour and resetting the `RateLimit-Remaining` value at the end of the hour,
each request instead has its own timer.

This means that each request contributes toward the `RateLimit-Remaining`
count for one complete hour after the request is made. When that request's
timer runs out, it is no longer counted towards the request limit.

This has implications on the meaning of the `RateLimit-Reset` header as
well. Because the entire rate limit is not reset at one time, the value of
this header is set to the time when the _oldest_ request will expire.

Keep this in mind if you see your `RateLimit-Reset` value change, but not
move an entire hour into the future.

If the `RateLimit-Remaining` reaches zero, subsequent requests will receive
a 429 error code until the request reset has been reached. You can see the
format of the response in the examples.

**Note:** The following endpoints have special rate limit requirements that
are independent of the limits defined above.

*   Only 12 `POST` requests to the `/v2/floating_ips` endpoint to create Floating IPs can be made per 60 seconds.
*   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds.

### Sample Rate Limit Headers

```
    . . .
    RateLimit-Limit: 1200
    RateLimit-Remaining: 1193
    RateLimit-Reset: 1402425459
    . . .
```

### Sample Rate Exceeded Response

```
    429 Too Many Requests
    {
            id: \"too_many_requests\",
            message: \"API Rate limit exceeded.\"
    }
```

## Curl Examples

Throughout this document, some example API requests will be given using the
`curl` command. This will allow us to demonstrate the various endpoints in a
simple, textual format.
  
  These examples assume that you are using a Linux or macOS command line. To run
these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:

* For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1)
to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).

* For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2)
to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).

* WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine.
Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10), 
then follow this API documentation normally.

The names of account-specific references (like Droplet IDs, for instance)
will be represented by variables. For instance, a Droplet ID may be
represented by a variable called `$DROPLET_ID`. You can set the associated
variables in your environment if you wish to use the examples without
modification.

The first variable that you should set to get started is your OAuth
authorization token. The next section will go over the details of this, but
you can set an environmental variable for it now.

Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications)
section of the DigitalOcean control panel. Use an existing token if you have
saved one, or generate a new token with the \"Generate new token\" button.
Copy the generated token and use it to set and export the TOKEN variable in
your environment as the example shows.

You may also wish to set some other variables now or as you go along. For
example, you may wish to set the `DROPLET_ID` variable to one of your
Droplet IDs since this will be used frequently in the API.

If you are following along, make sure you use a Droplet ID that you control
so that your commands will execute correctly.

If you need access to the headers of a response through `curl`, you can pass
the `-i` flag to display the header information along with the body. If you
are only interested in the header, you can instead pass the `-I` flag, which
will exclude the response body entirely.


### Set and Export your OAuth Token

```
export DIGITALOCEAN_TOKEN=your_token_here
```

### Set and Export a Variable

```
export DROPLET_ID=1111111
```

## Parameters

There are two different ways to pass parameters in a request with the API.

When passing parameters to create or update an object, parameters should be
passed as a JSON object containing the appropriate attribute names and
values as key-value pairs. When you use this format, you should specify that
you are sending a JSON object in the header. This is done by setting the
`Content-Type` header to `application/json`. This ensures that your request
is interpreted correctly.

When passing parameters to filter a response on GET requests, parameters can
be passed using standard query attributes. In this case, the parameters
would be embedded into the URI itself by appending a `?` to the end of the
URI and then setting each attribute with an equal sign. Attributes can be
separated with a `&`. Tools like `curl` can create the appropriate URI when
given parameters and values; this can also be done using the `-F` flag and
then passing the key and value as an argument. The argument should take the
form of a quoted string with the attribute being set to a value with an
equal sign.

### Pass Parameters as a JSON Object

```
    curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\
        -H \"Content-Type: application/json\" \\
        -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\
        -X POST \"https://api.digitalocean.com/v2/domains\"
```

### Pass Filter Parameters as a Query String

```
     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\
         -X GET \\
         \"https://api.digitalocean.com/v2/images?private=true\"
```

## Cross Origin Resource Sharing

In order to make requests to the API from other domains, the API implements
Cross Origin Resource Sharing (CORS) support.

CORS support is generally used to create AJAX requests outside of the domain
that the request originated from. This is necessary to implement projects
like control panels utilizing the API. This tells the browser that it can
send requests to an outside domain.

The procedure that the browser initiates in order to perform these actions
(other than GET requests) begins by sending a \"preflight\" request. This sets
the `Origin` header and uses the `OPTIONS` method. The server will reply
back with the methods it allows and some of the limits it imposes. The
client then sends the actual request if it falls within the allowed
constraints.

This process is usually done in the background by the browser, but you can
use curl to emulate this process using the example provided. The headers
that will be set to show the constraints are:

*   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header.
*   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods.
*   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain.
*   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent.
*   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.

You should not need to be concerned with the details of these headers,
because the browser will typically do all of the work for you.



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0
- Package version: 2.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.digitalocean.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountApi* | [**account_get**](docs/AccountApi.md#account_get) | **GET** /v2/account | Get User Information
*ActionsApi* | [**actions_get**](docs/ActionsApi.md#actions_get) | **GET** /v2/actions/{action_id} | Retrieve an Existing Action
*ActionsApi* | [**actions_list**](docs/ActionsApi.md#actions_list) | **GET** /v2/actions | List All Actions
*AppsApi* | [**apps_assign_alert_destinations**](docs/AppsApi.md#apps_assign_alert_destinations) | **POST** /v2/apps/{app_id}/alerts/{alert_id}/destinations | Update destinations for alerts
*AppsApi* | [**apps_cancel_deployment**](docs/AppsApi.md#apps_cancel_deployment) | **POST** /v2/apps/{app_id}/deployments/{deployment_id}/cancel | Cancel a Deployment
*AppsApi* | [**apps_commit_rollback**](docs/AppsApi.md#apps_commit_rollback) | **POST** /v2/apps/{app_id}/rollback/commit | Commit App Rollback
*AppsApi* | [**apps_create**](docs/AppsApi.md#apps_create) | **POST** /v2/apps | Create a New App
*AppsApi* | [**apps_create_deployment**](docs/AppsApi.md#apps_create_deployment) | **POST** /v2/apps/{app_id}/deployments | Create an App Deployment
*AppsApi* | [**apps_create_rollback**](docs/AppsApi.md#apps_create_rollback) | **POST** /v2/apps/{app_id}/rollback | Rollback App
*AppsApi* | [**apps_delete**](docs/AppsApi.md#apps_delete) | **DELETE** /v2/apps/{id} | Delete an App
*AppsApi* | [**apps_get**](docs/AppsApi.md#apps_get) | **GET** /v2/apps/{id} | Retrieve an Existing App
*AppsApi* | [**apps_get_deployment**](docs/AppsApi.md#apps_get_deployment) | **GET** /v2/apps/{app_id}/deployments/{deployment_id} | Retrieve an App Deployment
*AppsApi* | [**apps_get_instance_size**](docs/AppsApi.md#apps_get_instance_size) | **GET** /v2/apps/tiers/instance_sizes/{slug} | Retrieve an Instance Size
*AppsApi* | [**apps_get_logs**](docs/AppsApi.md#apps_get_logs) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/components/{component_name}/logs | Retrieve Deployment Logs
*AppsApi* | [**apps_get_logs_aggregate**](docs/AppsApi.md#apps_get_logs_aggregate) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/logs | Retrieve Aggregate Deployment Logs
*AppsApi* | [**apps_get_tier**](docs/AppsApi.md#apps_get_tier) | **GET** /v2/apps/tiers/{slug} | Retrieve an App Tier
*AppsApi* | [**apps_list**](docs/AppsApi.md#apps_list) | **GET** /v2/apps | List All Apps
*AppsApi* | [**apps_list_alerts**](docs/AppsApi.md#apps_list_alerts) | **GET** /v2/apps/{app_id}/alerts | List all app alerts
*AppsApi* | [**apps_list_deployments**](docs/AppsApi.md#apps_list_deployments) | **GET** /v2/apps/{app_id}/deployments | List App Deployments
*AppsApi* | [**apps_list_instance_sizes**](docs/AppsApi.md#apps_list_instance_sizes) | **GET** /v2/apps/tiers/instance_sizes | List Instance Sizes
*AppsApi* | [**apps_list_regions**](docs/AppsApi.md#apps_list_regions) | **GET** /v2/apps/regions | List App Regions
*AppsApi* | [**apps_list_tiers**](docs/AppsApi.md#apps_list_tiers) | **GET** /v2/apps/tiers | List App Tiers
*AppsApi* | [**apps_revert_rollback**](docs/AppsApi.md#apps_revert_rollback) | **POST** /v2/apps/{app_id}/rollback/revert | Revert App Rollback
*AppsApi* | [**apps_update**](docs/AppsApi.md#apps_update) | **PUT** /v2/apps/{id} | Update an App
*AppsApi* | [**apps_validate_app_spec**](docs/AppsApi.md#apps_validate_app_spec) | **POST** /v2/apps/propose | Propose an App Spec
*AppsApi* | [**apps_validate_rollback**](docs/AppsApi.md#apps_validate_rollback) | **POST** /v2/apps/{app_id}/rollback/validate | Validate App Rollback
*BillingApi* | [**balance_get**](docs/BillingApi.md#balance_get) | **GET** /v2/customers/my/balance | Get Customer Balance
*BillingApi* | [**billing_history_list**](docs/BillingApi.md#billing_history_list) | **GET** /v2/customers/my/billing_history | List Billing History
*BillingApi* | [**invoices_get_by_uuid**](docs/BillingApi.md#invoices_get_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid} | Retrieve an Invoice by UUID
*BillingApi* | [**invoices_get_csv_by_uuid**](docs/BillingApi.md#invoices_get_csv_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/csv | Retrieve an Invoice CSV by UUID
*BillingApi* | [**invoices_get_pdf_by_uuid**](docs/BillingApi.md#invoices_get_pdf_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/pdf | Retrieve an Invoice PDF by UUID
*BillingApi* | [**invoices_get_summary_by_uuid**](docs/BillingApi.md#invoices_get_summary_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/summary | Retrieve an Invoice Summary by UUID
*BillingApi* | [**invoices_list**](docs/BillingApi.md#invoices_list) | **GET** /v2/customers/my/invoices | List All Invoices
*BlockStorageApi* | [**volume_snapshots_create**](docs/BlockStorageApi.md#volume_snapshots_create) | **POST** /v2/volumes/{volume_id}/snapshots | Create Snapshot from a Volume
*BlockStorageApi* | [**volume_snapshots_delete_by_id**](docs/BlockStorageApi.md#volume_snapshots_delete_by_id) | **DELETE** /v2/volumes/snapshots/{snapshot_id} | Delete a Volume Snapshot
*BlockStorageApi* | [**volume_snapshots_get_by_id**](docs/BlockStorageApi.md#volume_snapshots_get_by_id) | **GET** /v2/volumes/snapshots/{snapshot_id} | Retrieve an Existing Volume Snapshot
*BlockStorageApi* | [**volume_snapshots_list**](docs/BlockStorageApi.md#volume_snapshots_list) | **GET** /v2/volumes/{volume_id}/snapshots | List Snapshots for a Volume
*BlockStorageApi* | [**volumes_create**](docs/BlockStorageApi.md#volumes_create) | **POST** /v2/volumes | Create a New Block Storage Volume
*BlockStorageApi* | [**volumes_delete**](docs/BlockStorageApi.md#volumes_delete) | **DELETE** /v2/volumes/{volume_id} | Delete a Block Storage Volume
*BlockStorageApi* | [**volumes_delete_by_name**](docs/BlockStorageApi.md#volumes_delete_by_name) | **DELETE** /v2/volumes | Delete a Block Storage Volume by Name
*BlockStorageApi* | [**volumes_get**](docs/BlockStorageApi.md#volumes_get) | **GET** /v2/volumes/{volume_id} | Retrieve an Existing Block Storage Volume
*BlockStorageApi* | [**volumes_list**](docs/BlockStorageApi.md#volumes_list) | **GET** /v2/volumes | List All Block Storage Volumes
*BlockStorageActionsApi* | [**volume_actions_get**](docs/BlockStorageActionsApi.md#volume_actions_get) | **GET** /v2/volumes/{volume_id}/actions/{action_id} | Retrieve an Existing Volume Action
*BlockStorageActionsApi* | [**volume_actions_list**](docs/BlockStorageActionsApi.md#volume_actions_list) | **GET** /v2/volumes/{volume_id}/actions | List All Actions for a Volume
*BlockStorageActionsApi* | [**volume_actions_post**](docs/BlockStorageActionsApi.md#volume_actions_post) | **POST** /v2/volumes/actions | Initiate A Block Storage Action By Volume Name
*BlockStorageActionsApi* | [**volume_actions_post_by_id**](docs/BlockStorageActionsApi.md#volume_actions_post_by_id) | **POST** /v2/volumes/{volume_id}/actions | Initiate A Block Storage Action By Volume Id
*CdnEndpointsApi* | [**cdn_create_endpoint**](docs/CdnEndpointsApi.md#cdn_create_endpoint) | **POST** /v2/cdn/endpoints | Create a New CDN Endpoint
*CdnEndpointsApi* | [**cdn_delete_endpoint**](docs/CdnEndpointsApi.md#cdn_delete_endpoint) | **DELETE** /v2/cdn/endpoints/{cdn_id} | Delete a CDN Endpoint
*CdnEndpointsApi* | [**cdn_get_endpoint**](docs/CdnEndpointsApi.md#cdn_get_endpoint) | **GET** /v2/cdn/endpoints/{cdn_id} | Retrieve an Existing CDN Endpoint
*CdnEndpointsApi* | [**cdn_list_endpoints**](docs/CdnEndpointsApi.md#cdn_list_endpoints) | **GET** /v2/cdn/endpoints | List All CDN Endpoints
*CdnEndpointsApi* | [**cdn_purge_cache**](docs/CdnEndpointsApi.md#cdn_purge_cache) | **DELETE** /v2/cdn/endpoints/{cdn_id}/cache | Purge the Cache for an Existing CDN Endpoint
*CdnEndpointsApi* | [**cdn_update_endpoints**](docs/CdnEndpointsApi.md#cdn_update_endpoints) | **PUT** /v2/cdn/endpoints/{cdn_id} | Update a CDN Endpoint
*CertificatesApi* | [**certificates_create**](docs/CertificatesApi.md#certificates_create) | **POST** /v2/certificates | Create a New Certificate
*CertificatesApi* | [**certificates_delete**](docs/CertificatesApi.md#certificates_delete) | **DELETE** /v2/certificates/{certificate_id} | Delete a Certificate
*CertificatesApi* | [**certificates_get**](docs/CertificatesApi.md#certificates_get) | **GET** /v2/certificates/{certificate_id} | Retrieve an Existing Certificate
*CertificatesApi* | [**certificates_list**](docs/CertificatesApi.md#certificates_list) | **GET** /v2/certificates | List All Certificates
*Class1ClickApplicationsApi* | [**one_clicks_install_kubernetes**](docs/Class1ClickApplicationsApi.md#one_clicks_install_kubernetes) | **POST** /v2/1-clicks/kubernetes | Install Kubernetes 1-Click Applications
*Class1ClickApplicationsApi* | [**one_clicks_list**](docs/Class1ClickApplicationsApi.md#one_clicks_list) | **GET** /v2/1-clicks | List 1-Click Applications
*ContainerRegistryApi* | [**registry_create**](docs/ContainerRegistryApi.md#registry_create) | **POST** /v2/registry | Create Container Registry
*ContainerRegistryApi* | [**registry_delete**](docs/ContainerRegistryApi.md#registry_delete) | **DELETE** /v2/registry | Delete Container Registry
*ContainerRegistryApi* | [**registry_delete_repository_manifest**](docs/ContainerRegistryApi.md#registry_delete_repository_manifest) | **DELETE** /v2/registry/{registry_name}/{repository_name}/digests/{manifest_digest} | Delete Container Registry Repository Manifest
*ContainerRegistryApi* | [**registry_delete_repository_tag**](docs/ContainerRegistryApi.md#registry_delete_repository_tag) | **DELETE** /v2/registry/{registry_name}/{repository_name}/tags/{repository_tag} | Delete Container Registry Repository Tag
*ContainerRegistryApi* | [**registry_get**](docs/ContainerRegistryApi.md#registry_get) | **GET** /v2/registry | Get Container Registry Information
*ContainerRegistryApi* | [**registry_get_docker_credentials**](docs/ContainerRegistryApi.md#registry_get_docker_credentials) | **GET** /v2/registry/docker-credentials | Get Docker Credentials for Container Registry
*ContainerRegistryApi* | [**registry_get_garbage_collection**](docs/ContainerRegistryApi.md#registry_get_garbage_collection) | **GET** /v2/registry/{registry_name}/garbage-collection | Get Active Garbage Collection
*ContainerRegistryApi* | [**registry_get_options**](docs/ContainerRegistryApi.md#registry_get_options) | **GET** /v2/registry/options | List Registry Options (Subscription Tiers and Available Regions)
*ContainerRegistryApi* | [**registry_get_subscription**](docs/ContainerRegistryApi.md#registry_get_subscription) | **GET** /v2/registry/subscription | Get Subscription Information
*ContainerRegistryApi* | [**registry_list_garbage_collections**](docs/ContainerRegistryApi.md#registry_list_garbage_collections) | **GET** /v2/registry/{registry_name}/garbage-collections | List Garbage Collections
*ContainerRegistryApi* | [**registry_list_repositories**](docs/ContainerRegistryApi.md#registry_list_repositories) | **GET** /v2/registry/{registry_name}/repositories | List All Container Registry Repositories
*ContainerRegistryApi* | [**registry_list_repositories_v2**](docs/ContainerRegistryApi.md#registry_list_repositories_v2) | **GET** /v2/registry/{registry_name}/repositoriesV2 | List All Container Registry Repositories (V2)
*ContainerRegistryApi* | [**registry_list_repository_manifests**](docs/ContainerRegistryApi.md#registry_list_repository_manifests) | **GET** /v2/registry/{registry_name}/{repository_name}/digests | List All Container Registry Repository Manifests
*ContainerRegistryApi* | [**registry_list_repository_tags**](docs/ContainerRegistryApi.md#registry_list_repository_tags) | **GET** /v2/registry/{registry_name}/{repository_name}/tags | List All Container Registry Repository Tags
*ContainerRegistryApi* | [**registry_run_garbage_collection**](docs/ContainerRegistryApi.md#registry_run_garbage_collection) | **POST** /v2/registry/{registry_name}/garbage-collection | Start Garbage Collection
*ContainerRegistryApi* | [**registry_update_garbage_collection**](docs/ContainerRegistryApi.md#registry_update_garbage_collection) | **PUT** /v2/registry/{registry_name}/garbage-collection/{garbage_collection_uuid} | Update Garbage Collection
*ContainerRegistryApi* | [**registry_update_subscription**](docs/ContainerRegistryApi.md#registry_update_subscription) | **POST** /v2/registry/subscription | Update Subscription Tier
*ContainerRegistryApi* | [**registry_validate_name**](docs/ContainerRegistryApi.md#registry_validate_name) | **POST** /v2/registry/validate-name | Validate a Container Registry Name
*DatabasesApi* | [**databases_add**](docs/DatabasesApi.md#databases_add) | **POST** /v2/databases/{database_cluster_uuid}/dbs | Add a New Database
*DatabasesApi* | [**databases_add_connection_pool**](docs/DatabasesApi.md#databases_add_connection_pool) | **POST** /v2/databases/{database_cluster_uuid}/pools | Add a New Connection Pool (PostgreSQL)
*DatabasesApi* | [**databases_add_user**](docs/DatabasesApi.md#databases_add_user) | **POST** /v2/databases/{database_cluster_uuid}/users | Add a Database User
*DatabasesApi* | [**databases_create_cluster**](docs/DatabasesApi.md#databases_create_cluster) | **POST** /v2/databases | Create a New Database Cluster
*DatabasesApi* | [**databases_create_replica**](docs/DatabasesApi.md#databases_create_replica) | **POST** /v2/databases/{database_cluster_uuid}/replicas | Create a Read-only Replica
*DatabasesApi* | [**databases_delete**](docs/DatabasesApi.md#databases_delete) | **DELETE** /v2/databases/{database_cluster_uuid}/dbs/{database_name} | Delete a Database
*DatabasesApi* | [**databases_delete_connection_pool**](docs/DatabasesApi.md#databases_delete_connection_pool) | **DELETE** /v2/databases/{database_cluster_uuid}/pools/{pool_name} | Delete a Connection Pool (PostgreSQL)
*DatabasesApi* | [**databases_delete_online_migration**](docs/DatabasesApi.md#databases_delete_online_migration) | **DELETE** /v2/databases/{database_cluster_uuid}/online-migration/{migration_id} | Stop an Online Migration
*DatabasesApi* | [**databases_delete_user**](docs/DatabasesApi.md#databases_delete_user) | **DELETE** /v2/databases/{database_cluster_uuid}/users/{username} | Remove a Database User
*DatabasesApi* | [**databases_destroy_cluster**](docs/DatabasesApi.md#databases_destroy_cluster) | **DELETE** /v2/databases/{database_cluster_uuid} | Destroy a Database Cluster
*DatabasesApi* | [**databases_destroy_replica**](docs/DatabasesApi.md#databases_destroy_replica) | **DELETE** /v2/databases/{database_cluster_uuid}/replicas/{replica_name} | Destroy a Read-only Replica
*DatabasesApi* | [**databases_get**](docs/DatabasesApi.md#databases_get) | **GET** /v2/databases/{database_cluster_uuid}/dbs/{database_name} | Retrieve an Existing Database
*DatabasesApi* | [**databases_get_ca**](docs/DatabasesApi.md#databases_get_ca) | **GET** /v2/databases/{database_cluster_uuid}/ca | Retrieve the Public Certificate
*DatabasesApi* | [**databases_get_cluster**](docs/DatabasesApi.md#databases_get_cluster) | **GET** /v2/databases/{database_cluster_uuid} | Retrieve an Existing Database Cluster
*DatabasesApi* | [**databases_get_config**](docs/DatabasesApi.md#databases_get_config) | **GET** /v2/databases/{database_cluster_uuid}/config | Retrieve an Existing Database Cluster Configuration
*DatabasesApi* | [**databases_get_connection_pool**](docs/DatabasesApi.md#databases_get_connection_pool) | **GET** /v2/databases/{database_cluster_uuid}/pools/{pool_name} | Retrieve Existing Connection Pool (PostgreSQL)
*DatabasesApi* | [**databases_get_eviction_policy**](docs/DatabasesApi.md#databases_get_eviction_policy) | **GET** /v2/databases/{database_cluster_uuid}/eviction_policy | Retrieve the Eviction Policy for a Redis Cluster
*DatabasesApi* | [**databases_get_migration_status**](docs/DatabasesApi.md#databases_get_migration_status) | **GET** /v2/databases/{database_cluster_uuid}/online-migration | Retrieve the Status of an Online Migration
*DatabasesApi* | [**databases_get_replica**](docs/DatabasesApi.md#databases_get_replica) | **GET** /v2/databases/{database_cluster_uuid}/replicas/{replica_name} | Retrieve an Existing Read-only Replica
*DatabasesApi* | [**databases_get_sql_mode**](docs/DatabasesApi.md#databases_get_sql_mode) | **GET** /v2/databases/{database_cluster_uuid}/sql_mode | Retrieve the SQL Modes for a MySQL Cluster
*DatabasesApi* | [**databases_get_user**](docs/DatabasesApi.md#databases_get_user) | **GET** /v2/databases/{database_cluster_uuid}/users/{username} | Retrieve an Existing Database User
*DatabasesApi* | [**databases_list**](docs/DatabasesApi.md#databases_list) | **GET** /v2/databases/{database_cluster_uuid}/dbs | List All Databases
*DatabasesApi* | [**databases_list_backups**](docs/DatabasesApi.md#databases_list_backups) | **GET** /v2/databases/{database_cluster_uuid}/backups | List Backups for a Database Cluster
*DatabasesApi* | [**databases_list_clusters**](docs/DatabasesApi.md#databases_list_clusters) | **GET** /v2/databases | List All Database Clusters
*DatabasesApi* | [**databases_list_connection_pools**](docs/DatabasesApi.md#databases_list_connection_pools) | **GET** /v2/databases/{database_cluster_uuid}/pools | List Connection Pools (PostgreSQL)
*DatabasesApi* | [**databases_list_firewall_rules**](docs/DatabasesApi.md#databases_list_firewall_rules) | **GET** /v2/databases/{database_cluster_uuid}/firewall | List Firewall Rules (Trusted Sources) for a Database Cluster
*DatabasesApi* | [**databases_list_options**](docs/DatabasesApi.md#databases_list_options) | **GET** /v2/databases/options | List Database Options
*DatabasesApi* | [**databases_list_replicas**](docs/DatabasesApi.md#databases_list_replicas) | **GET** /v2/databases/{database_cluster_uuid}/replicas | List All Read-only Replicas
*DatabasesApi* | [**databases_list_users**](docs/DatabasesApi.md#databases_list_users) | **GET** /v2/databases/{database_cluster_uuid}/users | List all Database Users
*DatabasesApi* | [**databases_patch_config**](docs/DatabasesApi.md#databases_patch_config) | **PATCH** /v2/databases/{database_cluster_uuid}/config | Update the Database Configuration for an Existing Database
*DatabasesApi* | [**databases_reset_auth**](docs/DatabasesApi.md#databases_reset_auth) | **POST** /v2/databases/{database_cluster_uuid}/users/{username}/reset_auth | Reset a Database User's Password or Authentication Method
*DatabasesApi* | [**databases_update_cluster_size**](docs/DatabasesApi.md#databases_update_cluster_size) | **PUT** /v2/databases/{database_cluster_uuid}/resize | Resize a Database Cluster
*DatabasesApi* | [**databases_update_eviction_policy**](docs/DatabasesApi.md#databases_update_eviction_policy) | **PUT** /v2/databases/{database_cluster_uuid}/eviction_policy | Configure the Eviction Policy for a Redis Cluster
*DatabasesApi* | [**databases_update_firewall_rules**](docs/DatabasesApi.md#databases_update_firewall_rules) | **PUT** /v2/databases/{database_cluster_uuid}/firewall | Update Firewall Rules (Trusted Sources) for a Database
*DatabasesApi* | [**databases_update_maintenance_window**](docs/DatabasesApi.md#databases_update_maintenance_window) | **PUT** /v2/databases/{database_cluster_uuid}/maintenance | Configure a Database Cluster's Maintenance Window
*DatabasesApi* | [**databases_update_online_migration**](docs/DatabasesApi.md#databases_update_online_migration) | **PUT** /v2/databases/{database_cluster_uuid}/online-migration | Start an Online Migration
*DatabasesApi* | [**databases_update_region**](docs/DatabasesApi.md#databases_update_region) | **PUT** /v2/databases/{database_cluster_uuid}/migrate | Migrate a Database Cluster to a New Region
*DatabasesApi* | [**databases_update_sql_mode**](docs/DatabasesApi.md#databases_update_sql_mode) | **PUT** /v2/databases/{database_cluster_uuid}/sql_mode | Update SQL Mode for a Cluster
*DomainRecordsApi* | [**domains_create_record**](docs/DomainRecordsApi.md#domains_create_record) | **POST** /v2/domains/{domain_name}/records | Create a New Domain Record
*DomainRecordsApi* | [**domains_delete_record**](docs/DomainRecordsApi.md#domains_delete_record) | **DELETE** /v2/domains/{domain_name}/records/{domain_record_id} | Delete a Domain Record
*DomainRecordsApi* | [**domains_get_record**](docs/DomainRecordsApi.md#domains_get_record) | **GET** /v2/domains/{domain_name}/records/{domain_record_id} | Retrieve an Existing Domain Record
*DomainRecordsApi* | [**domains_list_records**](docs/DomainRecordsApi.md#domains_list_records) | **GET** /v2/domains/{domain_name}/records | List All Domain Records
*DomainRecordsApi* | [**domains_patch_record**](docs/DomainRecordsApi.md#domains_patch_record) | **PATCH** /v2/domains/{domain_name}/records/{domain_record_id} | Update a Domain Record
*DomainRecordsApi* | [**domains_update_record**](docs/DomainRecordsApi.md#domains_update_record) | **PUT** /v2/domains/{domain_name}/records/{domain_record_id} | Update a Domain Record
*DomainsApi* | [**domains_create**](docs/DomainsApi.md#domains_create) | **POST** /v2/domains | Create a New Domain
*DomainsApi* | [**domains_delete**](docs/DomainsApi.md#domains_delete) | **DELETE** /v2/domains/{domain_name} | Delete a Domain
*DomainsApi* | [**domains_get**](docs/DomainsApi.md#domains_get) | **GET** /v2/domains/{domain_name} | Retrieve an Existing Domain
*DomainsApi* | [**domains_list**](docs/DomainsApi.md#domains_list) | **GET** /v2/domains | List All Domains
*DropletActionsApi* | [**droplet_actions_get**](docs/DropletActionsApi.md#droplet_actions_get) | **GET** /v2/droplets/{droplet_id}/actions/{action_id} | Retrieve a Droplet Action
*DropletActionsApi* | [**droplet_actions_list**](docs/DropletActionsApi.md#droplet_actions_list) | **GET** /v2/droplets/{droplet_id}/actions | List Actions for a Droplet
*DropletActionsApi* | [**droplet_actions_post**](docs/DropletActionsApi.md#droplet_actions_post) | **POST** /v2/droplets/{droplet_id}/actions | Initiate a Droplet Action
*DropletActionsApi* | [**droplet_actions_post_by_tag**](docs/DropletActionsApi.md#droplet_actions_post_by_tag) | **POST** /v2/droplets/actions | Acting on Tagged Droplets
*DropletsApi* | [**droplets_create**](docs/DropletsApi.md#droplets_create) | **POST** /v2/droplets | Create a New Droplet
*DropletsApi* | [**droplets_destroy**](docs/DropletsApi.md#droplets_destroy) | **DELETE** /v2/droplets/{droplet_id} | Delete an Existing Droplet
*DropletsApi* | [**droplets_destroy_by_tag**](docs/DropletsApi.md#droplets_destroy_by_tag) | **DELETE** /v2/droplets | Deleting Droplets by Tag
*DropletsApi* | [**droplets_destroy_retry_with_associated_resources**](docs/DropletsApi.md#droplets_destroy_retry_with_associated_resources) | **POST** /v2/droplets/{droplet_id}/destroy_with_associated_resources/retry | Retry a Droplet Destroy with Associated Resources Request
*DropletsApi* | [**droplets_destroy_with_associated_resources_dangerous**](docs/DropletsApi.md#droplets_destroy_with_associated_resources_dangerous) | **DELETE** /v2/droplets/{droplet_id}/destroy_with_associated_resources/dangerous | Destroy a Droplet and All of its Associated Resources (Dangerous)
*DropletsApi* | [**droplets_destroy_with_associated_resources_selective**](docs/DropletsApi.md#droplets_destroy_with_associated_resources_selective) | **DELETE** /v2/droplets/{droplet_id}/destroy_with_associated_resources/selective | Selectively Destroy a Droplet and its Associated Resources
*DropletsApi* | [**droplets_get**](docs/DropletsApi.md#droplets_get) | **GET** /v2/droplets/{droplet_id} | Retrieve an Existing Droplet
*DropletsApi* | [**droplets_get_destroy_associated_resources_status**](docs/DropletsApi.md#droplets_get_destroy_associated_resources_status) | **GET** /v2/droplets/{droplet_id}/destroy_with_associated_resources/status | Check Status of a Droplet Destroy with Associated Resources Request
*DropletsApi* | [**droplets_list**](docs/DropletsApi.md#droplets_list) | **GET** /v2/droplets | List All Droplets
*DropletsApi* | [**droplets_list_associated_resources**](docs/DropletsApi.md#droplets_list_associated_resources) | **GET** /v2/droplets/{droplet_id}/destroy_with_associated_resources | List Associated Resources for a Droplet
*DropletsApi* | [**droplets_list_backups**](docs/DropletsApi.md#droplets_list_backups) | **GET** /v2/droplets/{droplet_id}/backups | List Backups for a Droplet
*DropletsApi* | [**droplets_list_firewalls**](docs/DropletsApi.md#droplets_list_firewalls) | **GET** /v2/droplets/{droplet_id}/firewalls | List all Firewalls Applied to a Droplet
*DropletsApi* | [**droplets_list_kernels**](docs/DropletsApi.md#droplets_list_kernels) | **GET** /v2/droplets/{droplet_id}/kernels | List All Available Kernels for a Droplet
*DropletsApi* | [**droplets_list_neighbors**](docs/DropletsApi.md#droplets_list_neighbors) | **GET** /v2/droplets/{droplet_id}/neighbors | List Neighbors for a Droplet
*DropletsApi* | [**droplets_list_neighbors_ids**](docs/DropletsApi.md#droplets_list_neighbors_ids) | **GET** /v2/reports/droplet_neighbors_ids | List All Droplet Neighbors
*DropletsApi* | [**droplets_list_snapshots**](docs/DropletsApi.md#droplets_list_snapshots) | **GET** /v2/droplets/{droplet_id}/snapshots | List Snapshots for a Droplet
*FirewallsApi* | [**firewalls_add_rules**](docs/FirewallsApi.md#firewalls_add_rules) | **POST** /v2/firewalls/{firewall_id}/rules | Add Rules to a Firewall
*FirewallsApi* | [**firewalls_add_tags**](docs/FirewallsApi.md#firewalls_add_tags) | **POST** /v2/firewalls/{firewall_id}/tags | Add Tags to a Firewall
*FirewallsApi* | [**firewalls_assign_droplets**](docs/FirewallsApi.md#firewalls_assign_droplets) | **POST** /v2/firewalls/{firewall_id}/droplets | Add Droplets to a Firewall
*FirewallsApi* | [**firewalls_create**](docs/FirewallsApi.md#firewalls_create) | **POST** /v2/firewalls | Create a New Firewall
*FirewallsApi* | [**firewalls_delete**](docs/FirewallsApi.md#firewalls_delete) | **DELETE** /v2/firewalls/{firewall_id} | Delete a Firewall
*FirewallsApi* | [**firewalls_delete_droplets**](docs/FirewallsApi.md#firewalls_delete_droplets) | **DELETE** /v2/firewalls/{firewall_id}/droplets | Remove Droplets from a Firewall
*FirewallsApi* | [**firewalls_delete_rules**](docs/FirewallsApi.md#firewalls_delete_rules) | **DELETE** /v2/firewalls/{firewall_id}/rules | Remove Rules from a Firewall
*FirewallsApi* | [**firewalls_delete_tags**](docs/FirewallsApi.md#firewalls_delete_tags) | **DELETE** /v2/firewalls/{firewall_id}/tags | Remove Tags from a Firewall
*FirewallsApi* | [**firewalls_get**](docs/FirewallsApi.md#firewalls_get) | **GET** /v2/firewalls/{firewall_id} | Retrieve an Existing Firewall
*FirewallsApi* | [**firewalls_list**](docs/FirewallsApi.md#firewalls_list) | **GET** /v2/firewalls | List All Firewalls
*FirewallsApi* | [**firewalls_update**](docs/FirewallsApi.md#firewalls_update) | **PUT** /v2/firewalls/{firewall_id} | Update a Firewall
*FloatingIpActionsApi* | [**floating_ips_action_get**](docs/FloatingIpActionsApi.md#floating_ips_action_get) | **GET** /v2/floating_ips/{floating_ip}/actions/{action_id} | Retrieve an Existing Floating IP Action
*FloatingIpActionsApi* | [**floating_ips_action_list**](docs/FloatingIpActionsApi.md#floating_ips_action_list) | **GET** /v2/floating_ips/{floating_ip}/actions | List All Actions for a Floating IP
*FloatingIpActionsApi* | [**floating_ips_action_post**](docs/FloatingIpActionsApi.md#floating_ips_action_post) | **POST** /v2/floating_ips/{floating_ip}/actions | Initiate a Floating IP Action
*FloatingIpsApi* | [**floating_ips_create**](docs/FloatingIpsApi.md#floating_ips_create) | **POST** /v2/floating_ips | Create a New Floating IP
*FloatingIpsApi* | [**floating_ips_delete**](docs/FloatingIpsApi.md#floating_ips_delete) | **DELETE** /v2/floating_ips/{floating_ip} | Delete a Floating IPs
*FloatingIpsApi* | [**floating_ips_get**](docs/FloatingIpsApi.md#floating_ips_get) | **GET** /v2/floating_ips/{floating_ip} | Retrieve an Existing Floating IP
*FloatingIpsApi* | [**floating_ips_list**](docs/FloatingIpsApi.md#floating_ips_list) | **GET** /v2/floating_ips | List All Floating IPs
*FunctionsApi* | [**functions_create_namespace**](docs/FunctionsApi.md#functions_create_namespace) | **POST** /v2/functions/namespaces | Create Namespace
*FunctionsApi* | [**functions_delete_namespace**](docs/FunctionsApi.md#functions_delete_namespace) | **DELETE** /v2/functions/namespaces/{namespace_id} | Delete Namespace
*FunctionsApi* | [**functions_get_namespace**](docs/FunctionsApi.md#functions_get_namespace) | **GET** /v2/functions/namespaces/{namespace_id} | Get Namespace
*FunctionsApi* | [**functions_list_namespaces**](docs/FunctionsApi.md#functions_list_namespaces) | **GET** /v2/functions/namespaces | List Namespaces
*ImageActionsApi* | [**image_actions_get**](docs/ImageActionsApi.md#image_actions_get) | **GET** /v2/images/{image_id}/actions/{action_id} | Retrieve an Existing Action
*ImageActionsApi* | [**image_actions_list**](docs/ImageActionsApi.md#image_actions_list) | **GET** /v2/images/{image_id}/actions | List All Actions for an Image
*ImageActionsApi* | [**image_actions_post**](docs/ImageActionsApi.md#image_actions_post) | **POST** /v2/images/{image_id}/actions | Initiate an Image Action
*ImagesApi* | [**images_create_custom**](docs/ImagesApi.md#images_create_custom) | **POST** /v2/images | Create a Custom Image
*ImagesApi* | [**images_delete**](docs/ImagesApi.md#images_delete) | **DELETE** /v2/images/{image_id} | Delete an Image
*ImagesApi* | [**images_get**](docs/ImagesApi.md#images_get) | **GET** /v2/images/{image_id} | Retrieve an Existing Image
*ImagesApi* | [**images_list**](docs/ImagesApi.md#images_list) | **GET** /v2/images | List All Images
*ImagesApi* | [**images_update**](docs/ImagesApi.md#images_update) | **PUT** /v2/images/{image_id} | Update an Image
*KubernetesApi* | [**kubernetes_add_node_pool**](docs/KubernetesApi.md#kubernetes_add_node_pool) | **POST** /v2/kubernetes/clusters/{cluster_id}/node_pools | Add a Node Pool to a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_add_registry**](docs/KubernetesApi.md#kubernetes_add_registry) | **POST** /v2/kubernetes/registry | Add Container Registry to Kubernetes Clusters
*KubernetesApi* | [**kubernetes_create_cluster**](docs/KubernetesApi.md#kubernetes_create_cluster) | **POST** /v2/kubernetes/clusters | Create a New Kubernetes Cluster
*KubernetesApi* | [**kubernetes_delete_cluster**](docs/KubernetesApi.md#kubernetes_delete_cluster) | **DELETE** /v2/kubernetes/clusters/{cluster_id} | Delete a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_delete_node**](docs/KubernetesApi.md#kubernetes_delete_node) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}/nodes/{node_id} | Delete a Node in a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_delete_node_pool**](docs/KubernetesApi.md#kubernetes_delete_node_pool) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Delete a Node Pool in a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_destroy_associated_resources_dangerous**](docs/KubernetesApi.md#kubernetes_destroy_associated_resources_dangerous) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources/dangerous | Delete a Cluster and All of its Associated Resources (Dangerous)
*KubernetesApi* | [**kubernetes_destroy_associated_resources_selective**](docs/KubernetesApi.md#kubernetes_destroy_associated_resources_selective) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources/selective | Selectively Delete a Cluster and its Associated Resources
*KubernetesApi* | [**kubernetes_get_available_upgrades**](docs/KubernetesApi.md#kubernetes_get_available_upgrades) | **GET** /v2/kubernetes/clusters/{cluster_id}/upgrades | Retrieve Available Upgrades for an Existing Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_cluster**](docs/KubernetesApi.md#kubernetes_get_cluster) | **GET** /v2/kubernetes/clusters/{cluster_id} | Retrieve an Existing Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_cluster_lint_results**](docs/KubernetesApi.md#kubernetes_get_cluster_lint_results) | **GET** /v2/kubernetes/clusters/{cluster_id}/clusterlint | Fetch Clusterlint Diagnostics for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_cluster_user**](docs/KubernetesApi.md#kubernetes_get_cluster_user) | **GET** /v2/kubernetes/clusters/{cluster_id}/user | Retrieve User Information for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_credentials**](docs/KubernetesApi.md#kubernetes_get_credentials) | **GET** /v2/kubernetes/clusters/{cluster_id}/credentials | Retrieve Credentials for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_kubeconfig**](docs/KubernetesApi.md#kubernetes_get_kubeconfig) | **GET** /v2/kubernetes/clusters/{cluster_id}/kubeconfig | Retrieve the kubeconfig for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_node_pool**](docs/KubernetesApi.md#kubernetes_get_node_pool) | **GET** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Retrieve a Node Pool for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_list_associated_resources**](docs/KubernetesApi.md#kubernetes_list_associated_resources) | **GET** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources | List Associated Resources for Cluster Deletion
*KubernetesApi* | [**kubernetes_list_clusters**](docs/KubernetesApi.md#kubernetes_list_clusters) | **GET** /v2/kubernetes/clusters | List All Kubernetes Clusters
*KubernetesApi* | [**kubernetes_list_node_pools**](docs/KubernetesApi.md#kubernetes_list_node_pools) | **GET** /v2/kubernetes/clusters/{cluster_id}/node_pools | List All Node Pools in a Kubernetes Clusters
*KubernetesApi* | [**kubernetes_list_options**](docs/KubernetesApi.md#kubernetes_list_options) | **GET** /v2/kubernetes/options | List Available Regions, Node Sizes, and Versions of Kubernetes
*KubernetesApi* | [**kubernetes_recycle_node_pool**](docs/KubernetesApi.md#kubernetes_recycle_node_pool) | **POST** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}/recycle | Recycle a Kubernetes Node Pool
*KubernetesApi* | [**kubernetes_remove_registry**](docs/KubernetesApi.md#kubernetes_remove_registry) | **DELETE** /v2/kubernetes/registry | Remove Container Registry from Kubernetes Clusters
*KubernetesApi* | [**kubernetes_run_cluster_lint**](docs/KubernetesApi.md#kubernetes_run_cluster_lint) | **POST** /v2/kubernetes/clusters/{cluster_id}/clusterlint | Run Clusterlint Checks on a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_update_cluster**](docs/KubernetesApi.md#kubernetes_update_cluster) | **PUT** /v2/kubernetes/clusters/{cluster_id} | Update a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_update_node_pool**](docs/KubernetesApi.md#kubernetes_update_node_pool) | **PUT** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Update a Node Pool in a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_upgrade_cluster**](docs/KubernetesApi.md#kubernetes_upgrade_cluster) | **POST** /v2/kubernetes/clusters/{cluster_id}/upgrade | Upgrade a Kubernetes Cluster
*LoadBalancersApi* | [**load_balancers_add_droplets**](docs/LoadBalancersApi.md#load_balancers_add_droplets) | **POST** /v2/load_balancers/{lb_id}/droplets | Add Droplets to a Load Balancer
*LoadBalancersApi* | [**load_balancers_add_forwarding_rules**](docs/LoadBalancersApi.md#load_balancers_add_forwarding_rules) | **POST** /v2/load_balancers/{lb_id}/forwarding_rules | Add Forwarding Rules to a Load Balancer
*LoadBalancersApi* | [**load_balancers_create**](docs/LoadBalancersApi.md#load_balancers_create) | **POST** /v2/load_balancers | Create a New Load Balancer
*LoadBalancersApi* | [**load_balancers_delete**](docs/LoadBalancersApi.md#load_balancers_delete) | **DELETE** /v2/load_balancers/{lb_id} | Delete a Load Balancer
*LoadBalancersApi* | [**load_balancers_get**](docs/LoadBalancersApi.md#load_balancers_get) | **GET** /v2/load_balancers/{lb_id} | Retrieve an Existing Load Balancer
*LoadBalancersApi* | [**load_balancers_list**](docs/LoadBalancersApi.md#load_balancers_list) | **GET** /v2/load_balancers | List All Load Balancers
*LoadBalancersApi* | [**load_balancers_remove_droplets**](docs/LoadBalancersApi.md#load_balancers_remove_droplets) | **DELETE** /v2/load_balancers/{lb_id}/droplets | Remove Droplets from a Load Balancer
*LoadBalancersApi* | [**load_balancers_remove_forwarding_rules**](docs/LoadBalancersApi.md#load_balancers_remove_forwarding_rules) | **DELETE** /v2/load_balancers/{lb_id}/forwarding_rules | Remove Forwarding Rules from a Load Balancer
*LoadBalancersApi* | [**load_balancers_update**](docs/LoadBalancersApi.md#load_balancers_update) | **PUT** /v2/load_balancers/{lb_id} | Update a Load Balancer
*MonitoringApi* | [**monitoring_create_alert_policy**](docs/MonitoringApi.md#monitoring_create_alert_policy) | **POST** /v2/monitoring/alerts | Create Alert Policy
*MonitoringApi* | [**monitoring_delete_alert_policy**](docs/MonitoringApi.md#monitoring_delete_alert_policy) | **DELETE** /v2/monitoring/alerts/{alert_uuid} | Delete an Alert Policy
*MonitoringApi* | [**monitoring_get_alert_policy**](docs/MonitoringApi.md#monitoring_get_alert_policy) | **GET** /v2/monitoring/alerts/{alert_uuid} | Retrieve an Existing Alert Policy
*MonitoringApi* | [**monitoring_get_droplet_bandwidth_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_bandwidth_metrics) | **GET** /v2/monitoring/metrics/droplet/bandwidth | Get Droplet Bandwidth Metrics
*MonitoringApi* | [**monitoring_get_droplet_cpu_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_cpu_metrics) | **GET** /v2/monitoring/metrics/droplet/cpu | Get Droplet CPU Metrics
*MonitoringApi* | [**monitoring_get_droplet_filesystem_free_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_filesystem_free_metrics) | **GET** /v2/monitoring/metrics/droplet/filesystem_free | Get Droplet Filesystem Free Metrics
*MonitoringApi* | [**monitoring_get_droplet_filesystem_size_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_filesystem_size_metrics) | **GET** /v2/monitoring/metrics/droplet/filesystem_size | Get Droplet Filesystem Size Metrics
*MonitoringApi* | [**monitoring_get_droplet_load15_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_load15_metrics) | **GET** /v2/monitoring/metrics/droplet/load_15 | Get Droplet Load15 Metrics
*MonitoringApi* | [**monitoring_get_droplet_load1_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_load1_metrics) | **GET** /v2/monitoring/metrics/droplet/load_1 | Get Droplet Load1 Metrics
*MonitoringApi* | [**monitoring_get_droplet_load5_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_load5_metrics) | **GET** /v2/monitoring/metrics/droplet/load_5 | Get Droplet Load5 Metrics
*MonitoringApi* | [**monitoring_get_droplet_memory_available_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_memory_available_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_available | Get Droplet Available Memory Metrics
*MonitoringApi* | [**monitoring_get_droplet_memory_cached_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_memory_cached_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_cached | Get Droplet Cached Memory Metrics
*MonitoringApi* | [**monitoring_get_droplet_memory_free_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_memory_free_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_free | Get Droplet Free Memory Metrics
*MonitoringApi* | [**monitoring_get_droplet_memory_total_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_memory_total_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_total | Get Droplet Total Memory Metrics
*MonitoringApi* | [**monitoring_list_alert_policy**](docs/MonitoringApi.md#monitoring_list_alert_policy) | **GET** /v2/monitoring/alerts | List Alert Policies
*MonitoringApi* | [**monitoring_update_alert_policy**](docs/MonitoringApi.md#monitoring_update_alert_policy) | **PUT** /v2/monitoring/alerts/{alert_uuid} | Update an Alert Policy
*ProjectResourcesApi* | [**projects_assign_resources**](docs/ProjectResourcesApi.md#projects_assign_resources) | **POST** /v2/projects/{project_id}/resources | Assign Resources to a Project
*ProjectResourcesApi* | [**projects_assign_resources_default**](docs/ProjectResourcesApi.md#projects_assign_resources_default) | **POST** /v2/projects/default/resources | Assign Resources to Default Project
*ProjectResourcesApi* | [**projects_list_resources**](docs/ProjectResourcesApi.md#projects_list_resources) | **GET** /v2/projects/{project_id}/resources | List Project Resources
*ProjectResourcesApi* | [**projects_list_resources_default**](docs/ProjectResourcesApi.md#projects_list_resources_default) | **GET** /v2/projects/default/resources | List Default Project Resources
*ProjectsApi* | [**projects_create**](docs/ProjectsApi.md#projects_create) | **POST** /v2/projects | Create a Project
*ProjectsApi* | [**projects_delete**](docs/ProjectsApi.md#projects_delete) | **DELETE** /v2/projects/{project_id} | Delete an Existing Project
*ProjectsApi* | [**projects_get**](docs/ProjectsApi.md#projects_get) | **GET** /v2/projects/{project_id} | Retrieve an Existing Project
*ProjectsApi* | [**projects_get_default**](docs/ProjectsApi.md#projects_get_default) | **GET** /v2/projects/default | Retrieve the Default Project
*ProjectsApi* | [**projects_list**](docs/ProjectsApi.md#projects_list) | **GET** /v2/projects | List All Projects
*ProjectsApi* | [**projects_patch**](docs/ProjectsApi.md#projects_patch) | **PATCH** /v2/projects/{project_id} | Patch a Project
*ProjectsApi* | [**projects_patch_default**](docs/ProjectsApi.md#projects_patch_default) | **PATCH** /v2/projects/default | Patch the Default Project
*ProjectsApi* | [**projects_update**](docs/ProjectsApi.md#projects_update) | **PUT** /v2/projects/{project_id} | Update a Project
*ProjectsApi* | [**projects_update_default**](docs/ProjectsApi.md#projects_update_default) | **PUT** /v2/projects/default | Update the Default Project
*RegionsApi* | [**regions_list**](docs/RegionsApi.md#regions_list) | **GET** /v2/regions | List All Data Center Regions
*ReservedIpActionsApi* | [**reserved_ips_actions_get**](docs/ReservedIpActionsApi.md#reserved_ips_actions_get) | **GET** /v2/reserved_ips/{reserved_ip}/actions/{action_id} | Retrieve an Existing Reserved IP Action
*ReservedIpActionsApi* | [**reserved_ips_actions_list**](docs/ReservedIpActionsApi.md#reserved_ips_actions_list) | **GET** /v2/reserved_ips/{reserved_ip}/actions | List All Actions for a Reserved IP
*ReservedIpActionsApi* | [**reserved_ips_actions_post**](docs/ReservedIpActionsApi.md#reserved_ips_actions_post) | **POST** /v2/reserved_ips/{reserved_ip}/actions | Initiate a Reserved IP Action
*ReservedIpsApi* | [**reserved_ips_create**](docs/ReservedIpsApi.md#reserved_ips_create) | **POST** /v2/reserved_ips | Create a New Reserved IP
*ReservedIpsApi* | [**reserved_ips_delete**](docs/ReservedIpsApi.md#reserved_ips_delete) | **DELETE** /v2/reserved_ips/{reserved_ip} | Delete a Reserved IPs
*ReservedIpsApi* | [**reserved_ips_get**](docs/ReservedIpsApi.md#reserved_ips_get) | **GET** /v2/reserved_ips/{reserved_ip} | Retrieve an Existing Reserved IP
*ReservedIpsApi* | [**reserved_ips_list**](docs/ReservedIpsApi.md#reserved_ips_list) | **GET** /v2/reserved_ips | List All Reserved IPs
*SshKeysApi* | [**ssh_keys_create**](docs/SshKeysApi.md#ssh_keys_create) | **POST** /v2/account/keys | Create a New SSH Key
*SshKeysApi* | [**ssh_keys_delete**](docs/SshKeysApi.md#ssh_keys_delete) | **DELETE** /v2/account/keys/{ssh_key_identifier} | Delete an SSH Key
*SshKeysApi* | [**ssh_keys_get**](docs/SshKeysApi.md#ssh_keys_get) | **GET** /v2/account/keys/{ssh_key_identifier} | Retrieve an Existing SSH Key
*SshKeysApi* | [**ssh_keys_list**](docs/SshKeysApi.md#ssh_keys_list) | **GET** /v2/account/keys | List All SSH Keys
*SshKeysApi* | [**ssh_keys_update**](docs/SshKeysApi.md#ssh_keys_update) | **PUT** /v2/account/keys/{ssh_key_identifier} | Update an SSH Key's Name
*SizesApi* | [**sizes_list**](docs/SizesApi.md#sizes_list) | **GET** /v2/sizes | List All Droplet Sizes
*SnapshotsApi* | [**snapshots_delete**](docs/SnapshotsApi.md#snapshots_delete) | **DELETE** /v2/snapshots/{snapshot_id} | Delete a Snapshot
*SnapshotsApi* | [**snapshots_get**](docs/SnapshotsApi.md#snapshots_get) | **GET** /v2/snapshots/{snapshot_id} | Retrieve an Existing Snapshot
*SnapshotsApi* | [**snapshots_list**](docs/SnapshotsApi.md#snapshots_list) | **GET** /v2/snapshots | List All Snapshots
*TagsApi* | [**tags_assign_resources**](docs/TagsApi.md#tags_assign_resources) | **POST** /v2/tags/{tag_id}/resources | Tag a Resource
*TagsApi* | [**tags_create**](docs/TagsApi.md#tags_create) | **POST** /v2/tags | Create a New Tag
*TagsApi* | [**tags_delete**](docs/TagsApi.md#tags_delete) | **DELETE** /v2/tags/{tag_id} | Delete a Tag
*TagsApi* | [**tags_get**](docs/TagsApi.md#tags_get) | **GET** /v2/tags/{tag_id} | Retrieve a Tag
*TagsApi* | [**tags_list**](docs/TagsApi.md#tags_list) | **GET** /v2/tags | List All Tags
*TagsApi* | [**tags_unassign_resources**](docs/TagsApi.md#tags_unassign_resources) | **DELETE** /v2/tags/{tag_id}/resources | Untag a Resource
*UptimeApi* | [**uptime_alert_create**](docs/UptimeApi.md#uptime_alert_create) | **POST** /v2/uptime/checks/{check_id}/alerts | Create a New Alert
*UptimeApi* | [**uptime_alert_delete**](docs/UptimeApi.md#uptime_alert_delete) | **DELETE** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Delete an Alert
*UptimeApi* | [**uptime_alert_get**](docs/UptimeApi.md#uptime_alert_get) | **GET** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Retrieve an Existing Alert
*UptimeApi* | [**uptime_alert_update**](docs/UptimeApi.md#uptime_alert_update) | **PUT** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Update an Alert
*UptimeApi* | [**uptime_check_alerts_list**](docs/UptimeApi.md#uptime_check_alerts_list) | **GET** /v2/uptime/checks/{check_id}/alerts | List All Alerts
*UptimeApi* | [**uptime_check_create**](docs/UptimeApi.md#uptime_check_create) | **POST** /v2/uptime/checks | Create a New Check
*UptimeApi* | [**uptime_check_delete**](docs/UptimeApi.md#uptime_check_delete) | **DELETE** /v2/uptime/checks/{check_id} | Delete a Check
*UptimeApi* | [**uptime_check_get**](docs/UptimeApi.md#uptime_check_get) | **GET** /v2/uptime/checks/{check_id} | Retrieve an Existing Check
*UptimeApi* | [**uptime_check_state_get**](docs/UptimeApi.md#uptime_check_state_get) | **GET** /v2/uptime/checks/{check_id}/state | Retrieve Check State
*UptimeApi* | [**uptime_check_update**](docs/UptimeApi.md#uptime_check_update) | **PUT** /v2/uptime/checks/{check_id} | Update a Check
*UptimeApi* | [**uptime_checks_list**](docs/UptimeApi.md#uptime_checks_list) | **GET** /v2/uptime/checks | List All Checks
*VpcsApi* | [**vpcs_create**](docs/VpcsApi.md#vpcs_create) | **POST** /v2/vpcs | Create a New VPC
*VpcsApi* | [**vpcs_delete**](docs/VpcsApi.md#vpcs_delete) | **DELETE** /v2/vpcs/{vpc_id} | Delete a VPC
*VpcsApi* | [**vpcs_get**](docs/VpcsApi.md#vpcs_get) | **GET** /v2/vpcs/{vpc_id} | Retrieve an Existing VPC
*VpcsApi* | [**vpcs_list**](docs/VpcsApi.md#vpcs_list) | **GET** /v2/vpcs | List All VPCs
*VpcsApi* | [**vpcs_list_members**](docs/VpcsApi.md#vpcs_list_members) | **GET** /v2/vpcs/{vpc_id}/members | List the Member Resources of a VPC
*VpcsApi* | [**vpcs_patch**](docs/VpcsApi.md#vpcs_patch) | **PATCH** /v2/vpcs/{vpc_id} | Partially Update a VPC
*VpcsApi* | [**vpcs_update**](docs/VpcsApi.md#vpcs_update) | **PUT** /v2/vpcs/{vpc_id} | Update a VPC


## Documentation For Models

 - [Account](docs/Account.md)
 - [AccountGet200Response](docs/AccountGet200Response.md)
 - [AccountTeam](docs/AccountTeam.md)
 - [Action](docs/Action.md)
 - [ActionLink](docs/ActionLink.md)
 - [ActionsGet200Response](docs/ActionsGet200Response.md)
 - [ActionsList200Response](docs/ActionsList200Response.md)
 - [ActionsList200ResponseAllOf](docs/ActionsList200ResponseAllOf.md)
 - [Alert](docs/Alert.md)
 - [AlertBase](docs/AlertBase.md)
 - [AlertPolicy](docs/AlertPolicy.md)
 - [AlertPolicyRequest](docs/AlertPolicyRequest.md)
 - [AlertUpdatable](docs/AlertUpdatable.md)
 - [Alerts](docs/Alerts.md)
 - [App](docs/App.md)
 - [AppAlert](docs/AppAlert.md)
 - [AppAlertPhase](docs/AppAlertPhase.md)
 - [AppAlertProgress](docs/AppAlertProgress.md)
 - [AppAlertProgressStep](docs/AppAlertProgressStep.md)
 - [AppAlertProgressStepReason](docs/AppAlertProgressStepReason.md)
 - [AppAlertProgressStepStatus](docs/AppAlertProgressStepStatus.md)
 - [AppAlertSlackWebhook](docs/AppAlertSlackWebhook.md)
 - [AppAlertSpec](docs/AppAlertSpec.md)
 - [AppAlertSpecOperator](docs/AppAlertSpecOperator.md)
 - [AppAlertSpecRule](docs/AppAlertSpecRule.md)
 - [AppAlertSpecWindow](docs/AppAlertSpecWindow.md)
 - [AppComponentBase](docs/AppComponentBase.md)
 - [AppComponentInstanceBase](docs/AppComponentInstanceBase.md)
 - [AppDatabaseSpec](docs/AppDatabaseSpec.md)
 - [AppDomainSpec](docs/AppDomainSpec.md)
 - [AppFunctionsSpec](docs/AppFunctionsSpec.md)
 - [AppJobSpec](docs/AppJobSpec.md)
 - [AppJobSpecAllOf](docs/AppJobSpecAllOf.md)
 - [AppLogDestinationDatadogSpec](docs/AppLogDestinationDatadogSpec.md)
 - [AppLogDestinationDefinition](docs/AppLogDestinationDefinition.md)
 - [AppLogDestinationLogtailSpec](docs/AppLogDestinationLogtailSpec.md)
 - [AppLogDestinationPapertrailSpec](docs/AppLogDestinationPapertrailSpec.md)
 - [AppPinnedDeployment](docs/AppPinnedDeployment.md)
 - [AppPropose](docs/AppPropose.md)
 - [AppProposeResponse](docs/AppProposeResponse.md)
 - [AppResponse](docs/AppResponse.md)
 - [AppRollbackValidationCondition](docs/AppRollbackValidationCondition.md)
 - [AppRouteSpec](docs/AppRouteSpec.md)
 - [AppServiceSpec](docs/AppServiceSpec.md)
 - [AppServiceSpecAllOf](docs/AppServiceSpecAllOf.md)
 - [AppServiceSpecHealthCheck](docs/AppServiceSpecHealthCheck.md)
 - [AppSpec](docs/AppSpec.md)
 - [AppStaticSiteSpec](docs/AppStaticSiteSpec.md)
 - [AppStaticSiteSpecAllOf](docs/AppStaticSiteSpecAllOf.md)
 - [AppVariableDefinition](docs/AppVariableDefinition.md)
 - [AppWorkerSpec](docs/AppWorkerSpec.md)
 - [AppsAlertResponse](docs/AppsAlertResponse.md)
 - [AppsAssignAppAlertDestinationsRequest](docs/AppsAssignAppAlertDestinationsRequest.md)
 - [AppsCorsPolicy](docs/AppsCorsPolicy.md)
 - [AppsCreateAppRequest](docs/AppsCreateAppRequest.md)
 - [AppsCreateDeploymentRequest](docs/AppsCreateDeploymentRequest.md)
 - [AppsDeleteAppResponse](docs/AppsDeleteAppResponse.md)
 - [AppsDeployment](docs/AppsDeployment.md)
 - [AppsDeploymentFunctions](docs/AppsDeploymentFunctions.md)
 - [AppsDeploymentJob](docs/AppsDeploymentJob.md)
 - [AppsDeploymentPhase](docs/AppsDeploymentPhase.md)
 - [AppsDeploymentProgress](docs/AppsDeploymentProgress.md)
 - [AppsDeploymentProgressStep](docs/AppsDeploymentProgressStep.md)
 - [AppsDeploymentProgressStepReason](docs/AppsDeploymentProgressStepReason.md)
 - [AppsDeploymentProgressStepStatus](docs/AppsDeploymentProgressStepStatus.md)
 - [AppsDeploymentResponse](docs/AppsDeploymentResponse.md)
 - [AppsDeploymentService](docs/AppsDeploymentService.md)
 - [AppsDeploymentStaticSite](docs/AppsDeploymentStaticSite.md)
 - [AppsDeploymentWorker](docs/AppsDeploymentWorker.md)
 - [AppsDeploymentsResponse](docs/AppsDeploymentsResponse.md)
 - [AppsDeploymentsResponseAllOf](docs/AppsDeploymentsResponseAllOf.md)
 - [AppsDomain](docs/AppsDomain.md)
 - [AppsDomainPhase](docs/AppsDomainPhase.md)
 - [AppsDomainProgress](docs/AppsDomainProgress.md)
 - [AppsGetInstanceSizeResponse](docs/AppsGetInstanceSizeResponse.md)
 - [AppsGetLogsResponse](docs/AppsGetLogsResponse.md)
 - [AppsGetTierResponse](docs/AppsGetTierResponse.md)
 - [AppsGitSourceSpec](docs/AppsGitSourceSpec.md)
 - [AppsGithubSourceSpec](docs/AppsGithubSourceSpec.md)
 - [AppsGitlabSourceSpec](docs/AppsGitlabSourceSpec.md)
 - [AppsImageSourceSpec](docs/AppsImageSourceSpec.md)
 - [AppsInstanceSize](docs/AppsInstanceSize.md)
 - [AppsListAlertsResponse](docs/AppsListAlertsResponse.md)
 - [AppsListInstanceSizesResponse](docs/AppsListInstanceSizesResponse.md)
 - [AppsListRegionsResponse](docs/AppsListRegionsResponse.md)
 - [AppsListTiersResponse](docs/AppsListTiersResponse.md)
 - [AppsRegion](docs/AppsRegion.md)
 - [AppsResponse](docs/AppsResponse.md)
 - [AppsResponseAllOf](docs/AppsResponseAllOf.md)
 - [AppsRollbackAppRequest](docs/AppsRollbackAppRequest.md)
 - [AppsStringMatch](docs/AppsStringMatch.md)
 - [AppsTier](docs/AppsTier.md)
 - [AppsUpdateAppRequest](docs/AppsUpdateAppRequest.md)
 - [AppsValidateRollback200Response](docs/AppsValidateRollback200Response.md)
 - [AppsValidateRollback200ResponseError](docs/AppsValidateRollback200ResponseError.md)
 - [AssignToDroplet](docs/AssignToDroplet.md)
 - [AssignToDroplet1](docs/AssignToDroplet1.md)
 - [AssociatedKubernetesResource](docs/AssociatedKubernetesResource.md)
 - [AssociatedKubernetesResources](docs/AssociatedKubernetesResources.md)
 - [AssociatedResource](docs/AssociatedResource.md)
 - [AssociatedResourceStatus](docs/AssociatedResourceStatus.md)
 - [AssociatedResourceStatusResources](docs/AssociatedResourceStatusResources.md)
 - [Backup](docs/Backup.md)
 - [BackwardLinks](docs/BackwardLinks.md)
 - [Balance](docs/Balance.md)
 - [BillingAddress](docs/BillingAddress.md)
 - [BillingHistory](docs/BillingHistory.md)
 - [BillingHistoryList200Response](docs/BillingHistoryList200Response.md)
 - [BillingHistoryList200ResponseAllOf](docs/BillingHistoryList200ResponseAllOf.md)
 - [Ca](docs/Ca.md)
 - [CdnCreateEndpoint201Response](docs/CdnCreateEndpoint201Response.md)
 - [CdnEndpoint](docs/CdnEndpoint.md)
 - [CdnListEndpoints200Response](docs/CdnListEndpoints200Response.md)
 - [CdnListEndpoints200ResponseAllOf](docs/CdnListEndpoints200ResponseAllOf.md)
 - [Certificate](docs/Certificate.md)
 - [CertificateCreateBase](docs/CertificateCreateBase.md)
 - [CertificateRequestCustom](docs/CertificateRequestCustom.md)
 - [CertificateRequestCustomAllOf](docs/CertificateRequestCustomAllOf.md)
 - [CertificateRequestLetsEncrypt](docs/CertificateRequestLetsEncrypt.md)
 - [CertificateRequestLetsEncryptAllOf](docs/CertificateRequestLetsEncryptAllOf.md)
 - [CertificatesCreate201Response](docs/CertificatesCreate201Response.md)
 - [CertificatesCreateRequest](docs/CertificatesCreateRequest.md)
 - [CertificatesList200Response](docs/CertificatesList200Response.md)
 - [CertificatesList200ResponseAllOf](docs/CertificatesList200ResponseAllOf.md)
 - [Check](docs/Check.md)
 - [CheckBase](docs/CheckBase.md)
 - [CheckUpdatable](docs/CheckUpdatable.md)
 - [Cluster](docs/Cluster.md)
 - [ClusterRegistries](docs/ClusterRegistries.md)
 - [ClusterStatus](docs/ClusterStatus.md)
 - [ClusterUpdate](docs/ClusterUpdate.md)
 - [ClusterlintRequest](docs/ClusterlintRequest.md)
 - [ClusterlintResults](docs/ClusterlintResults.md)
 - [ClusterlintResultsDiagnosticsInner](docs/ClusterlintResultsDiagnosticsInner.md)
 - [ClusterlintResultsDiagnosticsInnerObject](docs/ClusterlintResultsDiagnosticsInnerObject.md)
 - [ConnectionPool](docs/ConnectionPool.md)
 - [ConnectionPools](docs/ConnectionPools.md)
 - [CreateNamespace](docs/CreateNamespace.md)
 - [Credentials](docs/Credentials.md)
 - [Database](docs/Database.md)
 - [DatabaseBackup](docs/DatabaseBackup.md)
 - [DatabaseCluster](docs/DatabaseCluster.md)
 - [DatabaseClusterConnection](docs/DatabaseClusterConnection.md)
 - [DatabaseClusterMaintenanceWindow](docs/DatabaseClusterMaintenanceWindow.md)
 - [DatabaseClusterResize](docs/DatabaseClusterResize.md)
 - [DatabaseConfig](docs/DatabaseConfig.md)
 - [DatabaseConnection](docs/DatabaseConnection.md)
 - [DatabaseLayoutOption](docs/DatabaseLayoutOption.md)
 - [DatabaseLayoutOptions](docs/DatabaseLayoutOptions.md)
 - [DatabaseMaintenanceWindow](docs/DatabaseMaintenanceWindow.md)
 - [DatabaseRegionOptions](docs/DatabaseRegionOptions.md)
 - [DatabaseReplica](docs/DatabaseReplica.md)
 - [DatabaseReplicaConnection](docs/DatabaseReplicaConnection.md)
 - [DatabaseUser](docs/DatabaseUser.md)
 - [DatabaseVersionOptions](docs/DatabaseVersionOptions.md)
 - [DatabasesAdd201Response](docs/DatabasesAdd201Response.md)
 - [DatabasesAddConnectionPool201Response](docs/DatabasesAddConnectionPool201Response.md)
 - [DatabasesAddUser201Response](docs/DatabasesAddUser201Response.md)
 - [DatabasesCreateCluster201Response](docs/DatabasesCreateCluster201Response.md)
 - [DatabasesCreateClusterRequest](docs/DatabasesCreateClusterRequest.md)
 - [DatabasesCreateClusterRequestAllOf](docs/DatabasesCreateClusterRequestAllOf.md)
 - [DatabasesCreateReplica201Response](docs/DatabasesCreateReplica201Response.md)
 - [DatabasesCreateReplicaRequest](docs/DatabasesCreateReplicaRequest.md)
 - [DatabasesGetCa200Response](docs/DatabasesGetCa200Response.md)
 - [DatabasesGetConfig200Response](docs/DatabasesGetConfig200Response.md)
 - [DatabasesGetConfig200ResponseConfig](docs/DatabasesGetConfig200ResponseConfig.md)
 - [DatabasesGetEvictionPolicy200Response](docs/DatabasesGetEvictionPolicy200Response.md)
 - [DatabasesList200Response](docs/DatabasesList200Response.md)
 - [DatabasesListBackups200Response](docs/DatabasesListBackups200Response.md)
 - [DatabasesListClusters200Response](docs/DatabasesListClusters200Response.md)
 - [DatabasesListFirewallRules200Response](docs/DatabasesListFirewallRules200Response.md)
 - [DatabasesListReplicas200Response](docs/DatabasesListReplicas200Response.md)
 - [DatabasesListUsers200Response](docs/DatabasesListUsers200Response.md)
 - [DatabasesResetAuthRequest](docs/DatabasesResetAuthRequest.md)
 - [DatabasesUpdateEvictionPolicyRequest](docs/DatabasesUpdateEvictionPolicyRequest.md)
 - [DatabasesUpdateFirewallRulesRequest](docs/DatabasesUpdateFirewallRulesRequest.md)
 - [DatabasesUpdateRegionRequest](docs/DatabasesUpdateRegionRequest.md)
 - [DestroyAssociatedKubernetesResources](docs/DestroyAssociatedKubernetesResources.md)
 - [DestroyedAssociatedResource](docs/DestroyedAssociatedResource.md)
 - [Distribution](docs/Distribution.md)
 - [DockerCredentials](docs/DockerCredentials.md)
 - [DockerCredentialsAuths](docs/DockerCredentialsAuths.md)
 - [DockerCredentialsAuthsRegistryDigitaloceanCom](docs/DockerCredentialsAuthsRegistryDigitaloceanCom.md)
 - [Domain](docs/Domain.md)
 - [DomainRecord](docs/DomainRecord.md)
 - [DomainRecordA](docs/DomainRecordA.md)
 - [DomainRecordAaaa](docs/DomainRecordAaaa.md)
 - [DomainRecordCaa](docs/DomainRecordCaa.md)
 - [DomainRecordCname](docs/DomainRecordCname.md)
 - [DomainRecordMx](docs/DomainRecordMx.md)
 - [DomainRecordNs](docs/DomainRecordNs.md)
 - [DomainRecordSoa](docs/DomainRecordSoa.md)
 - [DomainRecordSrv](docs/DomainRecordSrv.md)
 - [DomainRecordTxt](docs/DomainRecordTxt.md)
 - [DomainsCreate201Response](docs/DomainsCreate201Response.md)
 - [DomainsCreateRecord201Response](docs/DomainsCreateRecord201Response.md)
 - [DomainsCreateRecordRequest](docs/DomainsCreateRecordRequest.md)
 - [DomainsGet200Response](docs/DomainsGet200Response.md)
 - [DomainsGetRecord200Response](docs/DomainsGetRecord200Response.md)
 - [DomainsList200Response](docs/DomainsList200Response.md)
 - [DomainsList200ResponseAllOf](docs/DomainsList200ResponseAllOf.md)
 - [DomainsListRecords200Response](docs/DomainsListRecords200Response.md)
 - [DomainsListRecords200ResponseAllOf](docs/DomainsListRecords200ResponseAllOf.md)
 - [Droplet](docs/Droplet.md)
 - [DropletAction](docs/DropletAction.md)
 - [DropletActionChangeKernel](docs/DropletActionChangeKernel.md)
 - [DropletActionChangeKernelAllOf](docs/DropletActionChangeKernelAllOf.md)
 - [DropletActionRebuild](docs/DropletActionRebuild.md)
 - [DropletActionRebuildAllOf](docs/DropletActionRebuildAllOf.md)
 - [DropletActionRename](docs/DropletActionRename.md)
 - [DropletActionRenameAllOf](docs/DropletActionRenameAllOf.md)
 - [DropletActionResize](docs/DropletActionResize.md)
 - [DropletActionResizeAllOf](docs/DropletActionResizeAllOf.md)
 - [DropletActionRestore](docs/DropletActionRestore.md)
 - [DropletActionRestoreAllOf](docs/DropletActionRestoreAllOf.md)
 - [DropletActionSnapshot](docs/DropletActionSnapshot.md)
 - [DropletActionSnapshotAllOf](docs/DropletActionSnapshotAllOf.md)
 - [DropletActionsPostByTag201Response](docs/DropletActionsPostByTag201Response.md)
 - [DropletActionsPostByTagRequest](docs/DropletActionsPostByTagRequest.md)
 - [DropletActionsPostRequest](docs/DropletActionsPostRequest.md)
 - [DropletCreate](docs/DropletCreate.md)
 - [DropletCreateImage](docs/DropletCreateImage.md)
 - [DropletCreateSshKeysInner](docs/DropletCreateSshKeysInner.md)
 - [DropletMultiCreate](docs/DropletMultiCreate.md)
 - [DropletMultiCreateAllOf](docs/DropletMultiCreateAllOf.md)
 - [DropletNetworks](docs/DropletNetworks.md)
 - [DropletNextBackupWindow](docs/DropletNextBackupWindow.md)
 - [DropletSingleCreate](docs/DropletSingleCreate.md)
 - [DropletSingleCreateAllOf](docs/DropletSingleCreateAllOf.md)
 - [DropletSnapshot](docs/DropletSnapshot.md)
 - [DropletSnapshotAllOf](docs/DropletSnapshotAllOf.md)
 - [DropletSnapshotAllOf1](docs/DropletSnapshotAllOf1.md)
 - [DropletsCreate202Response](docs/DropletsCreate202Response.md)
 - [DropletsCreateRequest](docs/DropletsCreateRequest.md)
 - [DropletsGet200Response](docs/DropletsGet200Response.md)
 - [DropletsList200Response](docs/DropletsList200Response.md)
 - [DropletsList200ResponseAllOf](docs/DropletsList200ResponseAllOf.md)
 - [DropletsListAssociatedResources200Response](docs/DropletsListAssociatedResources200Response.md)
 - [DropletsListAssociatedResources200ResponseAllOf](docs/DropletsListAssociatedResources200ResponseAllOf.md)
 - [DropletsListBackups200Response](docs/DropletsListBackups200Response.md)
 - [DropletsListBackups200ResponseAllOf](docs/DropletsListBackups200ResponseAllOf.md)
 - [DropletsListFirewalls200Response](docs/DropletsListFirewalls200Response.md)
 - [DropletsListFirewalls200ResponseAllOf](docs/DropletsListFirewalls200ResponseAllOf.md)
 - [DropletsListKernels200Response](docs/DropletsListKernels200Response.md)
 - [DropletsListKernels200ResponseAllOf](docs/DropletsListKernels200ResponseAllOf.md)
 - [DropletsListNeighbors200Response](docs/DropletsListNeighbors200Response.md)
 - [DropletsListSnapshots200Response](docs/DropletsListSnapshots200Response.md)
 - [DropletsListSnapshots200ResponseAllOf](docs/DropletsListSnapshots200ResponseAllOf.md)
 - [Error](docs/Error.md)
 - [ErrorWithRootCauses](docs/ErrorWithRootCauses.md)
 - [EvictionPolicyModel](docs/EvictionPolicyModel.md)
 - [Firewall](docs/Firewall.md)
 - [FirewallAllOf](docs/FirewallAllOf.md)
 - [FirewallAllOfPendingChanges](docs/FirewallAllOfPendingChanges.md)
 - [FirewallRule](docs/FirewallRule.md)
 - [FirewallRuleBase](docs/FirewallRuleBase.md)
 - [FirewallRuleTarget](docs/FirewallRuleTarget.md)
 - [FirewallRules](docs/FirewallRules.md)
 - [FirewallRulesInboundRulesInner](docs/FirewallRulesInboundRulesInner.md)
 - [FirewallRulesInboundRulesInnerAllOf](docs/FirewallRulesInboundRulesInnerAllOf.md)
 - [FirewallRulesInboundRulesInnerAllOfSources](docs/FirewallRulesInboundRulesInnerAllOfSources.md)
 - [FirewallRulesOutboundRulesInner](docs/FirewallRulesOutboundRulesInner.md)
 - [FirewallRulesOutboundRulesInnerAllOf](docs/FirewallRulesOutboundRulesInnerAllOf.md)
 - [FirewallRulesOutboundRulesInnerAllOfDestinations](docs/FirewallRulesOutboundRulesInnerAllOfDestinations.md)
 - [FirewallsAddRulesRequest](docs/FirewallsAddRulesRequest.md)
 - [FirewallsAddTagsRequest](docs/FirewallsAddTagsRequest.md)
 - [FirewallsAssignDropletsRequest](docs/FirewallsAssignDropletsRequest.md)
 - [FirewallsCreate202Response](docs/FirewallsCreate202Response.md)
 - [FirewallsCreateRequest](docs/FirewallsCreateRequest.md)
 - [FirewallsCreateRequestAllOf](docs/FirewallsCreateRequestAllOf.md)
 - [FirewallsDeleteDropletsRequest](docs/FirewallsDeleteDropletsRequest.md)
 - [FirewallsDeleteTagsRequest](docs/FirewallsDeleteTagsRequest.md)
 - [FirewallsUpdateRequest](docs/FirewallsUpdateRequest.md)
 - [FloatingIp](docs/FloatingIp.md)
 - [FloatingIpActionAssign](docs/FloatingIpActionAssign.md)
 - [FloatingIpActionAssignAllOf](docs/FloatingIpActionAssignAllOf.md)
 - [FloatingIpActionUnassign](docs/FloatingIpActionUnassign.md)
 - [FloatingIpCreate](docs/FloatingIpCreate.md)
 - [FloatingIpDroplet](docs/FloatingIpDroplet.md)
 - [FloatingIpRegion](docs/FloatingIpRegion.md)
 - [FloatingIpsAction](docs/FloatingIpsAction.md)
 - [FloatingIpsActionList200Response](docs/FloatingIpsActionList200Response.md)
 - [FloatingIpsActionPost201Response](docs/FloatingIpsActionPost201Response.md)
 - [FloatingIpsActionPost201ResponseAction](docs/FloatingIpsActionPost201ResponseAction.md)
 - [FloatingIpsActionPost201ResponseActionAllOf](docs/FloatingIpsActionPost201ResponseActionAllOf.md)
 - [FloatingIpsActionPostRequest](docs/FloatingIpsActionPostRequest.md)
 - [FloatingIpsCreate202Response](docs/FloatingIpsCreate202Response.md)
 - [FloatingIpsCreate202ResponseLinks](docs/FloatingIpsCreate202ResponseLinks.md)
 - [FloatingIpsGet200Response](docs/FloatingIpsGet200Response.md)
 - [FloatingIpsList200Response](docs/FloatingIpsList200Response.md)
 - [FloatingIpsList200ResponseAllOf](docs/FloatingIpsList200ResponseAllOf.md)
 - [ForwardLinks](docs/ForwardLinks.md)
 - [ForwardingRule](docs/ForwardingRule.md)
 - [FunctionsCreateNamespace200Response](docs/FunctionsCreateNamespace200Response.md)
 - [FunctionsListNamespaces200Response](docs/FunctionsListNamespaces200Response.md)
 - [FunctionsListNamespaces200ResponseAllOf](docs/FunctionsListNamespaces200ResponseAllOf.md)
 - [GarbageCollection](docs/GarbageCollection.md)
 - [HealthCheck](docs/HealthCheck.md)
 - [Image](docs/Image.md)
 - [ImageActionBase](docs/ImageActionBase.md)
 - [ImageActionTransfer](docs/ImageActionTransfer.md)
 - [ImageActionTransferAllOf](docs/ImageActionTransferAllOf.md)
 - [ImageActionsPostRequest](docs/ImageActionsPostRequest.md)
 - [ImageNewCustom](docs/ImageNewCustom.md)
 - [ImageNewCustomAllOf](docs/ImageNewCustomAllOf.md)
 - [ImageUpdate](docs/ImageUpdate.md)
 - [ImagesCreateCustom202Response](docs/ImagesCreateCustom202Response.md)
 - [ImagesGet200Response](docs/ImagesGet200Response.md)
 - [ImagesGetImageIdParameter](docs/ImagesGetImageIdParameter.md)
 - [ImagesList200Response](docs/ImagesList200Response.md)
 - [ImagesList200ResponseAllOf](docs/ImagesList200ResponseAllOf.md)
 - [InstanceSizeCpuType](docs/InstanceSizeCpuType.md)
 - [InvoiceItem](docs/InvoiceItem.md)
 - [InvoicePreview](docs/InvoicePreview.md)
 - [InvoiceSummary](docs/InvoiceSummary.md)
 - [InvoiceSummaryCreditsAndAdjustments](docs/InvoiceSummaryCreditsAndAdjustments.md)
 - [InvoiceSummaryOverages](docs/InvoiceSummaryOverages.md)
 - [InvoiceSummaryProductCharges](docs/InvoiceSummaryProductCharges.md)
 - [InvoiceSummaryTaxes](docs/InvoiceSummaryTaxes.md)
 - [InvoiceSummaryUserBillingAddress](docs/InvoiceSummaryUserBillingAddress.md)
 - [InvoicesGetByUuid200Response](docs/InvoicesGetByUuid200Response.md)
 - [InvoicesGetByUuid200ResponseAllOf](docs/InvoicesGetByUuid200ResponseAllOf.md)
 - [InvoicesList200Response](docs/InvoicesList200Response.md)
 - [InvoicesList200ResponseAllOf](docs/InvoicesList200ResponseAllOf.md)
 - [Kernel](docs/Kernel.md)
 - [KubernetesAddNodePool201Response](docs/KubernetesAddNodePool201Response.md)
 - [KubernetesCreateCluster201Response](docs/KubernetesCreateCluster201Response.md)
 - [KubernetesGetAvailableUpgrades200Response](docs/KubernetesGetAvailableUpgrades200Response.md)
 - [KubernetesGetNodePool200Response](docs/KubernetesGetNodePool200Response.md)
 - [KubernetesListClusters200Response](docs/KubernetesListClusters200Response.md)
 - [KubernetesListClusters200ResponseAllOf](docs/KubernetesListClusters200ResponseAllOf.md)
 - [KubernetesListNodePools200Response](docs/KubernetesListNodePools200Response.md)
 - [KubernetesNodePool](docs/KubernetesNodePool.md)
 - [KubernetesNodePoolBase](docs/KubernetesNodePoolBase.md)
 - [KubernetesNodePoolSize](docs/KubernetesNodePoolSize.md)
 - [KubernetesNodePoolTaint](docs/KubernetesNodePoolTaint.md)
 - [KubernetesNodePoolUpdate](docs/KubernetesNodePoolUpdate.md)
 - [KubernetesOptions](docs/KubernetesOptions.md)
 - [KubernetesOptionsOptions](docs/KubernetesOptionsOptions.md)
 - [KubernetesRecycleNodePoolRequest](docs/KubernetesRecycleNodePoolRequest.md)
 - [KubernetesRegion](docs/KubernetesRegion.md)
 - [KubernetesRunClusterLint202Response](docs/KubernetesRunClusterLint202Response.md)
 - [KubernetesSize](docs/KubernetesSize.md)
 - [KubernetesUpdateNodePool202Response](docs/KubernetesUpdateNodePool202Response.md)
 - [KubernetesUpgradeClusterRequest](docs/KubernetesUpgradeClusterRequest.md)
 - [KubernetesVersion](docs/KubernetesVersion.md)
 - [LinkToFirstPage](docs/LinkToFirstPage.md)
 - [LinkToLastPage](docs/LinkToLastPage.md)
 - [LinkToNextPage](docs/LinkToNextPage.md)
 - [LinkToPrevPage](docs/LinkToPrevPage.md)
 - [ListAlertPolicy](docs/ListAlertPolicy.md)
 - [LoadBalancer](docs/LoadBalancer.md)
 - [LoadBalancerAllOf](docs/LoadBalancerAllOf.md)
 - [LoadBalancerAllOf1](docs/LoadBalancerAllOf1.md)
 - [LoadBalancerAllOf2](docs/LoadBalancerAllOf2.md)
 - [LoadBalancerBase](docs/LoadBalancerBase.md)
 - [LoadBalancerCreate](docs/LoadBalancerCreate.md)
 - [LoadBalancersAddDropletsRequest](docs/LoadBalancersAddDropletsRequest.md)
 - [LoadBalancersAddForwardingRulesRequest](docs/LoadBalancersAddForwardingRulesRequest.md)
 - [LoadBalancersCreate202Response](docs/LoadBalancersCreate202Response.md)
 - [LoadBalancersList200Response](docs/LoadBalancersList200Response.md)
 - [LoadBalancersList200ResponseAllOf](docs/LoadBalancersList200ResponseAllOf.md)
 - [MaintenancePolicy](docs/MaintenancePolicy.md)
 - [Meta](docs/Meta.md)
 - [MetaMeta](docs/MetaMeta.md)
 - [MetaOptionalTotal](docs/MetaOptionalTotal.md)
 - [MetaProperties](docs/MetaProperties.md)
 - [Metrics](docs/Metrics.md)
 - [MetricsData](docs/MetricsData.md)
 - [MetricsResult](docs/MetricsResult.md)
 - [MetricsResultValuesInnerInner](docs/MetricsResultValuesInnerInner.md)
 - [MonitoringCreateAlertPolicy200Response](docs/MonitoringCreateAlertPolicy200Response.md)
 - [MonitoringListAlertPolicy200Response](docs/MonitoringListAlertPolicy200Response.md)
 - [MultipleDropletResponse](docs/MultipleDropletResponse.md)
 - [Mysql](docs/Mysql.md)
 - [MysqlSettings](docs/MysqlSettings.md)
 - [NamespaceInfo](docs/NamespaceInfo.md)
 - [NeighborIds](docs/NeighborIds.md)
 - [NetworkV4](docs/NetworkV4.md)
 - [NetworkV6](docs/NetworkV6.md)
 - [Node](docs/Node.md)
 - [NodeStatus](docs/NodeStatus.md)
 - [Notification](docs/Notification.md)
 - [NotificationSlackInner](docs/NotificationSlackInner.md)
 - [OneClicks](docs/OneClicks.md)
 - [OneClicksCreate](docs/OneClicksCreate.md)
 - [OneClicksInstallKubernetes200Response](docs/OneClicksInstallKubernetes200Response.md)
 - [OneClicksList200Response](docs/OneClicksList200Response.md)
 - [OnlineMigration](docs/OnlineMigration.md)
 - [Options](docs/Options.md)
 - [OptionsOptions](docs/OptionsOptions.md)
 - [OptionsOptionsMongodb](docs/OptionsOptionsMongodb.md)
 - [PageLinks](docs/PageLinks.md)
 - [PageLinksPages](docs/PageLinksPages.md)
 - [Pagination](docs/Pagination.md)
 - [Pgbouncer](docs/Pgbouncer.md)
 - [Postgres](docs/Postgres.md)
 - [PreviousOutage](docs/PreviousOutage.md)
 - [ProductChargeItem](docs/ProductChargeItem.md)
 - [ProductUsageCharges](docs/ProductUsageCharges.md)
 - [Project](docs/Project.md)
 - [ProjectAllOf](docs/ProjectAllOf.md)
 - [ProjectAssignment](docs/ProjectAssignment.md)
 - [ProjectBase](docs/ProjectBase.md)
 - [ProjectsAssignResources200Response](docs/ProjectsAssignResources200Response.md)
 - [ProjectsCreate201Response](docs/ProjectsCreate201Response.md)
 - [ProjectsCreateRequest](docs/ProjectsCreateRequest.md)
 - [ProjectsGetDefault200Response](docs/ProjectsGetDefault200Response.md)
 - [ProjectsList200Response](docs/ProjectsList200Response.md)
 - [ProjectsList200ResponseAllOf](docs/ProjectsList200ResponseAllOf.md)
 - [ProjectsListResources200Response](docs/ProjectsListResources200Response.md)
 - [ProjectsListResources200ResponseAllOf](docs/ProjectsListResources200ResponseAllOf.md)
 - [ProjectsUpdateDefaultRequest](docs/ProjectsUpdateDefaultRequest.md)
 - [PurgeCache](docs/PurgeCache.md)
 - [Redis](docs/Redis.md)
 - [Region](docs/Region.md)
 - [RegionSlug](docs/RegionSlug.md)
 - [RegionState](docs/RegionState.md)
 - [RegionalState](docs/RegionalState.md)
 - [RegionsList200Response](docs/RegionsList200Response.md)
 - [RegionsList200ResponseAllOf](docs/RegionsList200ResponseAllOf.md)
 - [Registry](docs/Registry.md)
 - [RegistryCreate](docs/RegistryCreate.md)
 - [RegistryGet200Response](docs/RegistryGet200Response.md)
 - [RegistryGetGarbageCollection200Response](docs/RegistryGetGarbageCollection200Response.md)
 - [RegistryGetOptions200Response](docs/RegistryGetOptions200Response.md)
 - [RegistryGetOptions200ResponseOptions](docs/RegistryGetOptions200ResponseOptions.md)
 - [RegistryGetOptions200ResponseOptionsSubscriptionTiersInner](docs/RegistryGetOptions200ResponseOptionsSubscriptionTiersInner.md)
 - [RegistryGetSubscription200Response](docs/RegistryGetSubscription200Response.md)
 - [RegistryListGarbageCollections200Response](docs/RegistryListGarbageCollections200Response.md)
 - [RegistryListRepositories200Response](docs/RegistryListRepositories200Response.md)
 - [RegistryListRepositories200ResponseAllOf](docs/RegistryListRepositories200ResponseAllOf.md)
 - [RegistryListRepositoriesV2200Response](docs/RegistryListRepositoriesV2200Response.md)
 - [RegistryListRepositoriesV2200ResponseAllOf](docs/RegistryListRepositoriesV2200ResponseAllOf.md)
 - [RegistryListRepositoryManifests200Response](docs/RegistryListRepositoryManifests200Response.md)
 - [RegistryListRepositoryManifests200ResponseAllOf](docs/RegistryListRepositoryManifests200ResponseAllOf.md)
 - [RegistryListRepositoryTags200Response](docs/RegistryListRepositoryTags200Response.md)
 - [RegistryListRepositoryTags200ResponseAllOf](docs/RegistryListRepositoryTags200ResponseAllOf.md)
 - [RegistrySubscription](docs/RegistrySubscription.md)
 - [RegistryUpdateSubscriptionRequest](docs/RegistryUpdateSubscriptionRequest.md)
 - [Repository](docs/Repository.md)
 - [RepositoryBlob](docs/RepositoryBlob.md)
 - [RepositoryManifest](docs/RepositoryManifest.md)
 - [RepositoryTag](docs/RepositoryTag.md)
 - [RepositoryV2](docs/RepositoryV2.md)
 - [ReserveToRegion](docs/ReserveToRegion.md)
 - [ReserveToRegion1](docs/ReserveToRegion1.md)
 - [ReservedIp](docs/ReservedIp.md)
 - [ReservedIpActionAssign](docs/ReservedIpActionAssign.md)
 - [ReservedIpActionAssignAllOf](docs/ReservedIpActionAssignAllOf.md)
 - [ReservedIpActionType](docs/ReservedIpActionType.md)
 - [ReservedIpActionUnassign](docs/ReservedIpActionUnassign.md)
 - [ReservedIpCreate](docs/ReservedIpCreate.md)
 - [ReservedIpDroplet](docs/ReservedIpDroplet.md)
 - [ReservedIpRegion](docs/ReservedIpRegion.md)
 - [ReservedIpsActionsList200Response](docs/ReservedIpsActionsList200Response.md)
 - [ReservedIpsActionsPost201Response](docs/ReservedIpsActionsPost201Response.md)
 - [ReservedIpsActionsPostRequest](docs/ReservedIpsActionsPostRequest.md)
 - [ReservedIpsCreate202Response](docs/ReservedIpsCreate202Response.md)
 - [ReservedIpsGet200Response](docs/ReservedIpsGet200Response.md)
 - [ReservedIpsList200Response](docs/ReservedIpsList200Response.md)
 - [ReservedIpsList200ResponseAllOf](docs/ReservedIpsList200ResponseAllOf.md)
 - [Resource](docs/Resource.md)
 - [ResourceLinks](docs/ResourceLinks.md)
 - [SelectiveDestroyAssociatedResource](docs/SelectiveDestroyAssociatedResource.md)
 - [SimpleCharge](docs/SimpleCharge.md)
 - [SingleDropletResponse](docs/SingleDropletResponse.md)
 - [SingleDropletResponseLinks](docs/SingleDropletResponseLinks.md)
 - [Size](docs/Size.md)
 - [SizesList200Response](docs/SizesList200Response.md)
 - [SizesList200ResponseAllOf](docs/SizesList200ResponseAllOf.md)
 - [SlackDetails](docs/SlackDetails.md)
 - [Snapshots](docs/Snapshots.md)
 - [SnapshotsAllOf](docs/SnapshotsAllOf.md)
 - [SnapshotsAllOf1](docs/SnapshotsAllOf1.md)
 - [SnapshotsBase](docs/SnapshotsBase.md)
 - [SnapshotsGet200Response](docs/SnapshotsGet200Response.md)
 - [SnapshotsGetSnapshotIdParameter](docs/SnapshotsGetSnapshotIdParameter.md)
 - [SnapshotsList200Response](docs/SnapshotsList200Response.md)
 - [SnapshotsList200ResponseAllOf](docs/SnapshotsList200ResponseAllOf.md)
 - [SourceDatabase](docs/SourceDatabase.md)
 - [SourceDatabaseSource](docs/SourceDatabaseSource.md)
 - [SqlMode](docs/SqlMode.md)
 - [SshKeys](docs/SshKeys.md)
 - [SshKeysCreate201Response](docs/SshKeysCreate201Response.md)
 - [SshKeysGetSshKeyIdentifierParameter](docs/SshKeysGetSshKeyIdentifierParameter.md)
 - [SshKeysList200Response](docs/SshKeysList200Response.md)
 - [SshKeysList200ResponseAllOf](docs/SshKeysList200ResponseAllOf.md)
 - [SshKeysUpdateRequest](docs/SshKeysUpdateRequest.md)
 - [State](docs/State.md)
 - [StickySessions](docs/StickySessions.md)
 - [Subscription](docs/Subscription.md)
 - [SubscriptionTierBase](docs/SubscriptionTierBase.md)
 - [SubscriptionTierExtended](docs/SubscriptionTierExtended.md)
 - [Tags](docs/Tags.md)
 - [TagsCreate201Response](docs/TagsCreate201Response.md)
 - [TagsGet200Response](docs/TagsGet200Response.md)
 - [TagsList200Response](docs/TagsList200Response.md)
 - [TagsList200ResponseAllOf](docs/TagsList200ResponseAllOf.md)
 - [TagsMetadata](docs/TagsMetadata.md)
 - [TagsResource](docs/TagsResource.md)
 - [TagsResourceResourcesInner](docs/TagsResourceResourcesInner.md)
 - [TagsResources](docs/TagsResources.md)
 - [TagsResourcesAllOf](docs/TagsResourcesAllOf.md)
 - [Timescaledb](docs/Timescaledb.md)
 - [UpdateEndpoint](docs/UpdateEndpoint.md)
 - [UpdateRegistry](docs/UpdateRegistry.md)
 - [UptimeAlertCreate201Response](docs/UptimeAlertCreate201Response.md)
 - [UptimeAlertCreateRequest](docs/UptimeAlertCreateRequest.md)
 - [UptimeAlertUpdateRequest](docs/UptimeAlertUpdateRequest.md)
 - [UptimeCheckAlertsList200Response](docs/UptimeCheckAlertsList200Response.md)
 - [UptimeCheckAlertsList200ResponseAllOf](docs/UptimeCheckAlertsList200ResponseAllOf.md)
 - [UptimeCheckCreate201Response](docs/UptimeCheckCreate201Response.md)
 - [UptimeCheckCreateRequest](docs/UptimeCheckCreateRequest.md)
 - [UptimeCheckStateGet200Response](docs/UptimeCheckStateGet200Response.md)
 - [UptimeCheckUpdateRequest](docs/UptimeCheckUpdateRequest.md)
 - [UptimeChecksList200Response](docs/UptimeChecksList200Response.md)
 - [UptimeChecksList200ResponseAllOf](docs/UptimeChecksList200ResponseAllOf.md)
 - [User](docs/User.md)
 - [UserKubernetesClusterUser](docs/UserKubernetesClusterUser.md)
 - [ValidateRegistry](docs/ValidateRegistry.md)
 - [VolumeAction](docs/VolumeAction.md)
 - [VolumeActionAllOf](docs/VolumeActionAllOf.md)
 - [VolumeActionPostAttach](docs/VolumeActionPostAttach.md)
 - [VolumeActionPostAttachAllOf](docs/VolumeActionPostAttachAllOf.md)
 - [VolumeActionPostBase](docs/VolumeActionPostBase.md)
 - [VolumeActionPostDetach](docs/VolumeActionPostDetach.md)
 - [VolumeActionPostDetachAllOf](docs/VolumeActionPostDetachAllOf.md)
 - [VolumeActionPostResize](docs/VolumeActionPostResize.md)
 - [VolumeActionPostResizeAllOf](docs/VolumeActionPostResizeAllOf.md)
 - [VolumeActionsList200Response](docs/VolumeActionsList200Response.md)
 - [VolumeActionsList200ResponseAllOf](docs/VolumeActionsList200ResponseAllOf.md)
 - [VolumeActionsPost202Response](docs/VolumeActionsPost202Response.md)
 - [VolumeActionsPostByIdRequest](docs/VolumeActionsPostByIdRequest.md)
 - [VolumeActionsPostRequest](docs/VolumeActionsPostRequest.md)
 - [VolumeBase](docs/VolumeBase.md)
 - [VolumeFull](docs/VolumeFull.md)
 - [VolumeFullAllOf](docs/VolumeFullAllOf.md)
 - [VolumeSnapshotId](docs/VolumeSnapshotId.md)
 - [VolumeSnapshotsCreateRequest](docs/VolumeSnapshotsCreateRequest.md)
 - [VolumeSnapshotsGetById200Response](docs/VolumeSnapshotsGetById200Response.md)
 - [VolumeSnapshotsList200Response](docs/VolumeSnapshotsList200Response.md)
 - [VolumeWriteFileSystemType](docs/VolumeWriteFileSystemType.md)
 - [VolumesCreate201Response](docs/VolumesCreate201Response.md)
 - [VolumesCreateRequest](docs/VolumesCreateRequest.md)
 - [VolumesExt4](docs/VolumesExt4.md)
 - [VolumesExt4AllOf](docs/VolumesExt4AllOf.md)
 - [VolumesList200Response](docs/VolumesList200Response.md)
 - [VolumesList200ResponseAllOf](docs/VolumesList200ResponseAllOf.md)
 - [VolumesXfs](docs/VolumesXfs.md)
 - [VolumesXfsAllOf](docs/VolumesXfsAllOf.md)
 - [Vpc](docs/Vpc.md)
 - [VpcBase](docs/VpcBase.md)
 - [VpcCreate](docs/VpcCreate.md)
 - [VpcDefault](docs/VpcDefault.md)
 - [VpcMember](docs/VpcMember.md)
 - [VpcUpdatable](docs/VpcUpdatable.md)
 - [VpcsCreate201Response](docs/VpcsCreate201Response.md)
 - [VpcsCreateRequest](docs/VpcsCreateRequest.md)
 - [VpcsList200Response](docs/VpcsList200Response.md)
 - [VpcsList200ResponseAllOf](docs/VpcsList200ResponseAllOf.md)
 - [VpcsListMembers200Response](docs/VpcsListMembers200Response.md)
 - [VpcsListMembers200ResponseAllOf](docs/VpcsListMembers200ResponseAllOf.md)
 - [VpcsPatchRequest](docs/VpcsPatchRequest.md)
 - [VpcsUpdateRequest](docs/VpcsUpdateRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

api-engineering@digitalocean.com

