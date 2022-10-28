# \AppsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_assign_alert_destinations**](AppsApi.md#apps_assign_alert_destinations) | **POST** /v2/apps/{app_id}/alerts/{alert_id}/destinations | Update destinations for alerts
[**apps_cancel_deployment**](AppsApi.md#apps_cancel_deployment) | **POST** /v2/apps/{app_id}/deployments/{deployment_id}/cancel | Cancel a Deployment
[**apps_commit_rollback**](AppsApi.md#apps_commit_rollback) | **POST** /v2/apps/{app_id}/rollback/commit | Commit App Rollback
[**apps_create**](AppsApi.md#apps_create) | **POST** /v2/apps | Create a New App
[**apps_create_deployment**](AppsApi.md#apps_create_deployment) | **POST** /v2/apps/{app_id}/deployments | Create an App Deployment
[**apps_create_rollback**](AppsApi.md#apps_create_rollback) | **POST** /v2/apps/{app_id}/rollback | Rollback App
[**apps_delete**](AppsApi.md#apps_delete) | **DELETE** /v2/apps/{id} | Delete an App
[**apps_get**](AppsApi.md#apps_get) | **GET** /v2/apps/{id} | Retrieve an Existing App
[**apps_get_deployment**](AppsApi.md#apps_get_deployment) | **GET** /v2/apps/{app_id}/deployments/{deployment_id} | Retrieve an App Deployment
[**apps_get_instance_size**](AppsApi.md#apps_get_instance_size) | **GET** /v2/apps/tiers/instance_sizes/{slug} | Retrieve an Instance Size
[**apps_get_logs**](AppsApi.md#apps_get_logs) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/components/{component_name}/logs | Retrieve Deployment Logs
[**apps_get_logs_aggregate**](AppsApi.md#apps_get_logs_aggregate) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/logs | Retrieve Aggregate Deployment Logs
[**apps_get_tier**](AppsApi.md#apps_get_tier) | **GET** /v2/apps/tiers/{slug} | Retrieve an App Tier
[**apps_list**](AppsApi.md#apps_list) | **GET** /v2/apps | List All Apps
[**apps_list_alerts**](AppsApi.md#apps_list_alerts) | **GET** /v2/apps/{app_id}/alerts | List all app alerts
[**apps_list_deployments**](AppsApi.md#apps_list_deployments) | **GET** /v2/apps/{app_id}/deployments | List App Deployments
[**apps_list_instance_sizes**](AppsApi.md#apps_list_instance_sizes) | **GET** /v2/apps/tiers/instance_sizes | List Instance Sizes
[**apps_list_regions**](AppsApi.md#apps_list_regions) | **GET** /v2/apps/regions | List App Regions
[**apps_list_tiers**](AppsApi.md#apps_list_tiers) | **GET** /v2/apps/tiers | List App Tiers
[**apps_revert_rollback**](AppsApi.md#apps_revert_rollback) | **POST** /v2/apps/{app_id}/rollback/revert | Revert App Rollback
[**apps_update**](AppsApi.md#apps_update) | **PUT** /v2/apps/{id} | Update an App
[**apps_validate_app_spec**](AppsApi.md#apps_validate_app_spec) | **POST** /v2/apps/propose | Propose an App Spec
[**apps_validate_rollback**](AppsApi.md#apps_validate_rollback) | **POST** /v2/apps/{app_id}/rollback/validate | Validate App Rollback



## apps_assign_alert_destinations

> crate::models::AppsAlertResponse apps_assign_alert_destinations(app_id, alert_id, apps_assign_app_alert_destinations_request)
Update destinations for alerts

Updates the emails and slack webhook destinations for app alerts. Emails must be associated to a user with access to the app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**alert_id** | **String** | The alert ID | [required] |
**apps_assign_app_alert_destinations_request** | [**AppsAssignAppAlertDestinationsRequest**](AppsAssignAppAlertDestinationsRequest.md) |  | [required] |

### Return type

[**crate::models::AppsAlertResponse**](apps_alert_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_cancel_deployment

> crate::models::AppsDeploymentResponse apps_cancel_deployment(app_id, deployment_id)
Cancel a Deployment

Immediately cancel an in-progress deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |

### Return type

[**crate::models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_commit_rollback

> apps_commit_rollback(app_id)
Commit App Rollback

Commit an app rollback. This action permanently applies the rollback and unpins the app to resume new deployments. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_create

> crate::models::AppResponse apps_create(apps_create_app_request, accept, content_type)
Create a New App

Create a new app by submitting an app specification. For documentation on app specifications (`AppSpec` objects), please refer to [the product documentation](https://docs.digitalocean.com/products/app-platform/reference/app-spec/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apps_create_app_request** | [**AppsCreateAppRequest**](AppsCreateAppRequest.md) |  | [required] |
**accept** | Option<**String**> | The content-type that should be used by the response. By default, the response will be `application/json`. `application/yaml` is also supported. |  |
**content_type** | Option<**String**> | The content-type used for the request. By default, the requests are assumed to use `application/json`. `application/yaml` is also supported. |  |

### Return type

[**crate::models::AppResponse**](app_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/yaml
- **Accept**: application/json, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_create_deployment

> crate::models::AppsDeploymentResponse apps_create_deployment(app_id, apps_create_deployment_request)
Create an App Deployment

Creating an app deployment will pull the latest changes from your repository and schedule a new deployment for your app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**apps_create_deployment_request** | [**AppsCreateDeploymentRequest**](AppsCreateDeploymentRequest.md) |  | [required] |

### Return type

[**crate::models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_create_rollback

> crate::models::AppsDeploymentResponse apps_create_rollback(app_id, apps_rollback_app_request)
Rollback App

Rollback an app to a previous deployment. A new deployment will be created to perform the rollback. The app will be pinned to the rollback deployment preventing any new deployments from being created, either manually or through Auto Deploy on Push webhooks. To resume deployments, the rollback must be either committed or reverted.  It is recommended to use the Validate App Rollback endpoint to double check if the rollback is valid and if there are any warnings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**apps_rollback_app_request** | [**AppsRollbackAppRequest**](AppsRollbackAppRequest.md) |  | [required] |

### Return type

[**crate::models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_delete

> crate::models::AppsDeleteAppResponse apps_delete(id)
Delete an App

Delete an existing app. Once deleted, all active deployments will be permanently shut down and the app deleted. If needed, be sure to back up your app specification so that you may re-create it at a later time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the app | [required] |

### Return type

[**crate::models::AppsDeleteAppResponse**](apps_delete_app_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get

> crate::models::AppResponse apps_get(id, name)
Retrieve an Existing App

Retrieve details about an existing app by either its ID or name. To retrieve an app by its name, do not include an ID in the request path. Information about the current active deployment as well as any in progress ones will also be included in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the app | [required] |
**name** | Option<**String**> | The name of the app to retrieve. |  |

### Return type

[**crate::models::AppResponse**](app_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_deployment

> crate::models::AppsDeploymentResponse apps_get_deployment(app_id, deployment_id)
Retrieve an App Deployment

Retrieve information about an app deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |

### Return type

[**crate::models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_instance_size

> crate::models::AppsGetInstanceSizeResponse apps_get_instance_size(slug)
Retrieve an Instance Size

Retrieve information about a specific instance size for `service`, `worker`, and `job` components.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The slug of the instance size | [required] |

### Return type

[**crate::models::AppsGetInstanceSizeResponse**](apps_get_instance_size_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_logs

> crate::models::AppsGetLogsResponse apps_get_logs(app_id, deployment_id, component_name, r#type, follow, pod_connection_timeout)
Retrieve Deployment Logs

Retrieve the logs of a past, in-progress, or active deployment. If a component name is specified, the logs will be limited to only that component. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |
**component_name** | **String** | An optional component name. If set, logs will be limited to this component only. | [required] |
**r#type** | **String** | The type of logs to retrieve - BUILD: Build-time logs - DEPLOY: Deploy-time logs - RUN: Live run-time logs | [required] |[default to UNSPECIFIED]
**follow** | Option<**bool**> | Whether the logs should follow live updates. |  |
**pod_connection_timeout** | Option<**String**> | An optional time duration to wait if the underlying component instance is not immediately available. Default: `3m`. |  |

### Return type

[**crate::models::AppsGetLogsResponse**](apps_get_logs_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_logs_aggregate

> crate::models::AppsGetLogsResponse apps_get_logs_aggregate(app_id, deployment_id, r#type, follow, pod_connection_timeout)
Retrieve Aggregate Deployment Logs

Retrieve the logs of a past, in-progress, or active deployment. If a component name is specified, the logs will be limited to only that component. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |
**r#type** | **String** | The type of logs to retrieve - BUILD: Build-time logs - DEPLOY: Deploy-time logs - RUN: Live run-time logs | [required] |[default to UNSPECIFIED]
**follow** | Option<**bool**> | Whether the logs should follow live updates. |  |
**pod_connection_timeout** | Option<**String**> | An optional time duration to wait if the underlying component instance is not immediately available. Default: `3m`. |  |

### Return type

[**crate::models::AppsGetLogsResponse**](apps_get_logs_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_tier

> crate::models::AppsGetTierResponse apps_get_tier(slug)
Retrieve an App Tier

Retrieve information about a specific app tier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The slug of the tier | [required] |

### Return type

[**crate::models::AppsGetTierResponse**](apps_get_tier_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list

> crate::models::AppsResponse apps_list(page, per_page)
List All Apps

List all apps on your account. Information about the current active deployment as well as any in progress ones will also be included for each app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]

### Return type

[**crate::models::AppsResponse**](apps_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_alerts

> crate::models::AppsListAlertsResponse apps_list_alerts(app_id)
List all app alerts

List alerts associated to the app and any components. This includes configuration information about the alerts including emails, slack webhooks, and triggering events or conditions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

[**crate::models::AppsListAlertsResponse**](apps_list_alerts_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_deployments

> crate::models::AppsDeploymentsResponse apps_list_deployments(app_id, page, per_page)
List App Deployments

List all deployments of an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]

### Return type

[**crate::models::AppsDeploymentsResponse**](apps_deployments_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_instance_sizes

> crate::models::AppsListInstanceSizesResponse apps_list_instance_sizes()
List Instance Sizes

List all instance sizes for `service`, `worker`, and `job` components.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AppsListInstanceSizesResponse**](apps_list_instance_sizes_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_regions

> crate::models::AppsListRegionsResponse apps_list_regions()
List App Regions

List all regions supported by App Platform.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AppsListRegionsResponse**](apps_list_regions_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_tiers

> crate::models::AppsListTiersResponse apps_list_tiers()
List App Tiers

List all app tiers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AppsListTiersResponse**](apps_list_tiers_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_revert_rollback

> crate::models::AppsDeploymentResponse apps_revert_rollback(app_id)
Revert App Rollback

Revert an app rollback. This action reverts the active rollback by creating a new deployment from the latest app spec prior to the rollback and unpins the app to resume new deployments. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

[**crate::models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_update

> crate::models::AppResponse apps_update(id, apps_update_app_request)
Update an App

Update an existing app by submitting a new app specification. For documentation on app specifications (`AppSpec` objects), please refer to [the product documentation](https://docs.digitalocean.com/products/app-platform/reference/app-spec/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the app | [required] |
**apps_update_app_request** | [**AppsUpdateAppRequest**](AppsUpdateAppRequest.md) |  | [required] |

### Return type

[**crate::models::AppResponse**](app_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_validate_app_spec

> crate::models::AppProposeResponse apps_validate_app_spec(app_propose)
Propose an App Spec

To propose and validate a spec for a new or existing app, send a POST request to the `/v2/apps/propose` endpoint. The request returns some information about the proposed app, including app cost and upgrade cost. If an existing app ID is specified, the app spec is treated as a proposed update to the existing app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_propose** | [**AppPropose**](AppPropose.md) |  | [required] |

### Return type

[**crate::models::AppProposeResponse**](app_propose_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_validate_rollback

> crate::models::AppsValidateRollback200Response apps_validate_rollback(app_id, apps_rollback_app_request)
Validate App Rollback

Check whether an app can be rolled back to a specific deployment. This endpoint can also be used to check if there are any warnings or validation conditions that will cause the rollback to proceed under unideal circumstances. For example, if a component must be rebuilt as part of the rollback causing it to take longer than usual. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**apps_rollback_app_request** | [**AppsRollbackAppRequest**](AppsRollbackAppRequest.md) |  | [required] |

### Return type

[**crate::models::AppsValidateRollback200Response**](apps_validate_rollback_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

