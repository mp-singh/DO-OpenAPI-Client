# ClusterUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A human-readable name for a Kubernetes cluster. | 
**tags** | Option<**Vec<String>**> | An array of tags applied to the Kubernetes cluster. All clusters are automatically tagged `k8s` and `k8s:$K8S_CLUSTER_ID`. | [optional]
**maintenance_policy** | Option<[**crate::models::MaintenancePolicy**](maintenance_policy.md)> |  | [optional]
**auto_upgrade** | Option<**bool**> | A boolean value indicating whether the cluster will be automatically upgraded to new patch releases during its maintenance window. | [optional][default to false]
**surge_upgrade** | Option<**bool**> | A boolean value indicating whether surge upgrade is enabled/disabled for the cluster. Surge upgrade makes cluster upgrades fast and reliable by bringing up new nodes before destroying the outdated nodes. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


