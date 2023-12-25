use clap::Args;
use nft_storage_core::NftStorageApi as _;

use crate::{AppContext, Result};

#[derive(Args, Debug)]
pub struct UserDidPostArgs {
    #[arg(help = "DID")]
    pub did: Option<String>,
}

impl UserDidPostArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let req = nft_storage_api::models::UserDidPostRequest {
            did: self.did.clone(),
        };
        let response = context.client.user_did_post(req).await?;
        println!("{}", serde_json::to_string_pretty(&response)?);
        Ok(())
    }
}
