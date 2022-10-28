# AppsImageSourceSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**registry** | Option<**String**> | The registry name. Must be left empty for the `DOCR` registry type. | [optional]
**registry_type** | Option<**String**> | - DOCKER_HUB: The DockerHub container registry type. - DOCR: The DigitalOcean container registry type. | [optional]
**repository** | Option<**String**> | The repository name. | [optional]
**tag** | Option<**String**> | The repository tag. Defaults to `latest` if not provided. | [optional][default to latest]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


