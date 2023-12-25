use std::path::PathBuf;

use nft_storage_core::{encryptor::plugins::aes::AesEncryptor, NftStorageApi, NftStorageCore};

#[tokio::main]
async fn main() {
    let aes = AesEncryptor::generate_key().unwrap();

    let encryptor = Box::new(aes);

    // If pass none, use NFT_STORAGE_API_KEY envrioment variable.
    let api_key = Some("<FILL ME NFT_STORAGE_API_KEY>".to_string());
    let client = NftStorageCore::try_new(api_key, encryptor).unwrap();

    // CID only works with what nft.storage manages for now.
    // In the future, any IPFS CID will work.
    let cid = "<CID>>";
    let dest_dir = PathBuf::from("./");
    let resp = client.download(cid, &dest_dir).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
