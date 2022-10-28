# AppsValidateRollback200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valid** | Option<**bool**> | Indicates whether the app can be rolled back to the specified deployment. | [optional]
**error** | Option<[**crate::models::AppsValidateRollback200ResponseError**](apps_validate_rollback_200_response_error.md)> |  | [optional]
**warnings** | Option<[**Vec<crate::models::AppRollbackValidationCondition>**](app_rollback_validation_condition.md)> | Contains a list of warnings that may cause the rollback to run under unideal circumstances. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


