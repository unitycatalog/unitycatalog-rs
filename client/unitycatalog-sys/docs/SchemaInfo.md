# SchemaInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of schema, relative to parent catalog. | [optional]
**catalog_name** | Option<**String**> | Name of parent catalog. | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**properties** | Option<**std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**full_name** | Option<**String**> | Full name of schema, in form of __catalog_name__.__schema_name__. | [optional]
**created_at** | Option<**i64**> | Time at which this schema was created, in epoch milliseconds. | [optional]
**updated_at** | Option<**i64**> | Time at which this schema was last modified, in epoch milliseconds. | [optional]
**schema_id** | Option<**String**> | Unique identifier for the schema. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


