use std::path::PathBuf;

use nft_storage_core::{encryptor::plugins::aes::AesEncryptor, NftStorageApi, NftStorageCore};

#[tokio::main]
async fn main() {
    let aes = AesEncryptor::generate_key().unwrap();

    let encryptor = Box::new(aes);
    // If pass none, use NFT_STORAGE_API_KEY envrioment variable.
    let api_key = Some("<FILL ME NFT_STORAGE_API_KEY>".to_string());
    let client = NftStorageCore::try_new(api_key, encryptor).unwrap();

    let path = vec![PathBuf::from("test.txt")];
    let resp = client.upload_encrypted(&path, None).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
