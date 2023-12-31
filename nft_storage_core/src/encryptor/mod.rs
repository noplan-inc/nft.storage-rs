use std::fmt::Debug;

pub mod plugins;

pub trait Encryptor: Send + Sync + Debug {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError>;
    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError>;
    fn generate_key(&self) -> Result<Vec<u8>, EncryptionError>;
    fn clone_box(&self) -> Box<dyn Encryptor + Send + Sync>;
}

#[derive(Debug)]
pub enum EncryptionError {
    IoError(std::io::Error),
    InvalidArgument(String),
    InvalidKeyLength(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    KeyGenerationFailed(String),
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionError::IoError(e) => write!(f, "IO error: {}", e),
            EncryptionError::InvalidArgument(e) => write!(f, "Invalid argument: {}", e),
            EncryptionError::InvalidKeyLength(e) => write!(f, "Invalid key length: {}", e),
            EncryptionError::EncryptionFailed(e) => write!(f, "Encryption failed: {}", e),
            EncryptionError::DecryptionFailed(e) => write!(f, "Decryption failed: {}", e),
            EncryptionError::KeyGenerationFailed(e) => write!(f, "Key generation failed: {}", e),
        }
    }
}

pub trait EncryptorFactory: Send + Sync {
    type EncryptorType: Encryptor;
    type ArgsType;

    fn create(args: Self::ArgsType) -> Result<Self::EncryptorType, EncryptionError>;
}
