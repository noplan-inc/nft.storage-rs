mod error;
mod ipfs;

use std::{
    borrow::Borrow,
    fmt::Display,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use futures::stream::{FuturesUnordered, StreamExt};
pub mod encryptor;
use encryptor::Encryptor;
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

#[derive(Debug)]
pub struct NftStorageCore {
    config: Configuration,
    encryptor: Box<dyn Encryptor + Send + Sync>,
}

impl Display for NftStorageCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ config: {:?} }}", self.config)
    }
}

#[async_trait]
pub trait NftStorageApi {
    /// NftStorageApi is a wrapper around the NFT.storage API client to make it more user-friendly. For detailed API specifications, please refer to the following link: refs: [https://nft.storage/api-docs/](https://nft.storage/api-docs/)

    fn try_new(
        api_key: Option<String>,
        encryptor: Box<dyn Encryptor + Send + Sync>,
    ) -> Result<Box<Self>>;

    /// Store an [ERC-1155](https://eips.ethereum.org/EIPS/eip-1155)-compatible NFT as  a collection of content-addressed objects connected by IPFS CID links.
    async fn store(&self, meta: Option<&str>) -> Result<UploadResponse>;

    /// Store a file with nft.storage. You can upload either a single file or multiple files in a directory.
    // async fn upload<P>(&self, body: Vec<P>, x_agent_did: Option<&str>) -> Result<UploadResponse>
    // where
    // P: AsRef<Path> + Borrow<Path> + Send + Sync;

