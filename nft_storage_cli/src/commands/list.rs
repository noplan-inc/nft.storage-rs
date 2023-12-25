use clap::Args;
use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(help = "before for pagination")]
    pub before: Option<String>,

    #[arg(help = "limit for pagination")]
    pub limit: Option<i32>,
}

impl ListArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let status = context
            .client
            .list(self.before.as_ref(), self.limit)
            .await?;
        println!("{}", serde_json::to_string_pretty(&status)?);
        Ok(())
    }
}
