use std::path::PathBuf;

use nft_storage_core::{encryptor::plugins::aes::AesEncryptor, NftStorageApi, NftStorageCore};

#[tokio::main]
async fn main() {
    let aes = AesEncryptor::generate_key().unwrap();

    let encryptor = Box::new(aes);
    let client = NftStorageCore::try_new(None, encryptor).unwrap();

    let path = vec![PathBuf::from("test.txt")];
    let resp = client.upload(&path, None).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
