# TableInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of table, relative to parent schema. | [optional]
**catalog_name** | Option<**String**> | Name of parent catalog. | [optional]
**schema_name** | Option<**String**> | Name of parent schema relative to its parent catalog. | [optional]
**table_type** | Option<[**models::TableType**](TableType.md)> |  | [optional]
**data_source_format** | Option<[**models::DataSourceFormat**](DataSourceFormat.md)> |  | [optional]
**columns** | Option<[**Vec<models::ColumnInfo>**](ColumnInfo.md)> | The array of __ColumnInfo__ definitions of the table's columns. | [optional]
**storage_location** | Option<**String**> | Storage root URL for table (for **MANAGED**, **EXTERNAL** tables) | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**properties** | Option<**std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**created_at** | Option<**i64**> | Time at which this table was created, in epoch milliseconds. | [optional]
**updated_at** | Option<**i64**> | Time at which this table was last modified, in epoch milliseconds. | [optional]
**table_id** | Option<**String**> | Unique identifier for the table. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


