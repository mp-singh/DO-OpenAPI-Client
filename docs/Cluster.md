# Cluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference a Kubernetes cluster. | [optional][readonly]
**name** | **String** | A human-readable name for a Kubernetes cluster. | 
**region** | **String** | The slug identifier for the region where the Kubernetes cluster is located. | 
**version** | **String** | The slug identifier for the version of Kubernetes used for the cluster. If set to a minor version (e.g. \"1.14\"), the latest version within it will be used (e.g. \"1.14.6-do.1\"); if set to \"latest\", the latest published version will be used. See the `/v2/kubernetes/options` endpoint to find all currently available versions. | 
**cluster_subnet** | Option<**String**> | The range of IP addresses in the overlay network of the Kubernetes cluster in CIDR notation. | [optional][readonly]
**service_subnet** | Option<**String**> | The range of assignable IP addresses for services running in the Kubernetes cluster in CIDR notation. | [optional][readonly]
**vpc_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A string specifying the UUID of the VPC to which the Kubernetes cluster is assigned. | [optional]
**ipv4** | Option<**String**> | The public IPv4 address of the Kubernetes master node. This will not be set if high availability is configured on the cluster (v1.21+) | [optional][readonly]
**endpoint** | Option<**String**> | The base URL of the API server on the Kubernetes master node. | [optional][readonly]
**tags** | Option<**Vec<String>**> | An array of tags applied to the Kubernetes cluster. All clusters are automatically tagged `k8s` and `k8s:$K8S_CLUSTER_ID`. | [optional]
**node_pools** | [**Vec<crate::models::KubernetesNodePool>**](kubernetes_node_pool.md) | An object specifying the details of the worker nodes available to the Kubernetes cluster. | 
**maintenance_policy** | Option<[**crate::models::MaintenancePolicy**](maintenance_policy.md)> |  | [optional]
**auto_upgrade** | Option<**bool**> | A boolean value indicating whether the cluster will be automatically upgraded to new patch releases during its maintenance window. | [optional][default to false]
**status** | Option<[**crate::models::ClusterStatus**](cluster_status.md)> |  | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the Kubernetes cluster was created. | [optional][readonly]
**updated_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the Kubernetes cluster was last updated. | [optional][readonly]
**surge_upgrade** | Option<**bool**> | A boolean value indicating whether surge upgrade is enabled/disabled for the cluster. Surge upgrade makes cluster upgrades fast and reliable by bringing up new nodes before destroying the outdated nodes. | [optional][default to false]
**ha** | Option<**bool**> | A boolean value indicating whether the control plane is run in a highly available configuration in the cluster. Highly available control planes incur less downtime. | [optional][default to false]
**registry_enabled** | Option<**bool**> | A read-only boolean value indicating if a container registry is integrated with the cluster. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


