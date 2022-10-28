# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**droplet_limit** | **i32** | The total number of Droplets current user or team may have active at one time. | 
**floating_ip_limit** | **i32** | The total number of Floating IPs the current user or team may have. | 
**email** | **String** | The email address used by the current user to register for DigitalOcean. | 
**uuid** | **String** | The unique universal identifier for the current user. | 
**email_verified** | **bool** | If true, the user has verified their account via email. False otherwise. | [default to false]
**status** | **String** | This value is one of \"active\", \"warning\" or \"locked\". | [default to Active]
**status_message** | **String** | A human-readable message giving more details about the status of the account. | 
**team** | Option<[**crate::models::AccountTeam**](account_team.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


