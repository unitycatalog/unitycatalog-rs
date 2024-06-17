# FunctionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of function, relative to parent schema. | [optional]
**catalog_name** | Option<**String**> | Name of parent catalog. | [optional]
**schema_name** | Option<**String**> | Name of parent schema relative to its parent catalog. | [optional]
**input_params** | Option<[**models::FunctionParameterInfos**](FunctionParameterInfos.md)> |  | [optional]
**data_type** | Option<[**models::ColumnTypeName**](ColumnTypeName.md)> |  | [optional]
**full_data_type** | Option<**String**> | Pretty printed function data type. | [optional]
**return_params** | Option<[**models::FunctionParameterInfos**](FunctionParameterInfos.md)> |  | [optional]
**routine_body** | Option<**String**> | Function language. When **EXTERNAL** is used, the language of the routine function should be specified in the __external_language__ field,  and the __return_params__ of the function cannot be used (as **TABLE** return type is not supported), and the __sql_data_access__ field must be **NO_SQL**.  | [optional]
**routine_definition** | Option<**String**> | Function body. | [optional]
**routine_dependencies** | Option<[**models::DependencyList**](DependencyList.md)> |  | [optional]
**parameter_style** | Option<**String**> | Function parameter style. **S** is the value for SQL. | [optional]
**is_deterministic** | Option<**bool**> | Whether the function is deterministic. | [optional]
**sql_data_access** | Option<**String**> | Function SQL data access. | [optional]
**is_null_call** | Option<**bool**> | Function null call. | [optional]
**security_type** | Option<**String**> | Function security type. | [optional]
**specific_name** | Option<**String**> | Specific name of the function; Reserved for future use. | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**properties** | Option<**String**> | JSON-serialized key-value pair map, encoded (escaped) as a string. | [optional]
**full_name** | Option<**String**> | Full name of function, in form of __catalog_name__.__schema_name__.__function__name__ | [optional]
**created_at** | Option<**i64**> | Time at which this function was created, in epoch milliseconds. | [optional]
**updated_at** | Option<**i64**> | Time at which this function was last updated, in epoch milliseconds. | [optional]
**function_id** | Option<**String**> | Id of Function, relative to parent schema. | [optional]
**external_language** | Option<**String**> | External language of the function. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


