# \FunctionsApi

All URIs are relative to *http://localhost:8080/api/2.1/unity-catalog*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_function**](FunctionsApi.md#create_function) | **POST** /functions | Create a function. WARNING: This API is experimental and will change in future versions. 
[**delete_function**](FunctionsApi.md#delete_function) | **DELETE** /functions/{name} | Delete a function
[**get_function**](FunctionsApi.md#get_function) | **GET** /functions/{name} | Get a function
[**list_functions**](FunctionsApi.md#list_functions) | **GET** /functions | List functions



## create_function

> models::FunctionInfo create_function(create_function_request)
Create a function. WARNING: This API is experimental and will change in future versions. 

Creates a new function instance. WARNING: This API is experimental and will change in future versions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_function_request** | Option<[**CreateFunctionRequest**](CreateFunctionRequest.md)> |  |  |

### Return type

[**models::FunctionInfo**](FunctionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_function

> serde_json::Value delete_function(name)
Delete a function

Deletes the function that matches the supplied name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The fully-qualified name of the function (of the form __catalog_name__.__schema_name__.__function__name__). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_function

> models::FunctionInfo get_function(name)
Get a function

Gets a function from within a parent catalog and schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The fully-qualified name of the function (of the form __catalog_name__.__schema_name__.__function__name__). | [required] |

### Return type

[**models::FunctionInfo**](FunctionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_functions

> models::ListFunctionsResponse list_functions(catalog_name, schema_name, max_results, page_token)
List functions

List functions within the specified parent catalog and schema. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | Name of parent catalog for functions of interest. | [required] |
**schema_name** | **String** | Parent schema of functions. | [required] |
**max_results** | Option<**i32**> | Maximum number of functions to return. - when set to a value greater than 0, the page length is the minimum of this value and a server configured value; - when set to 0, the page length is set to a server configured value; - when set to a value less than 0, an invalid parameter error is returned;  |  |
**page_token** | Option<**String**> | Opaque pagination token to go to next page based on previous query. |  |

### Return type

[**models::ListFunctionsResponse**](ListFunctionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

