# VolumeInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**catalog_name** | Option<**String**> | The name of the catalog where the schema and the volume are | [optional]
**schema_name** | Option<**String**> | The name of the schema where the volume is | [optional]
**name** | Option<**String**> | The name of the volume | [optional]
**comment** | Option<**String**> | The comment attached to the volume | [optional]
**created_at** | Option<**i64**> | Time at which this volume was created, in epoch milliseconds. | [optional]
**updated_at** | Option<**i64**> | Time at which this volume was last modified, in epoch milliseconds. | [optional]
**volume_id** | Option<**String**> | Unique identifier for the volume | [optional]
**volume_type** | Option<[**models::VolumeType**](VolumeType.md)> |  | [optional]
**storage_location** | Option<**String**> | The storage location of the volume | [optional]
**full_name** | Option<**String**> | Full name of volume, in form of __catalog_name__.__schema_name__.__volume_name__. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


