# App

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_deployment** | Option<[**crate::models::AppsDeployment**](apps_deployment.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional][readonly]
**default_ingress** | Option<**String**> |  | [optional][readonly]
**domains** | Option<[**Vec<crate::models::AppsDomain>**](apps_domain.md)> |  | [optional][readonly]
**id** | Option<**String**> |  | [optional][readonly]
**in_progress_deployment** | Option<[**crate::models::AppsDeployment**](apps_deployment.md)> |  | [optional]
**last_deployment_created_at** | Option<**String**> |  | [optional][readonly]
**live_domain** | Option<**String**> |  | [optional][readonly]
**live_url** | Option<**String**> |  | [optional][readonly]
**live_url_base** | Option<**String**> |  | [optional][readonly]
**owner_uuid** | Option<**String**> |  | [optional][readonly]
**region** | Option<[**crate::models::AppsRegion**](apps_region.md)> |  | [optional]
**spec** | [**crate::models::AppSpec**](app_spec.md) |  | 
**tier_slug** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**pinned_deployment** | Option<[**crate::models::AppPinnedDeployment**](app_pinned_deployment.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


