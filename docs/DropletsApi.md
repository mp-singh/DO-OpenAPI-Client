# \DropletsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**droplets_create**](DropletsApi.md#droplets_create) | **POST** /v2/droplets | Create a New Droplet
[**droplets_destroy**](DropletsApi.md#droplets_destroy) | **DELETE** /v2/droplets/{droplet_id} | Delete an Existing Droplet
[**droplets_destroy_by_tag**](DropletsApi.md#droplets_destroy_by_tag) | **DELETE** /v2/droplets | Deleting Droplets by Tag
[**droplets_destroy_retry_with_associated_resources**](DropletsApi.md#droplets_destroy_retry_with_associated_resources) | **POST** /v2/droplets/{droplet_id}/destroy_with_associated_resources/retry | Retry a Droplet Destroy with Associated Resources Request
[**droplets_destroy_with_associated_resources_dangerous**](DropletsApi.md#droplets_destroy_with_associated_resources_dangerous) | **DELETE** /v2/droplets/{droplet_id}/destroy_with_associated_resources/dangerous | Destroy a Droplet and All of its Associated Resources (Dangerous)
[**droplets_destroy_with_associated_resources_selective**](DropletsApi.md#droplets_destroy_with_associated_resources_selective) | **DELETE** /v2/droplets/{droplet_id}/destroy_with_associated_resources/selective | Selectively Destroy a Droplet and its Associated Resources
[**droplets_get**](DropletsApi.md#droplets_get) | **GET** /v2/droplets/{droplet_id} | Retrieve an Existing Droplet
[**droplets_get_destroy_associated_resources_status**](DropletsApi.md#droplets_get_destroy_associated_resources_status) | **GET** /v2/droplets/{droplet_id}/destroy_with_associated_resources/status | Check Status of a Droplet Destroy with Associated Resources Request
[**droplets_list**](DropletsApi.md#droplets_list) | **GET** /v2/droplets | List All Droplets
[**droplets_list_associated_resources**](DropletsApi.md#droplets_list_associated_resources) | **GET** /v2/droplets/{droplet_id}/destroy_with_associated_resources | List Associated Resources for a Droplet
[**droplets_list_backups**](DropletsApi.md#droplets_list_backups) | **GET** /v2/droplets/{droplet_id}/backups | List Backups for a Droplet
[**droplets_list_firewalls**](DropletsApi.md#droplets_list_firewalls) | **GET** /v2/droplets/{droplet_id}/firewalls | List all Firewalls Applied to a Droplet
[**droplets_list_kernels**](DropletsApi.md#droplets_list_kernels) | **GET** /v2/droplets/{droplet_id}/kernels | List All Available Kernels for a Droplet
[**droplets_list_neighbors**](DropletsApi.md#droplets_list_neighbors) | **GET** /v2/droplets/{droplet_id}/neighbors | List Neighbors for a Droplet
[**droplets_list_neighbors_ids**](DropletsApi.md#droplets_list_neighbors_ids) | **GET** /v2/reports/droplet_neighbors_ids | List All Droplet Neighbors
[**droplets_list_snapshots**](DropletsApi.md#droplets_list_snapshots) | **GET** /v2/droplets/{droplet_id}/snapshots | List Snapshots for a Droplet



## droplets_create

> crate::models::DropletsCreate202Response droplets_create(droplets_create_request)
Create a New Droplet

To create a new Droplet, send a POST request to `/v2/droplets` setting the required attributes.  A Droplet will be created using the provided information. The response body will contain a JSON object with a key called `droplet`. The value will be an object containing the standard attributes for your new Droplet. The response code, 202 Accepted, does not indicate the success or failure of the operation, just that the request has been accepted for processing. The `actions` returned as part of the response's `links` object can be used to check the status of the Droplet create event.  ### Create Multiple Droplets  Creating multiple Droplets is very similar to creating a single Droplet. Instead of sending `name` as a string, send `names` as an array of strings. A Droplet will be created for each name you send using the associated information. Up to ten Droplets may be created this way at a time.  Rather than returning a single Droplet, the response body will contain a JSON array with a key called `droplets`. This will be set to an array of JSON objects, each of which will contain the standard Droplet attributes. The response code, 202 Accepted, does not indicate the success or failure of any operation, just that the request has been accepted for processing. The array of `actions` returned as part of the response's `links` object can be used to check the status of each individual Droplet create event. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplets_create_request** | Option<[**DropletsCreateRequest**](DropletsCreateRequest.md)> |  |  |

### Return type

[**crate::models::DropletsCreate202Response**](droplets_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_destroy

> droplets_destroy(droplet_id)
Delete an Existing Droplet

To delete a Droplet, send a DELETE request to `/v2/droplets/$DROPLET_ID`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_destroy_by_tag

> droplets_destroy_by_tag(tag_name)
Deleting Droplets by Tag

To delete **all** Droplets assigned to a specific tag, include the `tag_name` query parameter set to the name of the tag in your DELETE request. For example,  `/v2/droplets?tag_name=$TAG_NAME`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_name** | **String** | Specifies Droplets to be deleted by tag. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_destroy_retry_with_associated_resources

> droplets_destroy_retry_with_associated_resources(droplet_id)
Retry a Droplet Destroy with Associated Resources Request

If the status of a request to destroy a Droplet with its associated resources reported any errors, it can be retried by sending a POST request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources/retry` endpoint.  Only one destroy can be active at a time per Droplet. If a retry is issued while another destroy is in progress for the Droplet a 409 status code will be returned. A successful response will include a 202 response code and no content. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_destroy_with_associated_resources_dangerous

> droplets_destroy_with_associated_resources_dangerous(droplet_id, x_dangerous)
Destroy a Droplet and All of its Associated Resources (Dangerous)

To destroy a Droplet along with all of its associated resources, send a DELETE request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources/dangerous` endpoint. The headers of this request must include an `X-Dangerous` key set to `true`. To preview which resources will be destroyed, first query the Droplet's associated resources. This operation _can not_ be reverse and should be used with caution.  A successful response will include a 202 response code and no content. Use the status endpoint to check on the success or failure of the destruction of the individual resources. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
**x_dangerous** | **bool** | Acknowledge this action will destroy the Droplet and all associated resources and _can not_ be reversed. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_destroy_with_associated_resources_selective

> droplets_destroy_with_associated_resources_selective(droplet_id, selective_destroy_associated_resource)
Selectively Destroy a Droplet and its Associated Resources

To destroy a Droplet along with a sub-set of its associated resources, send a DELETE request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources/selective` endpoint. The JSON body of the request should include `reserved_ips`, `snapshots`, `volumes`, or `volume_snapshots` keys each set to an array of IDs for the associated resources to be destroyed. The IDs can be found by querying the Droplet's associated resources. Any associated resource not included in the request will remain and continue to accrue changes on your account.  A successful response will include a 202 response code and no content. Use the status endpoint to check on the success or failure of the destruction of the individual resources. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
**selective_destroy_associated_resource** | Option<[**SelectiveDestroyAssociatedResource**](SelectiveDestroyAssociatedResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_get

> crate::models::DropletsGet200Response droplets_get(droplet_id)
Retrieve an Existing Droplet

To show information about an individual Droplet, send a GET request to `/v2/droplets/$DROPLET_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |

### Return type

[**crate::models::DropletsGet200Response**](droplets_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_get_destroy_associated_resources_status

> crate::models::AssociatedResourceStatus droplets_get_destroy_associated_resources_status(droplet_id)
Check Status of a Droplet Destroy with Associated Resources Request

To check on the status of a request to destroy a Droplet with its associated resources, send a GET request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources/status` endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |

### Return type

[**crate::models::AssociatedResourceStatus**](associated_resource_status.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_list

> crate::models::DropletsList200Response droplets_list(per_page, page, tag_name, name)
List All Droplets

To list all Droplets in your account, send a GET request to `/v2/droplets`.  The response body will be a JSON object with a key of `droplets`. This will be set to an array containing objects each representing a Droplet. These will contain the standard Droplet attributes.  ### Filtering Results by Tag  It's possible to request filtered results by including certain query parameters. To only list Droplets assigned to a specific tag, include the `tag_name` query parameter set to the name of the tag in your GET request. For example, `/v2/droplets?tag_name=$TAG_NAME`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**tag_name** | Option<**String**> | Used to filter Droplets by a specific tag. Can not be combined with `name`. |  |
**name** | Option<**String**> | Used to filter list response by Droplet name returning only exact matches. It is case-insensitive and can not be combined with `tag_name`. |  |

### Return type

[**crate::models::DropletsList200Response**](droplets_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_list_associated_resources

> crate::models::DropletsListAssociatedResources200Response droplets_list_associated_resources(droplet_id)
List Associated Resources for a Droplet

To list the associated billable resources that can be destroyed along with a Droplet, send a GET request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources` endpoint.  The response will be a JSON object containing `snapshots`, `volumes`, and `volume_snapshots` keys. Each will be set to an array of objects containing information about the associated resources. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |

### Return type

[**crate::models::DropletsListAssociatedResources200Response**](droplets_list_associatedResources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_list_backups

> crate::models::DropletsListBackups200Response droplets_list_backups(droplet_id, per_page, page)
List Backups for a Droplet

To retrieve any backups associated with a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/backups`.  You will get back a JSON object that has a `backups` key. This will be set to an array of backup objects, each of which contain the standard Droplet backup attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::DropletsListBackups200Response**](droplets_list_backups_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_list_firewalls

> crate::models::DropletsListFirewalls200Response droplets_list_firewalls(droplet_id, per_page, page)
List all Firewalls Applied to a Droplet

To retrieve a list of all firewalls available to a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/firewalls`  The response will be a JSON object that has a key called `firewalls`. This will be set to an array of `firewall` objects, each of which contain the standard `firewall` attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::DropletsListFirewalls200Response**](droplets_list_firewalls_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_list_kernels

> crate::models::DropletsListKernels200Response droplets_list_kernels(droplet_id, per_page, page)
List All Available Kernels for a Droplet

To retrieve a list of all kernels available to a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/kernels`  The response will be a JSON object that has a key called `kernels`. This will be set to an array of `kernel` objects, each of which contain the standard `kernel` attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::DropletsListKernels200Response**](droplets_list_kernels_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_list_neighbors

> crate::models::DropletsListNeighbors200Response droplets_list_neighbors(droplet_id)
List Neighbors for a Droplet

To retrieve a list of any \"neighbors\" (i.e. Droplets that are co-located on the same physical hardware) for a specific Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/neighbors`.  The results will be returned as a JSON object with a key of `droplets`. This will be set to an array containing objects representing any other Droplets that share the same physical hardware. An empty array indicates that the Droplet is not co-located any other Droplets associated with your account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |

### Return type

[**crate::models::DropletsListNeighbors200Response**](droplets_list_neighbors_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_list_neighbors_ids

> crate::models::NeighborIds droplets_list_neighbors_ids()
List All Droplet Neighbors

To retrieve a list of all Droplets that are co-located on the same physical hardware, send a GET request to `/v2/reports/droplet_neighbors_ids`.  The results will be returned as a JSON object with a key of `neighbor_ids`. This will be set to an array of arrays. Each array will contain a set of Droplet IDs for Droplets that share a physical server. An empty array indicates that all Droplets associated with your account are located on separate physical hardware. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::NeighborIds**](neighbor_ids.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplets_list_snapshots

> crate::models::DropletsListSnapshots200Response droplets_list_snapshots(droplet_id, per_page, page)
List Snapshots for a Droplet

To retrieve the snapshots that have been created from a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/snapshots`.  You will get back a JSON object that has a `snapshots` key. This will be set to an array of snapshot objects, each of which contain the standard Droplet snapshot attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::DropletsListSnapshots200Response**](droplets_list_snapshots_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

