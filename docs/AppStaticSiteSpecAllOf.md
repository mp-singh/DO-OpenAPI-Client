# AppStaticSiteSpecAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index_document** | Option<**String**> | The name of the index document to use when serving this static site. Default: index.html | [optional][default to index.html]
**error_document** | Option<**String**> | The name of the error document to use when serving this static site. Default: 404.html. If no such file exists within the built assets, App Platform will supply one. | [optional][default to 404.html]
**catchall_document** | Option<**String**> | The name of the document to use as the fallback for any requests to documents that are not found when serving this static site. Only 1 of `catchall_document` or `error_document` can be set. | [optional]
**output_dir** | Option<**String**> | An optional path to where the built assets will be located, relative to the build context. If not set, App Platform will automatically scan for these directory names: `_static`, `dist`, `public`, `build`. | [optional]
**cors** | Option<[**crate::models::AppsCorsPolicy**](apps_cors_policy.md)> |  | [optional]
**routes** | Option<[**Vec<crate::models::AppRouteSpec>**](app_route_spec.md)> | A list of HTTP routes that should be routed to this component. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


