# \FloatingIpsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**floating_ips_create**](FloatingIpsApi.md#floating_ips_create) | **POST** /v2/floating_ips | Create a New Floating IP
[**floating_ips_delete**](FloatingIpsApi.md#floating_ips_delete) | **DELETE** /v2/floating_ips/{floating_ip} | Delete a Floating IPs
[**floating_ips_get**](FloatingIpsApi.md#floating_ips_get) | **GET** /v2/floating_ips/{floating_ip} | Retrieve an Existing Floating IP
[**floating_ips_list**](FloatingIpsApi.md#floating_ips_list) | **GET** /v2/floating_ips | List All Floating IPs



## floating_ips_create

> crate::models::FloatingIpsCreate202Response floating_ips_create(floating_ip_create)
Create a New Floating IP

On creation, a floating IP must be either assigned to a Droplet or reserved to a region. * To create a new floating IP assigned to a Droplet, send a POST   request to `/v2/floating_ips` with the `droplet_id` attribute.  * To create a new floating IP reserved to a region, send a POST request to   `/v2/floating_ips` with the `region` attribute.  **Note**:  In addition to the standard rate limiting, only 12 floating IPs may be created per 60 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ip_create** | [**FloatingIpCreate**](FloatingIpCreate.md) |  | [required] |

### Return type

[**crate::models::FloatingIpsCreate202Response**](floatingIPs_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## floating_ips_delete

> floating_ips_delete(floating_ip)
Delete a Floating IPs

To delete a floating IP and remove it from your account, send a DELETE request to `/v2/floating_ips/$FLOATING_IP_ADDR`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ip** | **String** | A floating IP address. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## floating_ips_get

> crate::models::FloatingIpsGet200Response floating_ips_get(floating_ip)
Retrieve an Existing Floating IP

To show information about a floating IP, send a GET request to `/v2/floating_ips/$FLOATING_IP_ADDR`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ip** | **String** | A floating IP address. | [required] |

### Return type

[**crate::models::FloatingIpsGet200Response**](floatingIPs_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## floating_ips_list

> crate::models::FloatingIpsList200Response floating_ips_list()
List All Floating IPs

To list all of the floating IPs available on your account, send a GET request to `/v2/floating_ips`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FloatingIpsList200Response**](floatingIPs_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

