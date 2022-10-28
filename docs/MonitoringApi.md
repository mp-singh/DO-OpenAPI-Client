# \MonitoringApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**monitoring_create_alert_policy**](MonitoringApi.md#monitoring_create_alert_policy) | **POST** /v2/monitoring/alerts | Create Alert Policy
[**monitoring_delete_alert_policy**](MonitoringApi.md#monitoring_delete_alert_policy) | **DELETE** /v2/monitoring/alerts/{alert_uuid} | Delete an Alert Policy
[**monitoring_get_alert_policy**](MonitoringApi.md#monitoring_get_alert_policy) | **GET** /v2/monitoring/alerts/{alert_uuid} | Retrieve an Existing Alert Policy
[**monitoring_get_droplet_bandwidth_metrics**](MonitoringApi.md#monitoring_get_droplet_bandwidth_metrics) | **GET** /v2/monitoring/metrics/droplet/bandwidth | Get Droplet Bandwidth Metrics
[**monitoring_get_droplet_cpu_metrics**](MonitoringApi.md#monitoring_get_droplet_cpu_metrics) | **GET** /v2/monitoring/metrics/droplet/cpu | Get Droplet CPU Metrics
[**monitoring_get_droplet_filesystem_free_metrics**](MonitoringApi.md#monitoring_get_droplet_filesystem_free_metrics) | **GET** /v2/monitoring/metrics/droplet/filesystem_free | Get Droplet Filesystem Free Metrics
[**monitoring_get_droplet_filesystem_size_metrics**](MonitoringApi.md#monitoring_get_droplet_filesystem_size_metrics) | **GET** /v2/monitoring/metrics/droplet/filesystem_size | Get Droplet Filesystem Size Metrics
[**monitoring_get_droplet_load15_metrics**](MonitoringApi.md#monitoring_get_droplet_load15_metrics) | **GET** /v2/monitoring/metrics/droplet/load_15 | Get Droplet Load15 Metrics
[**monitoring_get_droplet_load1_metrics**](MonitoringApi.md#monitoring_get_droplet_load1_metrics) | **GET** /v2/monitoring/metrics/droplet/load_1 | Get Droplet Load1 Metrics
[**monitoring_get_droplet_load5_metrics**](MonitoringApi.md#monitoring_get_droplet_load5_metrics) | **GET** /v2/monitoring/metrics/droplet/load_5 | Get Droplet Load5 Metrics
[**monitoring_get_droplet_memory_available_metrics**](MonitoringApi.md#monitoring_get_droplet_memory_available_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_available | Get Droplet Available Memory Metrics
[**monitoring_get_droplet_memory_cached_metrics**](MonitoringApi.md#monitoring_get_droplet_memory_cached_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_cached | Get Droplet Cached Memory Metrics
[**monitoring_get_droplet_memory_free_metrics**](MonitoringApi.md#monitoring_get_droplet_memory_free_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_free | Get Droplet Free Memory Metrics
[**monitoring_get_droplet_memory_total_metrics**](MonitoringApi.md#monitoring_get_droplet_memory_total_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_total | Get Droplet Total Memory Metrics
[**monitoring_list_alert_policy**](MonitoringApi.md#monitoring_list_alert_policy) | **GET** /v2/monitoring/alerts | List Alert Policies
[**monitoring_update_alert_policy**](MonitoringApi.md#monitoring_update_alert_policy) | **PUT** /v2/monitoring/alerts/{alert_uuid} | Update an Alert Policy



## monitoring_create_alert_policy

> crate::models::MonitoringCreateAlertPolicy200Response monitoring_create_alert_policy(alert_policy_request)
Create Alert Policy

To create a new alert, send a POST request to `/v2/monitoring/alerts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_policy_request** | [**AlertPolicyRequest**](AlertPolicyRequest.md) | The `type` field dictates what type of entity that the alert policy applies to and hence what type of entity is passed in the `entities` array. If both the `tags` array and `entities` array are empty the alert policy applies to all entities of the relevant type that are owned by the user account. Otherwise the following table shows the valid entity types for each type of alert policy:  Type | Description | Valid Entity Type -----|-------------|-------------------- `v1/insights/droplet/memory_utilization_percent` | alert on the percent of memory utilization | Droplet ID `v1/insights/droplet/disk_read` | alert on the rate of disk read I/O in MBps | Droplet ID `v1/insights/droplet/load_5` | alert on the 5 minute load average | Droplet ID `v1/insights/droplet/load_15` | alert on the 15 minute load average | Droplet ID `v1/insights/droplet/disk_utilization_percent` | alert on the percent of disk utilization | Droplet ID `v1/insights/droplet/cpu` | alert on the percent of CPU utilization | Droplet ID `v1/insights/droplet/disk_write` | alert on the rate of disk write I/O in MBps | Droplet ID `v1/insights/droplet/public_outbound_bandwidth` | alert on the rate of public outbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/public_inbound_bandwidth` | alert on the rate of public inbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/private_outbound_bandwidth` | alert on the rate of private outbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/private_inbound_bandwidth` | alert on the rate of private inbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/load_1` | alert on the 1 minute load average | Droplet ID `v1/insights/lbaas/avg_cpu_utilization_percent`|alert on the percent of CPU utilization|load balancer ID `v1/insights/lbaas/connection_utilization_percent`|alert on the percent of connection utilization|load balancer ID `v1/insights/lbaas/droplet_health`|alert on Droplet health status changes|load balancer ID `v1/insights/lbaas/tls_connections_per_second_utilization_percent`|alert on the percent of TLS connections per second utilization|load balancer ID  | [required] |

### Return type

[**crate::models::MonitoringCreateAlertPolicy200Response**](monitoring_create_alertPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_delete_alert_policy

> monitoring_delete_alert_policy(alert_uuid)
Delete an Alert Policy

To delete an alert policy, send a DELETE request to `/v2/monitoring/alerts/{alert_uuid}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_uuid** | **String** | A unique identifier for an alert policy. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_alert_policy

> crate::models::MonitoringCreateAlertPolicy200Response monitoring_get_alert_policy(alert_uuid)
Retrieve an Existing Alert Policy

To retrieve a given alert policy, send a GET request to `/v2/monitoring/alerts/{alert_uuid}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_uuid** | **String** | A unique identifier for an alert policy. | [required] |

### Return type

[**crate::models::MonitoringCreateAlertPolicy200Response**](monitoring_create_alertPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_bandwidth_metrics

> crate::models::Metrics monitoring_get_droplet_bandwidth_metrics(host_id, interface, direction, start, end)
Get Droplet Bandwidth Metrics

To retrieve bandwidth metrics for a given Droplet, send a GET request to `/v2/monitoring/metrics/droplet/bandwidth`. Use the `interface` query parameter to specify if the results should be for the `private` or `public` interface. Use the `direction` query parameter to specify if the results should be for `inbound` or `outbound` traffic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**interface** | **String** | The network interface. | [required] |
**direction** | **String** | The traffic direction. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_cpu_metrics

> crate::models::Metrics monitoring_get_droplet_cpu_metrics(host_id, start, end)
Get Droplet CPU Metrics

To retrieve CPU metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/cpu`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_filesystem_free_metrics

> crate::models::Metrics monitoring_get_droplet_filesystem_free_metrics(host_id, start, end)
Get Droplet Filesystem Free Metrics

To retrieve filesystem free metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/filesystem_free`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_filesystem_size_metrics

> crate::models::Metrics monitoring_get_droplet_filesystem_size_metrics(host_id, start, end)
Get Droplet Filesystem Size Metrics

To retrieve filesystem size metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/filesystem_size`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_load15_metrics

> crate::models::Metrics monitoring_get_droplet_load15_metrics(host_id, start, end)
Get Droplet Load15 Metrics

To retrieve 15 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_15`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_load1_metrics

> crate::models::Metrics monitoring_get_droplet_load1_metrics(host_id, start, end)
Get Droplet Load1 Metrics

To retrieve 1 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_1`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_load5_metrics

> crate::models::Metrics monitoring_get_droplet_load5_metrics(host_id, start, end)
Get Droplet Load5 Metrics

To retrieve 5 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_5`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_memory_available_metrics

> crate::models::Metrics monitoring_get_droplet_memory_available_metrics(host_id, start, end)
Get Droplet Available Memory Metrics

To retrieve available memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_available`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_memory_cached_metrics

> crate::models::Metrics monitoring_get_droplet_memory_cached_metrics(host_id, start, end)
Get Droplet Cached Memory Metrics

To retrieve cached memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_cached`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_memory_free_metrics

> crate::models::Metrics monitoring_get_droplet_memory_free_metrics(host_id, start, end)
Get Droplet Free Memory Metrics

To retrieve free memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_free`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_memory_total_metrics

> crate::models::Metrics monitoring_get_droplet_memory_total_metrics(host_id, start, end)
Get Droplet Total Memory Metrics

To retrieve total memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_total`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | Timestamp to start metric window. | [required] |
**end** | **String** | Timestamp to end metric window. | [required] |

### Return type

[**crate::models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_list_alert_policy

> crate::models::MonitoringListAlertPolicy200Response monitoring_list_alert_policy(per_page, page)
List Alert Policies

Returns all alert policies that are configured for the given account. To List all alert policies, send a GET request to `/v2/monitoring/alerts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::MonitoringListAlertPolicy200Response**](monitoring_list_alertPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_update_alert_policy

> crate::models::MonitoringCreateAlertPolicy200Response monitoring_update_alert_policy(alert_uuid, alert_policy_request)
Update an Alert Policy

To update en existing policy, send a PUT request to `v2/monitoring/alerts/{alert_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_uuid** | **String** | A unique identifier for an alert policy. | [required] |
**alert_policy_request** | [**AlertPolicyRequest**](AlertPolicyRequest.md) | The `type` field dictates what type of entity that the alert policy applies to and hence what type of entity is passed in the `entities` array. If both the `tags` array and `entities` array are empty the alert policy applies to all entities of the relevant type that are owned by the user account. Otherwise the following table shows the valid entity types for each type of alert policy:  Type | Description | Valid Entity Type -----|-------------|-------------------- `v1/insights/droplet/memory_utilization_percent` | alert on the percent of memory utilization | Droplet ID `v1/insights/droplet/disk_read` | alert on the rate of disk read I/O in MBps | Droplet ID `v1/insights/droplet/load_5` | alert on the 5 minute load average | Droplet ID `v1/insights/droplet/load_15` | alert on the 15 minute load average | Droplet ID `v1/insights/droplet/disk_utilization_percent` | alert on the percent of disk utilization | Droplet ID `v1/insights/droplet/cpu` | alert on the percent of CPU utilization | Droplet ID `v1/insights/droplet/disk_write` | alert on the rate of disk write I/O in MBps | Droplet ID `v1/insights/droplet/public_outbound_bandwidth` | alert on the rate of public outbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/public_inbound_bandwidth` | alert on the rate of public inbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/private_outbound_bandwidth` | alert on the rate of private outbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/private_inbound_bandwidth` | alert on the rate of private inbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/load_1` | alert on the 1 minute load average | Droplet ID `v1/insights/lbaas/avg_cpu_utilization_percent`|alert on the percent of CPU utilization|load balancer ID `v1/insights/lbaas/connection_utilization_percent`|alert on the percent of connection utilization|load balancer ID `v1/insights/lbaas/droplet_health`|alert on Droplet health status changes|load balancer ID `v1/insights/lbaas/tls_connections_per_second_utilization_percent`|alert on the percent of TLS connections per second utilization|load balancer ID  | [required] |

### Return type

[**crate::models::MonitoringCreateAlertPolicy200Response**](monitoring_create_alertPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

