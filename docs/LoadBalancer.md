# LoadBalancer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference a load balancer. | [optional][readonly]
**name** | Option<**String**> | A human-readable name for a load balancer instance. | [optional]
**ip** | Option<**String**> | An attribute containing the public-facing IP address of the load balancer. | [optional][readonly]
**size_unit** | Option<**i32**> | How many nodes the load balancer contains. Each additional node increases the load balancer's ability to manage more connections. Load balancers can be scaled up or down, and you can change the number of nodes after creation up to once per hour. This field is currently not available in the AMS2, NYC2, or SFO1 regions. Use the `size` field to scale load balancers that reside in these regions. | [optional][default to 1]
**size** | Option<**String**> | This field has been replaced by the `size_unit` field for all regions except in AMS2, NYC2, and SFO1. Each available load balancer size now equates to the load balancer having a set number of nodes. * `lb-small` = 1 node * `lb-medium` = 3 nodes * `lb-large` = 6 nodes  You can resize load balancers after creation up to once per hour. You cannot resize a load balancer within the first hour of its creation. | [optional][default to Small]
**algorithm** | Option<**String**> | This field has been deprecated. You can no longer specify an algorithm for load balancers. | [optional][default to RoundRobin]
**status** | Option<**String**> | A status string indicating the current state of the load balancer. This can be `new`, `active`, or `errored`. | [optional][readonly]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the load balancer was created. | [optional][readonly]
**forwarding_rules** | [**Vec<crate::models::ForwardingRule>**](forwarding_rule.md) | An array of objects specifying the forwarding rules for a load balancer. | 
**health_check** | Option<[**crate::models::HealthCheck**](health_check.md)> |  | [optional]
**sticky_sessions** | Option<[**crate::models::StickySessions**](sticky_sessions.md)> |  | [optional]
**redirect_http_to_https** | Option<**bool**> | A boolean value indicating whether HTTP requests to the load balancer on port 80 will be redirected to HTTPS on port 443. | [optional][default to false]
**enable_proxy_protocol** | Option<**bool**> | A boolean value indicating whether PROXY Protocol is in use. | [optional][default to false]
**enable_backend_keepalive** | Option<**bool**> | A boolean value indicating whether HTTP keepalive connections are maintained to target Droplets. | [optional][default to false]
**vpc_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A string specifying the UUID of the VPC to which the load balancer is assigned. | [optional]
**disable_lets_encrypt_dns_records** | Option<**bool**> | A boolean value indicating whether to disable automatic DNS record creation for Let's Encrypt certificates that are added to the load balancer. | [optional][default to false]
**region** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [optional]
**droplet_ids** | Option<**Vec<i32>**> | An array containing the IDs of the Droplets assigned to the load balancer. | [optional]
**tag** | Option<**String**> | The name of a Droplet tag corresponding to Droplets assigned to the load balancer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


