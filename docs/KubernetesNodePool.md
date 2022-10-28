# KubernetesNodePool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size** | **String** | The slug identifier for the type of Droplet used as workers in the node pool. | 
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference a specific node pool. | [optional][readonly]
**name** | **String** | A human-readable name for the node pool. | 
**count** | **i32** | The number of Droplet instances in the node pool. | 
**tags** | Option<**Vec<String>**> | An array containing the tags applied to the node pool. All node pools are automatically tagged `k8s`, `k8s-worker`, and `k8s:$K8S_CLUSTER_ID`. | [optional]
**labels** | Option<[**serde_json::Value**](.md)> | An object containing a set of Kubernetes labels. The keys and are values are both user-defined. | [optional]
**taints** | Option<[**Vec<crate::models::KubernetesNodePoolTaint>**](kubernetes_node_pool_taint.md)> | An array of taints to apply to all nodes in a pool. Taints will automatically be applied to all existing nodes and any subsequent nodes added to the pool. When a taint is removed, it is removed from all nodes in the pool. | [optional]
**auto_scale** | Option<**bool**> | A boolean value indicating whether auto-scaling is enabled for this node pool. | [optional]
**min_nodes** | Option<**i32**> | The minimum number of nodes that this node pool can be auto-scaled to. The value will be `0` if `auto_scale` is set to `false`. | [optional]
**max_nodes** | Option<**i32**> | The maximum number of nodes that this node pool can be auto-scaled to. The value will be `0` if `auto_scale` is set to `false`. | [optional]
**nodes** | Option<[**Vec<crate::models::Node>**](node.md)> | An object specifying the details of a specific worker node in a node pool. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


