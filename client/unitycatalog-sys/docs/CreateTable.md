# CreateTable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of table, relative to parent schema. | 
**catalog_name** | **String** | Name of parent catalog. | 
**schema_name** | **String** | Name of parent schema relative to its parent catalog. | 
**table_type** | [**models::TableType**](TableType.md) |  | 
**data_source_format** | [**models::DataSourceFormat**](DataSourceFormat.md) |  | 
**columns** | [**Vec<models::ColumnInfo>**](ColumnInfo.md) | The array of __ColumnInfo__ definitions of the table's columns. | 
**storage_location** | Option<**String**> | Storage root URL for table (for **MANAGED**, **EXTERNAL** tables) | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**properties** | Option<**std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


