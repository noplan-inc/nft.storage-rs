mod error;

use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use error::CoreError;
use nft_storage_api::{
    apis::configuration::Configuration,
    models::{
        CheckResponse, DeleteResponse, DidGet200Response, GetResponse, ListResponse,
        UcanTokenPost200Response, UploadResponse, UserDidPostRequest,
    },
};

use nft_storage_api::apis::nft_storage_api as api;

pub type Result<T> = std::result::Result<T, CoreError>;

pub struct NftStorageCore {
    config: Configuration,
}

#[async_trait]
pub trait NftStorageApi {
    /// NftStorageApi is a wrapper around the NFT.storage API client to make it more user-friendly. For detailed API specifications, please refer to the following link: refs: [https://nft.storage/api-docs/](https://nft.storage/api-docs/)

    fn try_new(api_key: Option<String>) -> Result<NftStorageCore>;

    /// Store an [ERC-1155](https://eips.ethereum.org/EIPS/eip-1155)-compatible NFT as  a collection of content-addressed objects connected by IPFS CID links.
    async fn store(&self, meta: Option<&str>) -> Result<UploadResponse>;

    /// Store a file with nft.storage. You can upload either a single file or multiple files in a directory.
    async fn upload<P>(&self, body: Vec<P>, x_agent_did: Option<&str>) -> Result<UploadResponse>
    where
        P: AsRef<Path> + Borrow<Path> + Send + Sync;
    async fn list(&self, before: Option<String>, limit: Option<i32>) -> Result<ListResponse>;
    async fn status(&self, cid: &str) -> Result<GetResponse>;

    async fn check(&self, cid: &str) -> Result<CheckResponse>;
    async fn delete(&self, cid: &str) -> Result<DeleteResponse>;

    // DID
    async fn did_get(&self) -> Result<DidGet200Response>;
    async fn user_did_post(
        &self,
        user_did_post_request: UserDidPostRequest,
    ) -> Result<DidGet200Response>;
    async fn ucan_token_post(&self) -> Result<UcanTokenPost200Response>;
}

#[async_trait]
impl NftStorageApi for NftStorageCore {
    fn try_new(api_key: Option<String>) -> Result<Self> {
        let api_key = api_key
            .or_else(|| std::env::var("NFT_STORAGE_API_KEY").ok())
            .map_or_else(|| Err(CoreError::ApiKeyMissing), Ok)?;

        let config = Configuration {
            bearer_access_token: Some(api_key),
            ..Configuration::new()
        };

        Ok(Self { config })
    }

    async fn store(&self, meta: Option<&str>) -> Result<UploadResponse> {
        let response = api::store(&self.config, meta).await?;
        Ok(response)
    }

    async fn upload<P>(&self, body: Vec<P>, x_agent_did: Option<&str>) -> Result<UploadResponse>
    where
        P: AsRef<Path> + Borrow<Path> + Send + Sync,
    {
        let response = api::upload(&self.config, body, x_agent_did).await?;
        Ok(response)
    }

    async fn list(&self, before: Option<String>, limit: Option<i32>) -> Result<ListResponse> {
        let response = api::list(&self.config, before, limit).await?;
        Ok(response)
    }

    async fn status(&self, cid: &str) -> Result<GetResponse> {
        let response = api::status(&self.config, cid).await?;
        Ok(response)
    }

    async fn check(&self, cid: &str) -> Result<CheckResponse> {
        let response = api::check(&self.config, cid).await?;
        Ok(response)
    }

    async fn delete(&self, cid: &str) -> Result<DeleteResponse> {
        let response = api::delete(&self.config, cid).await?;
        Ok(response)
    }

    async fn did_get(&self) -> Result<DidGet200Response> {
        let response = api::did_get(&self.config).await?;
        Ok(response)
    }

    async fn user_did_post(
        &self,
        user_did_post_request: UserDidPostRequest,
    ) -> Result<DidGet200Response> {
        let response = api::user_did_post(&self.config, user_did_post_request).await?;
        Ok(response)
    }

    async fn ucan_token_post(&self) -> Result<UcanTokenPost200Response> {
        let response = api::ucan_token_post(&self.config).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_core() -> NftStorageCore {
        dotenv::from_filename(".env.test").ok();
        NftStorageCore::try_new(None).unwrap()
    }

    #[tokio::test]
    async fn test_store() {
        let core = init_core();

        let str_meta = Some(
            r#"{
        "name": "Hello",
        "description": "Hello World"
      }"#,
        );

        let res = core.store(str_meta).await;

        if let Err(e) = &res {
            println!("Store operation failed: {:?}", e);
        }

        assert!(res.is_ok(), "Expected store operation to be successful");

        println!(
            "Store operation response: {:?}",
            res.as_ref().expect("Failed to get response")
        );

        // TODO
        // The response type is incorrect because the response definition in openapi is invalid;
        //  the API returns 200 but the response cannot be tested.

        // let cid_len = res
        //     .as_ref()
        //     .expect("Failed to get response")
        //     .value
        //     .as_ref()
        //     .expect("Failed to get value")
        //     .cid
        //     .as_ref()
        //     .expect("Failed to get CID")
        //     .len();

        // assert_eq!(cid_len, 46, "Expected CID length to be 46");
    }

    #[tokio::test]
    async fn test_upload() {
        let core = init_core();

        let body = vec![
            PathBuf::from("tests/fixtures/rust1.png"),
            PathBuf::from("tests/fixtures/rust2.png"),
        ];

        let res = core.upload(body, None).await;

        if let Err(e) = &res {
            println!("Upload operation failed: {:?}", e);
        }

        assert!(res.is_ok(), "Expected upload operation to be successful");

        println!(
            "Upload operation response: {:?}",
            res.as_ref().expect("Failed to get response")
        );
    }
}
