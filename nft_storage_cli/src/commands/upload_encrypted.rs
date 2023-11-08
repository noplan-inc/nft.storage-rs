// upload.rsを参考に、upload_encrypted.rsを作成する。

use std::path::PathBuf;

use clap::Args;
use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct UploadEncryptedArgs {
    #[arg(help = "File to upload path", value_delimiter = ',', required = true)]
    pub file: Vec<PathBuf>,

    #[clap(short, long)]
    pub x_agent_did: Option<String>,
}

impl UploadEncryptedArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        context
            .client
            .upload_encrypted(&self.file, self.x_agent_did.as_deref())
            .await?;

        Ok(())
    }
}
