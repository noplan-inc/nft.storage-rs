use clap::Args;
use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct CheckArgs {
    #[arg(help = "CID", required = true)]
    pub cid: String,
}

impl CheckArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let status = context.client.check(&self.cid).await?;
        println!("{}", serde_json::to_string_pretty(&status)?);
        Ok(())
    }
}
