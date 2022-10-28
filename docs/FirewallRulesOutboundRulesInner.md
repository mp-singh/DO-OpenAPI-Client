# FirewallRulesOutboundRulesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | **String** | The type of traffic to be allowed. This may be one of `tcp`, `udp`, or `icmp`. | 
**ports** | **String** | The ports on which traffic will be allowed specified as a string containing a single port, a range (e.g. \"8000-9000\"), or \"0\" when all ports are open for a protocol. For ICMP rules this parameter will always return \"0\". | 
**destinations** | [**crate::models::FirewallRulesOutboundRulesInnerAllOfDestinations**](firewall_rules_outbound_rules_inner_allOf_destinations.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


