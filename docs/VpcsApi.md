# \VpcsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**vpcs_create**](VpcsApi.md#vpcs_create) | **POST** /v2/vpcs | Create a New VPC
[**vpcs_delete**](VpcsApi.md#vpcs_delete) | **DELETE** /v2/vpcs/{vpc_id} | Delete a VPC
[**vpcs_get**](VpcsApi.md#vpcs_get) | **GET** /v2/vpcs/{vpc_id} | Retrieve an Existing VPC
[**vpcs_list**](VpcsApi.md#vpcs_list) | **GET** /v2/vpcs | List All VPCs
[**vpcs_list_members**](VpcsApi.md#vpcs_list_members) | **GET** /v2/vpcs/{vpc_id}/members | List the Member Resources of a VPC
[**vpcs_patch**](VpcsApi.md#vpcs_patch) | **PATCH** /v2/vpcs/{vpc_id} | Partially Update a VPC
[**vpcs_update**](VpcsApi.md#vpcs_update) | **PUT** /v2/vpcs/{vpc_id} | Update a VPC



## vpcs_create

> crate::models::VpcsCreate201Response vpcs_create(vpcs_create_request)
Create a New VPC

To create a VPC, send a POST request to `/v2/vpcs` specifying the attributes in the table below in the JSON body.  **Note:** If you do not currently have a VPC network in a specific datacenter region, the first one that you create will be set as the default for that region. The default VPC for a region cannot be changed or deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpcs_create_request** | [**VpcsCreateRequest**](VpcsCreateRequest.md) |  | [required] |

### Return type

[**crate::models::VpcsCreate201Response**](vpcs_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcs_delete

> vpcs_delete(vpc_id)
Delete a VPC

To delete a VPC, send a DELETE request to `/v2/vpcs/$VPC_ID`. A 204 status code with no body will be returned in response to a successful request.  The default VPC for a region can not be deleted. Additionally, a VPC can only be deleted if it does not contain any member resources. Attempting to delete a region's default VPC or a VPC that still has members will result in a 403 Forbidden error response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** | A unique identifier for a VPC. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcs_get

> crate::models::VpcsCreate201Response vpcs_get(vpc_id)
Retrieve an Existing VPC

To show information about an existing VPC, send a GET request to `/v2/vpcs/$VPC_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** | A unique identifier for a VPC. | [required] |

### Return type

[**crate::models::VpcsCreate201Response**](vpcs_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcs_list

> crate::models::VpcsList200Response vpcs_list(per_page, page)
List All VPCs

To list all of the VPCs on your account, send a GET request to `/v2/vpcs`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::VpcsList200Response**](vpcs_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcs_list_members

> crate::models::VpcsListMembers200Response vpcs_list_members(vpc_id, resource_type, per_page, page)
List the Member Resources of a VPC

To list all of the resources that are members of a VPC, send a GET request to `/v2/vpcs/$VPC_ID/members`.  To only list resources of a specific type that are members of the VPC, included a `resource_type` query parameter. For example, to only list Droplets in the VPC, send a GET request to `/v2/vpcs/$VPC_ID/members?resource_type=droplet`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** | A unique identifier for a VPC. | [required] |
**resource_type** | Option<**String**> | Used to filter VPC members by a resource type. |  |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::VpcsListMembers200Response**](vpcs_list_members_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcs_patch

> crate::models::VpcsCreate201Response vpcs_patch(vpc_id, vpcs_patch_request)
Partially Update a VPC

To update a subset of information about a VPC, send a PATCH request to `/v2/vpcs/$VPC_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** | A unique identifier for a VPC. | [required] |
**vpcs_patch_request** | [**VpcsPatchRequest**](VpcsPatchRequest.md) |  | [required] |

### Return type

[**crate::models::VpcsCreate201Response**](vpcs_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcs_update

> crate::models::VpcsCreate201Response vpcs_update(vpc_id, vpcs_update_request)
Update a VPC

To update information about a VPC, send a PUT request to `/v2/vpcs/$VPC_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** | A unique identifier for a VPC. | [required] |
**vpcs_update_request** | [**VpcsUpdateRequest**](VpcsUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::VpcsCreate201Response**](vpcs_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

