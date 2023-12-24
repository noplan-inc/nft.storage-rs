use std::path::PathBuf;

use clap::Args;
use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct UploadArgs {
    #[arg(help = "File to upload path", value_delimiter = ',', required = true)]
    pub file: Vec<PathBuf>,

    #[clap(short, long)]
    pub x_agent_did: Option<String>,
}

impl UploadArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let resp = context
            .client
            .upload(&self.file, self.x_agent_did.as_deref())
            .await?;

        println!("Uploaded to IPFS: {}", resp.value.unwrap().cid.unwrap());

        Ok(())
    }
}
