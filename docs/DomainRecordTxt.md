# DomainRecordTxt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | A unique identifier for each domain record. | [optional][readonly]
**r#type** | **String** | The type of the DNS record. For example: A, CNAME, TXT, ... | 
**name** | **String** | The host name, alias, or service being defined by the record. | 
**data** | **String** | Variable data depending on record type. For example, the \"data\" value for an A record would be the IPv4 address to which the domain will be mapped. For a CAA record, it would contain the domain name of the CA being granted permission to issue certificates. | 
**priority** | Option<**i32**> | The priority for SRV and MX records. | [optional]
**port** | Option<**i32**> | The port for SRV records. | [optional]
**ttl** | Option<**i32**> | This value is the time to live for the record, in seconds. This defines the time frame that clients can cache queried information before a refresh should be requested. | [optional]
**weight** | Option<**i32**> | The weight for SRV records. | [optional]
**flags** | Option<**i32**> | An unsigned integer between 0-255 used for CAA records. | 
**tag** | Option<**String**> | The parameter tag for CAA records. Valid values are \"issue\", \"issuewild\", or \"iodef\" | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


