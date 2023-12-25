use std::path::PathBuf;

use clap::Args;

use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct DownloadArgs {
    #[arg(help = "CID", required = true)]
    pub cid: String,

    #[arg(help = "Output file path", last = true)]
    pub output: PathBuf,
}

impl DownloadArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        println!("Downloading...");
        let paths = context.client.download(&self.cid, &self.output).await?;

        for path in paths {
            println!("Downloaded to {}", path.display());
        }

        Ok(())
    }
}
