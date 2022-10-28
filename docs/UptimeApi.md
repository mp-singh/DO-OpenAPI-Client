# \UptimeApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**uptime_alert_create**](UptimeApi.md#uptime_alert_create) | **POST** /v2/uptime/checks/{check_id}/alerts | Create a New Alert
[**uptime_alert_delete**](UptimeApi.md#uptime_alert_delete) | **DELETE** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Delete an Alert
[**uptime_alert_get**](UptimeApi.md#uptime_alert_get) | **GET** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Retrieve an Existing Alert
[**uptime_alert_update**](UptimeApi.md#uptime_alert_update) | **PUT** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Update an Alert
[**uptime_check_alerts_list**](UptimeApi.md#uptime_check_alerts_list) | **GET** /v2/uptime/checks/{check_id}/alerts | List All Alerts
[**uptime_check_create**](UptimeApi.md#uptime_check_create) | **POST** /v2/uptime/checks | Create a New Check
[**uptime_check_delete**](UptimeApi.md#uptime_check_delete) | **DELETE** /v2/uptime/checks/{check_id} | Delete a Check
[**uptime_check_get**](UptimeApi.md#uptime_check_get) | **GET** /v2/uptime/checks/{check_id} | Retrieve an Existing Check
[**uptime_check_state_get**](UptimeApi.md#uptime_check_state_get) | **GET** /v2/uptime/checks/{check_id}/state | Retrieve Check State
[**uptime_check_update**](UptimeApi.md#uptime_check_update) | **PUT** /v2/uptime/checks/{check_id} | Update a Check
[**uptime_checks_list**](UptimeApi.md#uptime_checks_list) | **GET** /v2/uptime/checks | List All Checks



## uptime_alert_create

> crate::models::UptimeAlertCreate201Response uptime_alert_create(check_id, uptime_alert_create_request)
Create a New Alert

To create an Uptime alert, send a POST request to `/v2/uptime/checks/$CHECK_ID/alerts` specifying the attributes in the table below in the JSON body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**uptime_alert_create_request** | [**UptimeAlertCreateRequest**](UptimeAlertCreateRequest.md) | The ''type'' field dictates the type of alert, and hence what type of value to pass into the threshold property. Type | Description | Threshold Value -----|-------------|-------------------- `latency` | alerts on the response latency | milliseconds `down` | alerts on a target registering as down in any region | N/A (Not required) `down_global` | alerts on a target registering as down globally | N/A (Not required) `ssl_expiry` | alerts on a SSL certificate expiring within $threshold days | days  | [required] |

### Return type

[**crate::models::UptimeAlertCreate201Response**](uptime_alert_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_alert_delete

> uptime_alert_delete(check_id, alert_id)
Delete an Alert

To delete an Uptime alert, send a DELETE request to `/v2/uptime/checks/$CHECK_ID/alerts/$ALERT_ID`. A 204 status code with no body will be returned in response to a successful request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**alert_id** | **uuid::Uuid** | A unique identifier for an alert. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_alert_get

> crate::models::UptimeAlertCreate201Response uptime_alert_get(check_id, alert_id)
Retrieve an Existing Alert

To show information about an existing alert, send a GET request to `/v2/uptime/checks/$CHECK_ID/alerts/$ALERT_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**alert_id** | **uuid::Uuid** | A unique identifier for an alert. | [required] |

### Return type

[**crate::models::UptimeAlertCreate201Response**](uptime_alert_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_alert_update

> crate::models::UptimeAlertCreate201Response uptime_alert_update(check_id, alert_id, uptime_alert_update_request)
Update an Alert

To update the settings of an Uptime alert, send a PUT request to `/v2/uptime/checks/$CHECK_ID/alerts/$ALERT_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**alert_id** | **uuid::Uuid** | A unique identifier for an alert. | [required] |
**uptime_alert_update_request** | [**UptimeAlertUpdateRequest**](UptimeAlertUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::UptimeAlertCreate201Response**](uptime_alert_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_check_alerts_list

> crate::models::UptimeCheckAlertsList200Response uptime_check_alerts_list(check_id, per_page, page)
List All Alerts

To list all of the alerts for an Uptime check, send a GET request to `/v2/uptime/checks/$CHECK_ID/alerts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::UptimeCheckAlertsList200Response**](uptime_check_alerts_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_check_create

> crate::models::UptimeCheckCreate201Response uptime_check_create(uptime_check_create_request)
Create a New Check

To create an Uptime check, send a POST request to `/v2/uptime/checks` specifying the attributes in the table below in the JSON body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uptime_check_create_request** | [**UptimeCheckCreateRequest**](UptimeCheckCreateRequest.md) |  | [required] |

### Return type

[**crate::models::UptimeCheckCreate201Response**](uptime_check_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_check_delete

> uptime_check_delete(check_id)
Delete a Check

To delete an Uptime check, send a DELETE request to `/v2/uptime/checks/$CHECK_ID`. A 204 status code with no body will be returned in response to a successful request.   Deleting a check will also delete alerts associated with the check. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_check_get

> crate::models::UptimeCheckCreate201Response uptime_check_get(check_id)
Retrieve an Existing Check

To show information about an existing check, send a GET request to `/v2/uptime/checks/$CHECK_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |

### Return type

[**crate::models::UptimeCheckCreate201Response**](uptime_check_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_check_state_get

> crate::models::UptimeCheckStateGet200Response uptime_check_state_get(check_id)
Retrieve Check State

To show information about an existing check's state, send a GET request to `/v2/uptime/checks/$CHECK_ID/state`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |

### Return type

[**crate::models::UptimeCheckStateGet200Response**](uptime_check_state_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_check_update

> crate::models::UptimeCheckCreate201Response uptime_check_update(check_id, uptime_check_update_request)
Update a Check

To update the settings of an Uptime check, send a PUT request to `/v2/uptime/checks/$CHECK_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**uptime_check_update_request** | [**UptimeCheckUpdateRequest**](UptimeCheckUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::UptimeCheckCreate201Response**](uptime_check_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_checks_list

> crate::models::UptimeChecksList200Response uptime_checks_list(per_page, page)
List All Checks

To list all of the Uptime checks on your account, send a GET request to `/v2/uptime/checks`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**crate::models::UptimeChecksList200Response**](uptime_checks_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

