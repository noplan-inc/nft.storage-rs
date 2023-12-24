use anyhow::bail;
use clap::Args;

use crate::{
    config::{write_config, Config},
    AppContext, Result,
};

#[derive(Args, Debug)]
pub struct InitArgs {}

impl InitArgs {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let key = match context.encryptor.generate_key() {
            Ok(key) => key,
            Err(e) => {
                bail!("Failed to generate key: {}", e);
            }
        };

        let config = Config {
            encrypt_method: Some(context.encrypt_method.clone()),
            encrypt_private_key: Some(key.clone()),
            api_key: Some(context.api_key.clone()),
            verbose: Some(context.verbose),
        };

        let config_path = write_config(&config, None).await?;
        println!("Config written to {}", config_path.display());

        Ok(())
    }
}
