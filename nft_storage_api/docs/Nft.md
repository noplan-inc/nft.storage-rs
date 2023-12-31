# Nft

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cid** | Option<**String**> | Self-describing content-addressed identifiers for distributed systems. Check [spec](https://github.com/multiformats/cid) for more info. | [optional]
**size** | Option<**f32**> | Size in bytes of the NFT data. | [optional][default to 132614]
**created** | Option<**String**> | This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: YYYY-MM-DDTHH:MM:SSZ. | [optional]
**r#type** | Option<**String**> | MIME type of the uploaded file or 'directory' when uploading multiple files. | [optional]
**scope** | Option<**String**> | Name of the JWT token used to create this NFT. | [optional][default to default]
**pin** | Option<[**crate::models::Pin**](Pin.md)> |  | [optional]
**files** | Option<[**Vec<crate::models::FilesInner>**](Files_inner.md)> | Files in the directory (only if this NFT is a directory). | [optional]
**deals** | Option<[**Vec<crate::models::Deal>**](Deal.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


