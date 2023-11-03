# \NftStorageApi

All URIs are relative to *https://api.nft.storage*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check**](NftStorageApi.md#check) | **GET** /check/{cid} | Check if a CID of an NFT is being stored by nft.storage.
[**delete**](NftStorageApi.md#delete) | **DELETE** /{cid} | Stop storing the content with the passed CID
[**did_get**](NftStorageApi.md#did_get) | **GET** /did | Get nft.storage DID
[**list**](NftStorageApi.md#list) | **GET** / | List all stored files
[**status**](NftStorageApi.md#status) | **GET** /{cid} | Get information for the stored file CID
[**store**](NftStorageApi.md#store) | **POST** /store | Store an ERC-1155 compatible NFT
[**ucan_token_post**](NftStorageApi.md#ucan_token_post) | **POST** /ucan/token | Get a root UCAN.
[**upload**](NftStorageApi.md#upload) | **POST** /upload | Store a file
[**user_did_post**](NftStorageApi.md#user_did_post) | **POST** /user/did | Register a DID for a user.



## check

> crate::models::CheckResponse check(cid)
Check if a CID of an NFT is being stored by nft.storage.

Includes the IPFS pinning state and the Filecoin deal state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | CID for the NFT | [required] |

### Return type

[**crate::models::CheckResponse**](CheckResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete

> crate::models::DeleteResponse delete(cid)
Stop storing the content with the passed CID

Stop storing the content with the passed CID on nft.storage. - Unpin the item from the underlying IPFS pinning service. - Cease renewals for expired Filecoin deals involving the CID.    ⚠️ This does not remove the content from the network.  - Does not terminate any established Filecoin deal. - Does not remove the content from other IPFS nodes in the network that already cached or pinned the CID.    Note: the content will remain available if another user has stored the CID with nft.storage. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | CID for the NFT | [required] |

### Return type

[**crate::models::DeleteResponse**](DeleteResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## did_get

> crate::models::DidGet200Response did_get()
Get nft.storage DID

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DidGet200Response**](_did_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::ListResponse list(before, limit)
List all stored files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | Return results created before provided timestamp |  |
**limit** | Option<**i32**> | Max records to return |  |[default to 10]

### Return type

[**crate::models::ListResponse**](ListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status

> crate::models::GetResponse status(cid)
Get information for the stored file CID

Includes the IPFS pinning state and the Filecoin deal state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | CID for the NFT | [required] |

### Return type

[**crate::models::GetResponse**](GetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store

> crate::models::UploadResponse store(meta)
Store an ERC-1155 compatible NFT

Store an [ERC-1155](https://eips.ethereum.org/EIPS/eip-1155)-compatible NFT as  a collection of content-addressed objects connected by IPFS CID links.  **Not recommended for performance sensitive applications**. While the `/store` endpoint is convenient, it is less performant than `/upload`, and might be deprecated in the future. If you would like to have a similar level of convenience with better performance, check out the Javascript client's [`store` method](https://nft.storage/docs/client/js/#store---store-erc1155-nft-data). We recommend constructing CAR files on the client wherever possible and using the `/upload` endpoint to store your data. This lets you compute the CID for your upload on the client and verify it against what the API returns.  The POST request accepts `multipart/form-data` content restricted to a body size of 100MB (see \"Size limitations\" below for more information). The request must contain a form field named `meta`.  The `meta` field must contain a JSON string that conforms to the [ERC-1155 metadata schema](https://eips.ethereum.org/EIPS/eip-1155#metadata).  Any field(s) inside the `meta` object can be substituted with an ipfs URL to a file(s), by providing a form data field with a name matching a (`.` delimited) property path and value containing the file content (in binary string or plain text depending on file format).  The name of the form data field containing the file content should be the \"path\" of the JSON field, using `.` to traverse nested objects.  For example, with a `meta` object of the form:    ```json   {     \"name\": \"Hello\",     \"image\": undefined,     \"properties\": {       \"videoClip\": undefined     }   }   ```  You must include form fields named `image` and `properties.videoClip` in your request body, with the content of the image and video files as the form field values.  All form fields other than `meta` must contain binary file data, and the field name will be used as a '.' delimited property path for the `meta` object, as described above. If the form field name matches a `meta` property with a non-falsy value, the request will be rejected with an error.  ### Mime Type Recommendations  The ERC-1155 metadata standard specifies that the `image` metadata field should reference a file with a content type of `image/_*`. An earlier version of this API enforced this as a requirement, but this proved to be incompatible with some existing systems and the requirement was relaxed, although you may see a warning when using the official JavaScript client.  We highly recommend that you only use content with an `image/_*` content type for your `image` field, and include other content types in the `properties` field as additional references.  ### Size limitations  The store endpoint is restricted to a total request body size of 100MB, which includes the metadata and all attached files. To store larger files, you can use the /upload endpoint with chunked CAR files (see \"/upload\").  ### Rate limits  This API imposes rate limits to ensure quality of service. You may receive a 429 \"Too many requests\" error if you make more than 30 requests with the same API token within a 10 second window. Upon receiving a response with a 429 status, clients should retry the failed request after a small delay. To avoid 429 responses, you may wish to implement client-side request throttling to stay within the limits (note: the JS client automatically does this). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> |  |  |

### Return type

[**crate::models::UploadResponse**](UploadResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ucan_token_post

> crate::models::UcanTokenPost200Response ucan_token_post()
Get a root UCAN.

This endpoint returns a root UCAN for nft.storage. The UCAN will be valid for **two weeks** and has full capabilities. The JWT payload will looking similar to this:    ```json {   \"att\": [     {       \"with\": \"storage://did:key:z6MkheUPoHhYRS5LoHbbttpaTkkxvFFFUV5VPSziwTJmbb7D\",       \"can\": \"upload/_*\"     }   ],   \"exp\": 1646668606,   \"iss\": \"did:key:z6MkheUPoHhYRS5LoHbbttpaTkkxvFFFUV5VPSziwTJmbb7D\",   \"prf\": [] } ```  A valid UCAN can be used for authorization in this endpoint. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UcanTokenPost200Response**](_ucan_token_post_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload

> crate::models::UploadResponse upload(body, x_agent_did)
Store a file

Store a file with nft.storage. You can upload either a single file or multiple files in a directory. Each request to /upload is restricted to a body size of 100MB, though this does not mean you cannot upload larger files (see \"Size limitations\" below).  ### Single file Send the POST request with `Blob`/`File` data as the request body.  ### Multiple files Send the POST request as `FormData` with the header `Content-Type: multipart/form-data` set. Each part should have a `Content-Disposition` header to specify \"name\" (which must be \"file\") and \"filename\". e.g.  ``` ------WebKitFormBoundary5peilISl2YOOweQy Content-Disposition: form-data; name=\"file\"; filename=\"image.png\" Content-Type: image/png  <data> ------WebKitFormBoundary5peilISl2YOOweQy-- ```  ### Content Addressed Archive (CAR) files You can also upload a CAR file, by setting the request body as a single CAR Blob/File object and providing the request header `Content-Type: application/car` Providing a CAR file allows you to pre-compute the root CID for 1 or more files, ensures that NFT.Storage will store and provide your assets with the same CID.  ### Size limitations Each request to the upload endpoint is limited to a total request body size of 100MB. However, you can upload files larger than 100MB by packing them into a CAR file and splitting the CAR into chunks of less than 100MB. This strategy is used by the JavaScript client library to support uploads of large files.  The simplest method of splitting CAR files is with the [carbites cli tool](https://github.com/nftstorage/carbites):  ``` npm i -g carbites-cli  # Split a big CAR into many smaller CARs carbites split big.car --size 100MB --strategy treewalk ```  Note that you MUST use the `treewalk` strategy, so that all the chunked CARs have the same root CID. Once all the CAR chunks have been uploaded, the CARs will be combined, made available via IPFS, and queued for storage on Filecoin.  For more about working with CARs, see https://docs.web3.storage/how-tos/work-with-car-files  ### Rate limits  This API imposes rate limits to ensure quality of service. You may receive a 429 \"Too many requests\" error if you make more than 30 requests with the same API token within a ten second window. Upon receiving a response with a 429 status, clients should retry the failed request after a small delay. To avoid 429 responses, you may wish to implement client-side request throttling to stay within the limits (note: the JS client automatically does this). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **std::path::PathBuf** |  | [required] |
**x_agent_did** | Option<**String**> | DID that issued / signed UCAN authorization token (required if UCAN token is used) |  |

### Return type

[**crate::models::UploadResponse**](UploadResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: image/*, application/car, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_did_post

> crate::models::DidGet200Response user_did_post(user_did_post_request)
Register a DID for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_did_post_request** | [**UserDidPostRequest**](UserDidPostRequest.md) |  | [required] |

### Return type

[**crate::models::DidGet200Response**](_did_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

