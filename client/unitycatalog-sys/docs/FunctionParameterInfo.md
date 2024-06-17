# FunctionParameterInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of parameter. | 
**type_text** | **String** | Full data type spec, SQL/catalogString text. | 
**type_json** | **String** | Full data type spec, JSON-serialized. | 
**type_name** | [**models::ColumnTypeName**](ColumnTypeName.md) |  | 
**type_precision** | Option<**i32**> | Digits of precision; required on Create for DecimalTypes. | [optional]
**type_scale** | Option<**i32**> | Digits to right of decimal; Required on Create for DecimalTypes. | [optional]
**type_interval_type** | Option<**String**> | Format of IntervalType. | [optional]
**position** | **i32** | Ordinal position of column (starting at position 0). | 
**parameter_mode** | Option<[**models::FunctionParameterMode**](FunctionParameterMode.md)> |  | [optional]
**parameter_type** | Option<[**models::FunctionParameterType**](FunctionParameterType.md)> |  | [optional]
**parameter_default** | Option<**String**> | Default value of the parameter. | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


