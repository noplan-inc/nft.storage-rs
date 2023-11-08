use clap::Args;
use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct UcanTokenPostArgs {}

impl UcanTokenPostArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let response = context.client.ucan_token_post().await?;
        println!("{}", serde_json::to_string_pretty(&response)?);
        Ok(())
    }
}
