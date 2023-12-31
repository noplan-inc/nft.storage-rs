# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.nft.storage*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*NftStorageApi* | [**check**](docs/NftStorageApi.md#check) | **GET** /check/{cid} | Check if a CID of an NFT is being stored by nft.storage.
*NftStorageApi* | [**delete**](docs/NftStorageApi.md#delete) | **DELETE** /{cid} | Stop storing the content with the passed CID
*NftStorageApi* | [**did_get**](docs/NftStorageApi.md#did_get) | **GET** /did | Get nft.storage DID
*NftStorageApi* | [**list**](docs/NftStorageApi.md#list) | **GET** / | List all stored files
*NftStorageApi* | [**status**](docs/NftStorageApi.md#status) | **GET** /{cid} | Get information for the stored file CID
*NftStorageApi* | [**store**](docs/NftStorageApi.md#store) | **POST** /store | Store an ERC-1155 compatible NFT
*NftStorageApi* | [**ucan_token_post**](docs/NftStorageApi.md#ucan_token_post) | **POST** /ucan/token | Get a root UCAN.
*NftStorageApi* | [**upload**](docs/NftStorageApi.md#upload) | **POST** /upload | Store a file
*NftStorageApi* | [**user_did_post**](docs/NftStorageApi.md#user_did_post) | **POST** /user/did | Register a DID for a user.


## Documentation For Models

 - [CheckResponse](docs/CheckResponse.md)
 - [CheckResponseValue](docs/CheckResponseValue.md)
 - [Deal](docs/Deal.md)
 - [DeleteResponse](docs/DeleteResponse.md)
 - [DidGet200Response](docs/DidGet200Response.md)
 - [DidNotFoundErrorResponse](docs/DidNotFoundErrorResponse.md)
 - [DidNotFoundErrorResponseError](docs/DidNotFoundErrorResponseError.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ErrorResponseError](docs/ErrorResponseError.md)
 - [FilesInner](docs/FilesInner.md)
 - [ForbiddenErrorResponse](docs/ForbiddenErrorResponse.md)
 - [ForbiddenErrorResponseError](docs/ForbiddenErrorResponseError.md)
 - [GetResponse](docs/GetResponse.md)
 - [Links](docs/Links.md)
 - [LinksFileInner](docs/LinksFileInner.md)
 - [ListResponse](docs/ListResponse.md)
 - [Nft](docs/Nft.md)
 - [Pin](docs/Pin.md)
 - [PinStatus](docs/PinStatus.md)
 - [UcanTokenPost200Response](docs/UcanTokenPost200Response.md)
 - [UnauthorizedErrorResponse](docs/UnauthorizedErrorResponse.md)
 - [UnauthorizedErrorResponseError](docs/UnauthorizedErrorResponseError.md)
 - [UploadResponse](docs/UploadResponse.md)
 - [UserDidPostRequest](docs/UserDidPostRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



