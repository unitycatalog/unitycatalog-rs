# \TemporaryTableCredentialsApi

All URIs are relative to *http://localhost:8080/api/2.1/unity-catalog*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_temporary_table_credentials**](TemporaryTableCredentialsApi.md#generate_temporary_table_credentials) | **POST** /temporary-table-credentials | Generate temporary table credentials.



## generate_temporary_table_credentials

> models::GenerateTemporaryTableCredentialResponse generate_temporary_table_credentials(generate_temporary_table_credential)
Generate temporary table credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_temporary_table_credential** | Option<[**GenerateTemporaryTableCredential**](GenerateTemporaryTableCredential.md)> |  |  |

### Return type

[**models::GenerateTemporaryTableCredentialResponse**](GenerateTemporaryTableCredentialResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

