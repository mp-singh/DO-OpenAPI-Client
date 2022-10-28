# AppServiceSpecAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cors** | Option<[**crate::models::AppsCorsPolicy**](apps_cors_policy.md)> |  | [optional]
**health_check** | Option<[**crate::models::AppServiceSpecHealthCheck**](app_service_spec_health_check.md)> |  | [optional]
**http_port** | Option<**i64**> | The internal port on which this service's run command will listen. Default: 8080 If there is not an environment variable with the name `PORT`, one will be automatically added with its value set to the value of this field. | [optional]
**internal_ports** | Option<**Vec<i64>**> | The ports on which this service will listen for internal traffic. | [optional]
**routes** | Option<[**Vec<crate::models::AppRouteSpec>**](app_route_spec.md)> | A list of HTTP routes that should be routed to this component. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


