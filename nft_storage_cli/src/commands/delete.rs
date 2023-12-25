use clap::Args;

use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct DeleteArgs {
    #[arg(help = "CID", required = true)]
    pub cid: String,

    #[clap(short, long, default_value = "false", help = "yes or no")]
    pub yes: Option<bool>,
}

impl DeleteArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        if self.yes != Some(true) {
            println!("Are you sure you want to delete this CID? [y/N]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() != "y" {
                println!("Aborted");
                return Ok(());
            }
        }
        context.client.delete(&self.cid).await?;

        Ok(())
    }
}
