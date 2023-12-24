use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::Result;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    pub encrypt_method: Option<String>,
    pub encrypt_private_key: Option<Vec<u8>>,
    pub api_key: Option<String>,
    pub verbose: Option<bool>,
}

fn get_config_path() -> PathBuf {
    let mut config_dir = ProjectDirs::from("com", "noplan-inc", "nft-storage")
        .expect("no home directory")
        .config_dir()
        .to_path_buf();

    config_dir.push("config.toml");
    config_dir
}

pub async fn load_config(target_config_path: Option<PathBuf>) -> Result<Config> {
    let config_path = target_config_path.unwrap_or_else(|| get_config_path());

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let config_str = fs::read_to_string(config_path).await?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}


pub async fn write_config(config: &Config, target_config_path: Option<PathBuf>) -> Result<PathBuf> {
    let config_path = target_config_path.unwrap_or_else(|| get_config_path());
    // not exist dir
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).await?;
        }
    }

    let config_str = toml::to_string(config)?;
    fs::write(&config_path, config_str).await?;
    Ok(config_path)
}


mod tests {
    use std::path::PathBuf;

    use nft_storage_core::encryptor::{plugins::aes::AesEncryptor, self, Encryptor};

    use crate::config::{load_config, Config, write_config};

    
    #[tokio::test]
    async fn test_load_config_without_config_file() {
        let not_exist_path = Some(PathBuf::from("not_exist_config.toml"));
        let config = load_config(not_exist_path).await.expect("load config error");
        assert_eq!(config.api_key, None);
        assert_eq!(config.verbose, None);
        assert_eq!(config.encrypt_method, None);
        assert_eq!(config.encrypt_private_key, None);
    }

    #[tokio::test]
    async fn test_write_config() {
        let encryptor = AesEncryptor::generate_key().expect("failed to generate key");

        let config = Config {
            encrypt_method: Some("AES".to_string()),
            encrypt_private_key: Some(encryptor.key),
            api_key: Some("api_key".to_string()),
            verbose: Some(true),
        };

        let config_path = write_config(&config, Some(PathBuf::from("test/config.toml"))).await.unwrap();

        assert!(config_path.exists());
        // delete config file
        std::fs::remove_file(config_path).expect("failed to delete config file");
    }

    #[tokio::test]
    async fn test_load_config_with_config_file() {
        let config_path = Some(PathBuf::from("test/config_test.toml"));
        let config = load_config(config_path).await.expect("load config error");
        assert_eq!(config.api_key, Some("api_key".to_string()));
        assert_eq!(config.verbose, Some(true));
        assert_eq!(config.encrypt_method, Some("AES".to_string()));
        let private_key: Vec<u8> = vec![193, 41, 7, 112, 8, 143, 45, 101, 110, 21, 99, 154, 194, 35, 82, 64, 83, 223, 185, 207, 149, 132, 147, 203, 58, 185, 131, 185, 106, 149, 122, 47];
        assert_eq!(config.encrypt_private_key, Some(private_key));
    }

    #[tokio::test]
    async fn test_write_and_load() {
        let encryptor = AesEncryptor::generate_key().expect("failed to generate key");

        let config = Config {
            encrypt_method: Some("AES".to_string()),
            encrypt_private_key: Some(encryptor.key),
            api_key: Some("api_key".to_string()),
            verbose: Some(true),
        };

        let config_path = write_config(&config, Some(PathBuf::from("test/config_test2.toml"))).await.unwrap();

        let loaded_config = load_config(Some(config_path.clone())).await.expect("load config error");
        assert_eq!(loaded_config.api_key, config.api_key);
        assert_eq!(loaded_config.verbose, config.verbose);
        assert_eq!(loaded_config.encrypt_method, config.encrypt_method);
        assert_eq!(loaded_config.encrypt_private_key, config.encrypt_private_key);

        // delete config file
        std::fs::remove_file(config_path).expect("failed to delete config file");
    }

    #[tokio::test]
    async fn test_load_config_with_invalid_config_file() {
        let config_path = Some(PathBuf::from("test/invalid_config.toml"));
        let config = load_config(config_path).await.expect("failed to load config");
        assert_eq!(config, Config::default());
    }
}