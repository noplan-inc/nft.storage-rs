use crate::encryptor::{EncryptionError, Encryptor, EncryptorFactory};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::{thread_rng, RngCore as _};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub struct AesEncryptor {
    key: Vec<u8>,
}

impl AesEncryptor {
    #[allow(dead_code)]
    pub fn new(args: AesArgs) -> Result<Self, EncryptionError> {
        let AesArgs { key } = args;
        if key.len() != 32 {
            return Err(EncryptionError::InvalidArgument(
                "Invalid key length".to_string(),
            ));
        }

        Ok(Self { key })
    }

    #[allow(dead_code)]
    pub fn generate_key() -> Result<Self, EncryptionError> {
        let mut key = [0u8; 32];
        thread_rng().fill_bytes(&mut key);
        Ok(Self { key: key.to_vec() })
    }
}

impl Encryptor for AesEncryptor {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        let mut iv = [0u8; 16];
        thread_rng().fill_bytes(&mut iv);

        let encryptor = Aes256CbcEnc::new_from_slices(self.key.as_slice(), &iv).map_err(|e| {
            EncryptionError::InvalidKeyLength(format!("Aes256CbcEnc::new_from_slices: {}", e))
        })?;

        let encrypted = encryptor.encrypt_padded_vec_mut::<Pkcs7>(data);

        let mut encrypted_data = Vec::with_capacity(iv.len() + encrypted.len());

        encrypted_data.extend_from_slice(&iv);
        encrypted_data.extend_from_slice(encrypted[..].as_ref());
        Ok(encrypted_data)
    }

    fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        if encrypted_data.len() < 16 {
            return Err(EncryptionError::InvalidArgument(
                "Encrypted data is too short".to_string(),
            ));
        }
        let (iv, encrypted_data_without_iv) = encrypted_data.split_at(16);

        let decryptor = Aes256CbcDec::new_from_slices(self.key.as_slice(), iv).map_err(|e| {
            EncryptionError::InvalidKeyLength(format!(
                "Aes256CbcDec::new_from_slices failed: {}",
                e
            ))
        })?;

        let encrypted_vec = encrypted_data_without_iv.to_vec();
        let decrypted_data = decryptor
            .decrypt_padded_vec_mut::<Pkcs7>(&encrypted_vec)
            .map_err(|e| EncryptionError::DecryptionFailed(format!("Decryption failed: {}", e)))?;

        Ok(decrypted_data)
    }

    fn generate_key(&self) -> Result<Vec<u8>, EncryptionError> {
        todo!()
    }
}
pub struct AesArgs {
    key: Vec<u8>,
}

impl AesArgs {
    #[allow(dead_code)]
    pub fn new(key: Vec<u8>) -> Result<Self, EncryptionError> {
        if key.len() != 32 {
            return Err(EncryptionError::InvalidArgument(
                "Invalid key length".to_string(),
            ));
        }

        Ok(Self { key })
    }
}

impl Default for AesArgs {
    fn default() -> Self {
        Self { key: vec![0u8; 32] }
    }
}

impl EncryptorFactory for AesEncryptor {
    type EncryptorType = AesEncryptor;
    type ArgsType = AesArgs;

    fn create(args: Self::ArgsType) -> Result<Self::EncryptorType, EncryptionError> {
        Ok(AesEncryptor {
            key: args.key.to_vec(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::encryptor::Encryptor;

    #[test]
    fn aes_encryptor_test() {
        let encryptor = AesEncryptor::generate_key().unwrap();

        let data = b"01234567890123456789012345678901";
        let encrypted_data = encryptor.encrypt(data).unwrap();
        let decrypted_data = encryptor.decrypt(&encrypted_data).unwrap();

        assert_eq!(data, decrypted_data.as_slice());
    }

    #[test]
    fn aes_encryptor_test_2() {
        let encryptor = AesEncryptor::generate_key().unwrap();

        let data = b"Hello World!";
        let encrypted_data = encryptor.encrypt(data).unwrap();
        let decrypted_data = encryptor.decrypt(&encrypted_data).unwrap();

        assert_eq!(data, decrypted_data.as_slice());
    }

    #[test]
    fn aes_encryptor_test_3() {
        let encryptor = AesEncryptor::generate_key().unwrap();

        let data = "こんにちは、世界".as_bytes();
        let encrypted_data = encryptor.encrypt(data).unwrap();
        let decrypted_data = encryptor.decrypt(&encrypted_data).unwrap();

        assert_eq!(data, decrypted_data.as_slice());
    }

    #[test]
    fn aes_encryptor_test_4() {
        let encryptor = AesEncryptor::generate_key().unwrap();

        let data = "ハローハローハローハローハローハローハローハローハローハローハロー".as_bytes();
        let encrypted_data = encryptor.encrypt(data).unwrap();
        let decrypted_data = encryptor.decrypt(&encrypted_data).unwrap();

        assert_eq!(data, decrypted_data.as_slice());
    }
}
