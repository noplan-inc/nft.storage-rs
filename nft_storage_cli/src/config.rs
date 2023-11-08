use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub encrypt_method: String,
    pub encrypt_private_key: Vec<u8>,
    pub api_key: String,
}

fn get_config_path() -> PathBuf {
    let mut config_dir = ProjectDirs::from("com", "noplan-inc", "nft-storage")
        .expect("no home directory")
        .config_dir()
        .to_path_buf();

    config_dir.push("config.toml");
    config_dir
}

pub async fn load_config() -> Result<Config> {
    let config_path = get_config_path();
    let config_str = fs::read_to_string(config_path).await?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

pub async fn write_config(config: &Config) -> Result<PathBuf> {
    let config_path = get_config_path();
    // not exist dir
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).await?;
        }
    }

    let config_str = toml::to_string(config)?;
    fs::write(config_path.clone(), config_str).await?;
    Ok(config_path)
}
