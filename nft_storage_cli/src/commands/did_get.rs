use clap::Args;
use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct DidGetArgs {}

impl DidGetArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let response = context.client.did_get().await?;
        println!("{}", serde_json::to_string_pretty(&response)?);
        Ok(())
    }
}
