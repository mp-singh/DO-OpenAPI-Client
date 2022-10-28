# \ReservedIpsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reserved_ips_create**](ReservedIpsApi.md#reserved_ips_create) | **POST** /v2/reserved_ips | Create a New Reserved IP
[**reserved_ips_delete**](ReservedIpsApi.md#reserved_ips_delete) | **DELETE** /v2/reserved_ips/{reserved_ip} | Delete a Reserved IPs
[**reserved_ips_get**](ReservedIpsApi.md#reserved_ips_get) | **GET** /v2/reserved_ips/{reserved_ip} | Retrieve an Existing Reserved IP
[**reserved_ips_list**](ReservedIpsApi.md#reserved_ips_list) | **GET** /v2/reserved_ips | List All Reserved IPs



## reserved_ips_create

> crate::models::ReservedIpsCreate202Response reserved_ips_create(reserved_ip_create)
Create a New Reserved IP

On creation, a reserved IP must be either assigned to a Droplet or reserved to a region. * To create a new reserved IP assigned to a Droplet, send a POST   request to `/v2/reserved_ips` with the `droplet_id` attribute.  * To create a new reserved IP reserved to a region, send a POST request to   `/v2/reserved_ips` with the `region` attribute.  **Note**:  In addition to the standard rate limiting, only 12 reserved IPs may be created per 60 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip_create** | [**ReservedIpCreate**](ReservedIpCreate.md) |  | [required] |

### Return type

[**crate::models::ReservedIpsCreate202Response**](reservedIPs_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ips_delete

> reserved_ips_delete(reserved_ip)
Delete a Reserved IPs

To delete a reserved IP and remove it from your account, send a DELETE request to `/v2/reserved_ips/$RESERVED_IP_ADDR`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip** | **String** | A reserved IP address. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ips_get

> crate::models::ReservedIpsGet200Response reserved_ips_get(reserved_ip)
Retrieve an Existing Reserved IP

To show information about a reserved IP, send a GET request to `/v2/reserved_ips/$RESERVED_IP_ADDR`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip** | **String** | A reserved IP address. | [required] |

### Return type

[**crate::models::ReservedIpsGet200Response**](reservedIPs_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ips_list

> crate::models::ReservedIpsList200Response reserved_ips_list()
List All Reserved IPs

To list all of the reserved IPs available on your account, send a GET request to `/v2/reserved_ips`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ReservedIpsList200Response**](reservedIPs_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

