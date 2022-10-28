# AppSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the app. Must be unique across all apps in the same account. | 
**region** | Option<**String**> | The slug form of the geographical origin of the app. Default: `nearest available` | [optional]
**domains** | Option<[**Vec<crate::models::AppDomainSpec>**](app_domain_spec.md)> | A set of hostnames where the application will be available. | [optional]
**services** | Option<[**Vec<crate::models::AppServiceSpec>**](app_service_spec.md)> | Workloads which expose publicly-accessible HTTP services. | [optional]
**static_sites** | Option<[**Vec<crate::models::AppStaticSiteSpec>**](app_static_site_spec.md)> | Content which can be rendered to static web assets. | [optional]
**jobs** | Option<[**Vec<crate::models::AppJobSpec>**](app_job_spec.md)> | Pre and post deployment workloads which do not expose publicly-accessible HTTP routes. | [optional]
**workers** | Option<[**Vec<crate::models::AppWorkerSpec>**](app_worker_spec.md)> | Workloads which do not expose publicly-accessible HTTP services. | [optional]
**functions** | Option<[**Vec<crate::models::AppFunctionsSpec>**](app_functions_spec.md)> | Workloads which expose publicly-accessible HTTP services via Functions Components. | [optional]
**databases** | Option<[**Vec<crate::models::AppDatabaseSpec>**](app_database_spec.md)> | Database instances which can provide persistence to workloads within the application. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


