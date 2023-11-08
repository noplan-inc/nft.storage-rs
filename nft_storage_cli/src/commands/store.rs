use clap::Args;
use nft_storage_core::NftStorageApi as _;
use tokio::io::AsyncReadExt as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct StoreArgs {
    #[arg(
        help = "Json String to upload. ex) {\"name\": \"kitty\", \"description\": \"pretty kitty\"}"
    )]
    pub json: Option<String>,
}

impl StoreArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let json_str: &str;
        let mut buffer = String::new();

        if let Some(json) = &self.json {
            json_str = json;
        } else {
            println!("Enter json string to upload or cat json file: ex) cat metadata.json | nft-storage-cli store");
            println!("(Press Ctrl+D to finish)");
            tokio::io::stdin().read_to_string(&mut buffer).await?;
            if buffer.is_empty() {
                return Err(anyhow::anyhow!("Empty json string"));
            }
            json_str = &buffer;
        };

        println!("Uploading...");
        let resp = context.client.store(Some(json_str)).await?;
        println!("{}", serde_json::to_string_pretty(&resp)?);

        Ok(())
    }
}
