use anyhow::bail;
use clap::Args;

use crate::{
    config::{write_config, Config},
    AppContext, Result,
};

#[derive(Args, Debug)]
pub struct InitArgs {
    api_key: Option<String>,
}

impl InitArgs {
    // pub async fn execute(&self, encryptor: Box<dyn Encryptor + Send + Sync>, _api_key: Option<String>, encrypt_method: String, verbose: bool) -> Result<()> {
    pub async fn execute(&self, context: &AppContext) -> Result<()> {
        let AppContext {
            client: _,
            encryptor,
            encrypt_method,
            verbose,
            api_key: _api_key,
        } = context;

        let key = match encryptor.generate_key() {
            Ok(key) => key,
            Err(e) => {
                bail!("Failed to generate key: {}", e);
            }
        };

        let api_key = Some(self.api_key.clone().unwrap_or_else(|| _api_key.to_string()));

        let config = Config {
            api_key,
            encrypt_method: Some(encrypt_method.clone()),
            encrypt_private_key: Some(key.clone()),
            verbose: Some(verbose.to_owned()),
        };

        let config_path = write_config(&config, None).await?;
        println!("Config written to {:?}", config_path);

        Ok(())
    }
}
