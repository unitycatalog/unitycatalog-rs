# \CatalogsApi

All URIs are relative to *http://localhost:8080/api/2.1/unity-catalog*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_catalog**](CatalogsApi.md#create_catalog) | **POST** /catalogs | Create a catalog
[**delete_catalog**](CatalogsApi.md#delete_catalog) | **DELETE** /catalogs/{name} | Delete a catalog
[**get_catalog**](CatalogsApi.md#get_catalog) | **GET** /catalogs/{name} | Get a catalog
[**list_catalogs**](CatalogsApi.md#list_catalogs) | **GET** /catalogs | List catalogs
[**update_catalog**](CatalogsApi.md#update_catalog) | **PATCH** /catalogs/{name} | Update a catalog



## create_catalog

> models::CatalogInfo create_catalog(create_catalog)
Create a catalog

Creates a new catalog instance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_catalog** | Option<[**CreateCatalog**](CreateCatalog.md)> |  |  |

### Return type

[**models::CatalogInfo**](CatalogInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_catalog

> serde_json::Value delete_catalog(name, force)
Delete a catalog

Deletes the catalog that matches the supplied name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the catalog. | [required] |
**force** | Option<**bool**> | Force deletion even if the catalog is not empty. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_catalog

> models::CatalogInfo get_catalog(name)
Get a catalog

Gets the specified catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the catalog. | [required] |

### Return type

[**models::CatalogInfo**](CatalogInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_catalogs

> models::ListCatalogsResponse list_catalogs(page_token, max_results)
List catalogs

Lists the available catalogs. There is no guarantee of a specific ordering of the elements in the list. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_token** | Option<**String**> | Opaque pagination token to go to next page based on previous query.  |  |
**max_results** | Option<**i32**> | Maximum number of catalogs to return. - when set to a value greater than 0, the page length is the minimum of this value and a server configured value; - when set to 0, the page length is set to a server configured value; - when set to a value less than 0, an invalid parameter error is returned;  |  |

### Return type

[**models::ListCatalogsResponse**](ListCatalogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_catalog

> models::CatalogInfo update_catalog(name, update_catalog)
Update a catalog

Updates the catalog that matches the supplied name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the catalog. | [required] |
**update_catalog** | Option<[**UpdateCatalog**](UpdateCatalog.md)> |  |  |

### Return type

[**models::CatalogInfo**](CatalogInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

