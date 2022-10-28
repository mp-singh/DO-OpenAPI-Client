# \ProjectResourcesApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projects_assign_resources**](ProjectResourcesApi.md#projects_assign_resources) | **POST** /v2/projects/{project_id}/resources | Assign Resources to a Project
[**projects_assign_resources_default**](ProjectResourcesApi.md#projects_assign_resources_default) | **POST** /v2/projects/default/resources | Assign Resources to Default Project
[**projects_list_resources**](ProjectResourcesApi.md#projects_list_resources) | **GET** /v2/projects/{project_id}/resources | List Project Resources
[**projects_list_resources_default**](ProjectResourcesApi.md#projects_list_resources_default) | **GET** /v2/projects/default/resources | List Default Project Resources



## projects_assign_resources

> crate::models::ProjectsAssignResources200Response projects_assign_resources(project_id, project_assignment)
Assign Resources to a Project

To assign resources to a project, send a POST request to `/v2/projects/$PROJECT_ID/resources`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | A unique identifier for a project. | [required] |
**project_assignment** | [**ProjectAssignment**](ProjectAssignment.md) |  | [required] |

### Return type

[**crate::models::ProjectsAssignResources200Response**](projects_assign_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_assign_resources_default

> crate::models::ProjectsAssignResources200Response projects_assign_resources_default(project_assignment)
Assign Resources to Default Project

To assign resources to your default project, send a POST request to `/v2/projects/default/resources`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_assignment** | [**ProjectAssignment**](ProjectAssignment.md) |  | [required] |

### Return type

[**crate::models::ProjectsAssignResources200Response**](projects_assign_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list_resources

> crate::models::ProjectsListResources200Response projects_list_resources(project_id)
List Project Resources

To list all your resources in a project, send a GET request to `/v2/projects/$PROJECT_ID/resources`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | A unique identifier for a project. | [required] |

### Return type

[**crate::models::ProjectsListResources200Response**](projects_list_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list_resources_default

> crate::models::ProjectsListResources200Response projects_list_resources_default()
List Default Project Resources

To list all your resources in your default project, send a GET request to `/v2/projects/default/resources`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ProjectsListResources200Response**](projects_list_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

