# \FunctionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**functions_create_namespace**](FunctionsApi.md#functions_create_namespace) | **POST** /v2/functions/namespaces | Create Namespace
[**functions_delete_namespace**](FunctionsApi.md#functions_delete_namespace) | **DELETE** /v2/functions/namespaces/{namespace_id} | Delete Namespace
[**functions_get_namespace**](FunctionsApi.md#functions_get_namespace) | **GET** /v2/functions/namespaces/{namespace_id} | Get Namespace
[**functions_list_namespaces**](FunctionsApi.md#functions_list_namespaces) | **GET** /v2/functions/namespaces | List Namespaces



## functions_create_namespace

> crate::models::FunctionsCreateNamespace200Response functions_create_namespace(create_namespace)
Create Namespace

Creates a new serverless functions namespace in the desired region and associates it with the provided label. A namespace is a collection of functions and their associated packages, triggers, and project specifications. To create a namespace, send a POST request to `/v2/functions/namespaces` with the `region` and `label` properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_namespace** | [**CreateNamespace**](CreateNamespace.md) |  | [required] |

### Return type

[**crate::models::FunctionsCreateNamespace200Response**](functions_create_namespace_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_delete_namespace

> functions_delete_namespace(namespace_id)
Delete Namespace

Deletes the given namespace.  When a namespace is deleted all assets, in the namespace are deleted, this includes packages, functions and triggers. Deleting a namespace is a destructive operation and assets in the namespace are not recoverable after deletion. Some metadata is retained, such as activations, or soft deleted for reporting purposes. To delete namespace, send a DELETE request to `/v2/functions/namespaces/$NAMESPACE_ID`. A successful deletion returns a 204 response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_get_namespace

> crate::models::FunctionsCreateNamespace200Response functions_get_namespace(namespace_id)
Get Namespace

Gets the namespace details for the given namespace UUID. To get namespace details, send a GET request to `/v2/functions/namespaces/$NAMESPACE_ID` with no parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |

### Return type

[**crate::models::FunctionsCreateNamespace200Response**](functions_create_namespace_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_list_namespaces

> crate::models::FunctionsListNamespaces200Response functions_list_namespaces()
List Namespaces

Returns a list of namespaces associated with the current user. To get all namespaces, send a GET request to `/v2/functions/namespaces`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FunctionsListNamespaces200Response**](functions_list_namespaces_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

