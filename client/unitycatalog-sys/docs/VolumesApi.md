# \VolumesApi

All URIs are relative to *http://localhost:8080/api/2.1/unity-catalog*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume**](VolumesApi.md#create_volume) | **POST** /volumes | Create a Volume
[**delete_volume**](VolumesApi.md#delete_volume) | **DELETE** /volumes/{name} | Delete a Volume
[**get_volume**](VolumesApi.md#get_volume) | **GET** /volumes/{name} | Get a Volume
[**list_volumes**](VolumesApi.md#list_volumes) | **GET** /volumes | List Volumes
[**update_volume**](VolumesApi.md#update_volume) | **PATCH** /volumes/{name} | Update a Volume



## create_volume

> models::VolumeInfo create_volume(create_volume_request_content)
Create a Volume

Creates a new volume. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_volume_request_content** | [**CreateVolumeRequestContent**](CreateVolumeRequestContent.md) |  | [required] |

### Return type

[**models::VolumeInfo**](VolumeInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume

> serde_json::Value delete_volume(name)
Delete a Volume

Deletes a volume from the specified parent catalog and schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The three-level (fully qualified) name of the volume | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume

> models::VolumeInfo get_volume(name)
Get a Volume

Gets a volume for a specific catalog and schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The three-level (fully qualified) name of the volume | [required] |

### Return type

[**models::VolumeInfo**](VolumeInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_volumes

> models::ListVolumesResponseContent list_volumes(catalog_name, schema_name, max_results, page_token)
List Volumes

Gets an array of available volumes under the parent catalog and schema. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | The identifier of the catalog | [required] |
**schema_name** | **String** | The identifier of the schema | [required] |
**max_results** | Option<**i32**> | Maximum number of volumes to return (page length).  If not set, the page length is set to a server configured value. - when set to a value greater than 0, the page length is the minimum of this value and a server configured value; - when set to 0, the page length is set to a server configured value; - when set to a value less than 0, an invalid parameter error is returned;  Note: this parameter controls only the maximum number of volumes to return. The actual number of volumes returned in a page may be smaller than this value, including 0, even if there are more pages.   |  |
**page_token** | Option<**String**> | Opaque token returned by a previous request. It must be included in the request to retrieve the next page of results (pagination). |  |

### Return type

[**models::ListVolumesResponseContent**](ListVolumesResponseContent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_volume

> models::VolumeInfo update_volume(name, update_volume_request_content)
Update a Volume

Updates the specified volume under the specified parent catalog and schema.  Currently only the name or the comment of the volume could be updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The three-level (fully qualified) name of the volume | [required] |
**update_volume_request_content** | Option<[**UpdateVolumeRequestContent**](UpdateVolumeRequestContent.md)> |  |  |

### Return type

[**models::VolumeInfo**](VolumeInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

