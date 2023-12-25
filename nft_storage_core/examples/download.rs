use std::path::PathBuf;

use nft_storage_core::{
    encryptor::plugins::aes::AesEncryptor,
    NftStorageApi, NftStorageCore,
};

#[tokio::main]
async fn main() {
    let aes = AesEncryptor::generate_key().unwrap();

    let encryptor = Box::new(aes);
    let client = NftStorageCore::try_new(None, encryptor).unwrap();


    // CID only works with what nft.storage manages for now.
    // In the future, any IPFS CID will work.
    let cid = "<CID>>";
    let dest_dir = PathBuf::from("./");
    let resp = client.download(cid, &dest_dir).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
