# DatabaseUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of a database user. | 
**role** | Option<**String**> | A string representing the database user's role. The value will be either \"primary\" or \"normal\".  | [optional][readonly]
**password** | Option<**String**> | A randomly generated password for the database user. | [optional][readonly]
**mysql_settings** | Option<[**crate::models::MysqlSettings**](mysql_settings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


