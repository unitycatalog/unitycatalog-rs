# \TablesApi

All URIs are relative to *http://localhost:8080/api/2.1/unity-catalog*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_table**](TablesApi.md#create_table) | **POST** /tables | Create a table. WARNING: This API is experimental and will change in future versions. 
[**delete_table**](TablesApi.md#delete_table) | **DELETE** /tables/{full_name} | Delete a table
[**get_table**](TablesApi.md#get_table) | **GET** /tables/{full_name} | Get a table
[**list_tables**](TablesApi.md#list_tables) | **GET** /tables | List tables



## create_table

> models::TableInfo create_table(create_table)
Create a table. WARNING: This API is experimental and will change in future versions. 

Creates a new table instance. WARNING: This API is experimental and will change in future versions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_table** | Option<[**CreateTable**](CreateTable.md)> |  |  |

### Return type

[**models::TableInfo**](TableInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_table

> serde_json::Value delete_table(full_name)
Delete a table

Deletes a table from the specified parent catalog and schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the table. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_table

> models::TableInfo get_table(full_name)
Get a table

Gets a table for a specific catalog and schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the table. | [required] |

### Return type

[**models::TableInfo**](TableInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tables

> models::ListTablesResponse list_tables(catalog_name, schema_name, max_results, page_token)
List tables

Gets the list of all available tables under the parent catalog and schema. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | Name of parent catalog for tables of interest. | [required] |
**schema_name** | **String** | Parent schema of tables. | [required] |
**max_results** | Option<**i32**> | Maximum number of tables to return. - when set to a value greater than 0, the page length is the minimum of this value and a server configured value; - when set to 0, the page length is set to a server configured value; - when set to a value less than 0, an invalid parameter error is returned;  |  |
**page_token** | Option<**String**> | Opaque token to send for the next page of results (pagination). |  |

### Return type

[**models::ListTablesResponse**](ListTablesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

