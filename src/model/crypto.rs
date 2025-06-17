// crypto.rs - Backend cryptography module
use serde::{Deserialize, Serialize};
//use bs58;
//use wallet_adapter::ed25519_dalek::{SigningKey, VerifyingKey};
//use x25519_dalek::{StaticSecret,  PublicKey as X25519PublicKey};
//use chacha20poly1305::{
    //aead::{Aead, AeadCore, KeyInit, OsRng},
   // ChaCha20Poly1305, Nonce,
//};
//use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub nonce: String,
    pub encrypted: String,
    pub sender_public_key: String,
}



#[derive(Debug)]
#[allow(dead_code)]
pub enum CryptoError {
    InvalidKey(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    SerializationError(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidKey(msg) => write!(f, "Invalid key: {msg}", ),
            CryptoError::EncryptionFailed(msg) => write!(f, "Encryption failed: {msg}"),
            CryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {msg}"),
            CryptoError::SerializationError(msg) => write!(f, "Serialization error: {msg}"),
        }
    }
}

impl std::error::Error for CryptoError {}

#[allow(dead_code)]
pub struct SolanaEncryption;
    
impl SolanaEncryption {
    #[allow(dead_code)]
    pub fn decrypt_from_sender(
        _payload: &EncryptedPayload,
        _recipient_private: &str,
    ) -> Result<String, CryptoError> {
        // TODO: Implement decryption logic
        unimplemented!("Decryption logic")
    }
    //pub fn encrypt_for_recipient () {}
    #[allow(dead_code)]
    pub fn encrypt_for_recipient(
    _message: &str,
    _recipient_public: &str,
    _sender_private: &str,
    _sender_public: &str,
) -> Result<EncryptedPayload, CryptoError> {
    // TODO: Implement encryption logic
    unimplemented!("Encryption logic")
}

    //pub fn decrypt_from_sender () {}
    //pub fn encrypt_for_recipient () {}
//     pub fn encrypt_for_recipient(
//     message: &str,
//     recipient_public: &str,
//     sender_private: &str,
//     sender_public: &str,
// ) -> Result<EncryptedPayload, CryptoError> {
//     // TODO: Implement encryption logic
//     unimplemented!("Encryption logic")
// }
#[allow(dead_code)]
    pub fn is_valid_public_key(_public_key: &str) -> bool {
        false
    }
#[allow(dead_code)]
    pub fn is_valid_private_key(_public_key: &str) -> bool {
        false
    }
#[allow(dead_code)]
    pub(crate) fn validate_private_key(_private_key: &str) -> Result<(), CryptoError> {
        // TODO: Implement private key validation logic
        unimplemented!("Private key validation not implemented")
    }
#[allow(dead_code)]
    pub(crate) fn validate_public_key(_public_key: &str) -> Result<(), CryptoError> {
        // TODO: Implement public key validation logic
        unimplemented!("Public key validation not implemented")
    }
}





