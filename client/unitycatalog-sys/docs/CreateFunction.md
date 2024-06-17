# CreateFunction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of function, relative to parent schema. | 
**catalog_name** | **String** | Name of parent catalog. | 
**schema_name** | **String** | Name of parent schema relative to its parent catalog. | 
**input_params** | [**models::FunctionParameterInfos**](FunctionParameterInfos.md) |  | 
**data_type** | [**models::ColumnTypeName**](ColumnTypeName.md) |  | 
**full_data_type** | **String** | Pretty printed function data type. | 
**return_params** | Option<[**models::FunctionParameterInfos**](FunctionParameterInfos.md)> |  | [optional]
**routine_body** | **String** | Function language. When **EXTERNAL** is used, the language of the routine function should be specified in the __external_language__ field,  and the __return_params__ of the function cannot be used (as **TABLE** return type is not supported), and the __sql_data_access__ field must be **NO_SQL**.  | 
**routine_definition** | **String** | Function body. | 
**routine_dependencies** | Option<[**models::DependencyList**](DependencyList.md)> |  | [optional]
**parameter_style** | **String** | Function parameter style. **S** is the value for SQL. | 
**is_deterministic** | **bool** | Whether the function is deterministic. | 
**sql_data_access** | **String** | Function SQL data access. | 
**is_null_call** | **bool** | Function null call. | 
**security_type** | **String** | Function security type. | 
**specific_name** | **String** | Specific name of the function; Reserved for future use. | 
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**properties** | **String** | JSON-serialized key-value pair map, encoded (escaped) as a string. | 
**external_language** | Option<**String**> | External language of the function. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