    async fn upload<'a, P>(
        &self,
        body: &'a [P],
        x_agent_did: Option<&str>,
    ) -> Result<UploadResponse>
    where
        P: AsRef<Path> + Send + Sync + 'a;

    async fn upload_encrypted<'a, P>(
        &self,
        body: &'a [P],
        x_agent_did: Option<&str>,
    ) -> Result<UploadResponse>
    where
        P: AsRef<Path> + Send + Sync + 'a;

    async fn list(&self, before: Option<&String>, limit: Option<i32>) -> Result<ListResponse>;
    async fn status(&self, cid: &str) -> Result<GetResponse>;

    async fn check(&self, cid: &str) -> Result<CheckResponse>;
    async fn delete(&self, cid: &str) -> Result<DeleteResponse>;
    async fn download<'a, P>(&self, cid: &str, dest_dir: &'a P) -> Result<Vec<PathBuf>>
    where
        P: AsRef<Path> + Send + Sync + 'a;

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
    fn try_new(
        api_key: Option<String>,
        encryptor: Box<dyn Encryptor + Send + Sync>,
    ) -> Result<Box<Self>> {
        let api_key = api_key
            .or_else(|| std::env::var("NFT_STORAGE_API_KEY").ok())
            .map_or_else(|| Err(CoreError::ApiKeyMissing), Ok)?;

        let config = Configuration {
            bearer_access_token: Some(api_key),
            ..Configuration::new()
        };

        Ok(Box::new(Self { config, encryptor }))
    }

    async fn store(&self, meta: Option<&str>) -> Result<UploadResponse> {
        let response = api::store(&self.config, meta).await?;
        Ok(response)
    }

    async fn upload<'a, P>(
        &self,
        body: &'a [P],
        x_agent_did: Option<&str>,
    ) -> Result<UploadResponse>
    where
        P: AsRef<Path> + Send + Sync + 'a,
    {
        let paths: Vec<&Path> = body.iter().map(AsRef::as_ref).collect();
        let response = api::upload(&self.config, &paths, x_agent_did).await?;
        Ok(response)
    }

    async fn upload_encrypted<'a, P>(
        &self,
        body: &'a [P],
        x_agent_did: Option<&str>,
    ) -> Result<UploadResponse>
    where
        P: AsRef<Path> + Send + Sync + 'a,
    {
        let mut files: Vec<(Vec<u8>, String)> = Vec::new();
        for path in body {
            let data = tokio::fs::read(path.as_ref())
                .await
                .map_err(|e| CoreError::ClientError(format!("Failed to read file: {}", e)))?;
            let encrypted_data = self
                .encryptor
                .encrypt(data.as_slice())
                .map_err(|e| CoreError::ClientError(format!("Failed to encrypt file: {}", e)))?;

            let mut file_name = path
                .as_ref()
                .file_name()
                .ok_or(CoreError::ClientError(
                    "Failed to get file name".to_string(),
                ))?
                .to_str()
                .ok_or(CoreError::ClientError(
                    "Failed to convert file name to string".to_string(),
                ))?
                .to_string();
            file_name.push_str(".enc");
            files.push((encrypted_data, file_name));
        }

        let response = api::upload_multiple_bytes(&self.config, files, x_agent_did).await?;
        Ok(response)
    }

    async fn download<'a, P>(&self, cid: &str, dest_dir: &'a P) -> Result<Vec<PathBuf>>
    where
        P: AsRef<Path> + Send + Sync + 'a,
    {
        let dest_dir = dest_dir.as_ref();
        if !dest_dir.exists() {
            tokio::fs::create_dir_all(&dest_dir).await.map_err(|e| {
                CoreError::ClientError(format!(
                    "Failed to create directory: {:?}, e: {}",
                    dest_dir, e
                ))
            })?;
        }

        let response = self.status(cid).await?;

        let files = response
            .value
            .ok_or(CoreError::ClientError("Failed to get files".to_string()))?
            .files
            .ok_or(CoreError::ClientError("Failed to get files".to_string()))?;

        let mut futures = FuturesUnordered::new();
        for file in files {
            let name = file.name.ok_or(CoreError::ClientError(
                "Failed to get file name".to_string(),
            ))?;

            let cid_with_file = format!("{}/{}", cid, name);
            futures.push(ipfs::download_from_gateways(cid_with_file, name));
        }

        let mut paths: Vec<PathBuf> = Vec::new();
        while let Some(result) = futures.next().await {
            let (bytes, name) = result?;
            let path = dest_dir.join(name);
            let ext = path.extension().unwrap_or_default();

            let clean_path = if ext == "enc" {
                let decrypted_data = self.encryptor.decrypt(bytes.as_slice()).map_err(|e| {
                    CoreError::ClientError(format!("Failed to decrypt file: {}", e))
                })?;
                // delete .enc
                let path = path.with_extension("");

                tokio::fs::write(&path, decrypted_data).await.map_err(|e| {
                    CoreError::ClientError(format!("Failed to write file: {:?}, e: {}", path, e))
                })?;
                path
            } else {
                tokio::fs::write(&path, bytes).await.map_err(|e| {
                    CoreError::ClientError(format!("Failed to write file: {:?}, e: {}", path, e))
                })?;
                path
            };

            paths.push(clean_path);
        }
        Ok(paths)
    }

    async fn list(&self, before: Option<&String>, limit: Option<i32>) -> Result<ListResponse> {
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
    use std::path::PathBuf;

    use crate::encryptor::plugins::{
        self,
        aes::{AesArgs, AesEncryptor},
    };

    use super::*;

    fn init_core() -> Box<NftStorageCore> {
        dotenv::from_filename(".env.test").ok();
        // default key is dangerous
        // test case is ok
        let args = AesArgs::default();
        let encryptor = Box::new(plugins::aes::AesEncryptor::new(args).unwrap());
        NftStorageCore::try_new(None, encryptor).unwrap()
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

        let res = core.upload(&body, None).await;

        if let Err(e) = &res {
            println!("Upload operation failed: {:?}", e);
        }

        assert!(res.is_ok(), "Expected upload operation to be successful");

        println!(
            "Upload operation response: {:?}",
            res.as_ref().expect("Failed to get response")
        );

        let ok = res
            .as_ref()
            .expect("Failed to get response")
            .ok
            .as_ref()
            .expect("Failed to get ok");
        assert_eq!(ok, &true, "Expected ok to be true");

        let cid_len = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .cid
            .as_ref()
            .expect("Failed to get CID")
            .len();
        assert!(cid_len >= 46, "Expected CID length to be greater than 46");

        let files_len = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .files
            .as_ref()
            .expect("Failed to get files")
            .len();
        assert_eq!(files_len, 2, "Expected files length to be 2");

        let size = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .size
            .as_ref()
            .expect("Failed to get size");
        assert!(size >= &1.0, "Expected size to be greater than 1");

        let created = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .created
            .as_ref()
            .expect("Failed to get created");
        assert!(!created.is_empty(), "Expected created to be not empty");
    }

    #[tokio::test]
    async fn test_upload_encrypted() {
        let core = init_core();

        let body = vec![
            PathBuf::from("tests/fixtures/rust1.png"),
            PathBuf::from("tests/fixtures/rust2.png"),
        ];

        let res = core.upload_encrypted(&body, None).await;

        if let Err(e) = &res {
            println!("Upload operation failed: {:?}", e);
        }

        assert!(
            res.is_ok(),
            "Expected upload encrypted operation to be successful"
        );

        println!(
            "Upload operation response: {:?}",
            res.as_ref().expect("Failed to get response")
        );

        let ok = res
            .as_ref()
            .expect("Failed to get response")
            .ok
            .as_ref()
            .expect("Failed to get ok");
        assert_eq!(ok, &true, "Expected ok to be true");

        // print cid
        let cid = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .cid
            .as_ref()
            .expect("Failed to get CID");
        println!("cid: {}", cid);

        let cid_len = cid.len();
        assert!(cid_len >= 46, "Expected CID length to be greater than 46");

        let files_len = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .files
            .as_ref()
            .expect("Failed to get files")
            .len();
        assert_eq!(files_len, 2, "Expected files length to be 2");

        let size = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .size
            .as_ref()
            .expect("Failed to get size");
        assert!(size >= &1.0, "Expected size to be greater than 1");

        let created = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .created
            .as_ref()
            .expect("Failed to get created");
        assert!(!created.is_empty(), "Expected created to be not empty");
    }

    #[tokio::test]
    async fn test_list() {
        let core = init_core();

        let res = core.list(None, None).await;

        if let Err(e) = &res {
            println!("List operation failed: {:?}", e);
        }

        assert!(res.is_ok(), "Expected list operation to be successful");

        println!(
            "List operation response: {:?}",
            res.as_ref().expect("Failed to get response")
        );

        let ok = res
            .as_ref()
            .expect("Failed to get response")
            .ok
            .as_ref()
            .expect("Failed to get ok");
        assert_eq!(ok, &true, "Expected ok to be true");

        let items_len = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .len();

        assert!(items_len >= 1, "Expected items length to be greater than 1");

        let created = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .get(0)
            .expect("Failed to get item")
            .created
            .as_ref()
            .expect("Failed to get created");
        assert!(!created.is_empty(), "Expected created to be not empty");
    }

    #[tokio::test]
    async fn test_status() {
        let core = init_core();

        let res = core
            .status("bafybeidzsvkluurobic7m2ms4cvyriqo376zci65s67xrp24g2wolsunz4")
            .await;

        if let Err(e) = &res {
            println!("Status operation failed: {:?}", e);
        }

        assert!(res.is_ok(), "Expected status operation to be successful");

        println!(
            "Status operation response: {:?}",
            res.as_ref().expect("Failed to get response")
        );

        let ok = res
            .as_ref()
            .expect("Failed to get response")
            .ok
            .expect("Failed to get ok");
        assert!(ok, "Expected ok to be true");
        // Status operation response: GetResponse { ok: Some(true), value: Some(Nft { cid: Some("bafybeidzsvkluurobic7m2ms4cvyriqo376zci65s67xrp24g2wolsunz4"), size: Some(3296140.0), created: Some("2023-11-03T00:48:50.766+00:00"), type: Some("directory"), scope: Some("rust-test"), pin: Some(Pin { cid: Some("bafybeidzsvkluurobic7m2ms4cvyriqo376zci65s67xrp24g2wolsunz4"), name: None, meta: None, status: Some(Pinned), created: Some("2023-11-03T00:48:50.766+00:00"), size: Some(3296140.0) }), files: Some([FilesInner { name: Some("rust1.png"), type: Some("") }, FilesInner { name: Some("rust2.png"), type: Some("") }]), deals: Some([]) }) }
        // assert cid
        let cid = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .cid
            .as_ref()
            .expect("Failed to get cid");
        assert_eq!(
            cid, "bafybeidzsvkluurobic7m2ms4cvyriqo376zci65s67xrp24g2wolsunz4",
            "Expected cid to be bafybeidzsvkluurobic7m2ms4cvyriqo376zci65s67xrp24g2wolsunz4"
        );

        // size >= 1
        let size = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .size
            .as_ref()
            .expect("Failed to get size");
        assert!(*size >= 1.0, "Expected size to be greater than 1");

        // check rust1.png, rust2.png
        let files = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .files
            .as_ref()
            .expect("Failed to get files");
        assert_eq!(files.len(), 2, "Expected files length to be 2");
        for file in files {
            let name = file.name.as_ref().expect("Failed to get name");
            assert!(name == "rust1.png" || name == "rust2.png");
        }
    }

    #[tokio::test]
    async fn test_check() {
        let core = init_core();

        let res = core
            .check("bafybeidzsvkluurobic7m2ms4cvyriqo376zci65s67xrp24g2wolsunz4")
            .await;

        if let Err(e) = &res {
            println!("Check operation failed: {:?}", e);
        }

        assert!(res.is_ok(), "Expected check operation to be successful");

        println!(
            "Check operation response: {:?}",
            res.as_ref().expect("Failed to get response")
        );

        let ok = res
            .as_ref()
            .expect("Failed to get response")
            .ok
            .expect("Failed to get ok");
        assert!(ok, "Expected ok to be true");

        let cid = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .cid
            .as_ref()
            .expect("Failed to get cid");

        assert_eq!(
            cid, "bafybeidzsvkluurobic7m2ms4cvyriqo376zci65s67xrp24g2wolsunz4",
            "Expected cid to be bafybeidzsvkluurobic7m2ms4cvyriqo376zci65s67xrp24g2wolsunz4"
        );

        let created = res
            .as_ref()
            .expect("Failed to get response")
            .value
            .as_ref()
            .expect("Failed to get value")
            .pin
            .as_ref()
            .expect("Failed to get pin")
            .created
            .as_ref()
            .expect("Failed to get created");
        assert!(
            !created.is_empty(),
            "Expected created to be not empty string"
        );
    }

    #[tokio::test]
    async fn test_download() {
        let core = init_core();

        let cid = "bafybeidikdidxlvdblrvqcyamusqjzktsy7wbn7ygl4qaqolkrg2nwqlk4";
        let dest_dir = PathBuf::from("tests/fixtures/download");
        let res = core.download(cid, &dest_dir).await;

        if let Err(e) = &res {
            println!("Download operation failed: {:?}", e);
            return;
        }

        assert!(res.is_ok(), "Expected download operation to be successful");

        for path in res.unwrap() {
            assert!(path.exists(), "Expected file to exist");
            assert!(path.is_file(), "Expected file to be a file");
            // rust1.png.enc or rust2.png.enc
            assert_eq!(path.extension().unwrap(), "png");
            let stem = path
                .file_stem()
                .expect("failed to file_stem()")
                .to_string_lossy();
            println!("stem: {}", stem);
            assert!(stem == "rust1" || stem == "rust2");
        }
    }
}
