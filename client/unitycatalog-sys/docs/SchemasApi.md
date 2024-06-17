# \SchemasApi

All URIs are relative to *http://localhost:8080/api/2.1/unity-catalog*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_schema**](SchemasApi.md#create_schema) | **POST** /schemas | Create a schema
[**delete_schema**](SchemasApi.md#delete_schema) | **DELETE** /schemas/{full_name} | Delete a schema
[**get_schema**](SchemasApi.md#get_schema) | **GET** /schemas/{full_name} | Get a schema
[**list_schemas**](SchemasApi.md#list_schemas) | **GET** /schemas | List schemas
[**update_schema**](SchemasApi.md#update_schema) | **PATCH** /schemas/{full_name} | Update a schema



## create_schema

> models::SchemaInfo create_schema(create_schema)
Create a schema

Creates a new schema in the specified catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_schema** | Option<[**CreateSchema**](CreateSchema.md)> |  |  |

### Return type

[**models::SchemaInfo**](SchemaInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_schema

> serde_json::Value delete_schema(full_name)
Delete a schema

Deletes the specified schema from the parent catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the schema. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema

> models::SchemaInfo get_schema(full_name)
Get a schema

Gets the specified schema for a catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the schema. | [required] |

### Return type

[**models::SchemaInfo**](SchemaInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_schemas

> models::ListSchemasResponse list_schemas(catalog_name, max_results, page_token)
List schemas

Gets an array of schemas for a catalog. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | Parent catalog for schemas of interest. | [required] |
**max_results** | Option<**i32**> | Maximum number of schemas to return. - when set to a value greater than 0, the page length is the minimum of this value and a server configured value; - when set to 0, the page length is set to a server configured value; - when set to a value less than 0, an invalid parameter error is returned;  |  |
**page_token** | Option<**String**> | Opaque pagination token to go to next page based on previous query.  |  |

### Return type

[**models::ListSchemasResponse**](ListSchemasResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_schema

> models::SchemaInfo update_schema(full_name, update_schema)
Update a schema

Updates the specified schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the schema. | [required] |
**update_schema** | Option<[**UpdateSchema**](UpdateSchema.md)> |  |  |

### Return type

[**models::SchemaInfo**](SchemaInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

