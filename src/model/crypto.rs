// crypto.rs - Backend cryptography module
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine};
use bs58;
use wallet_adapter::ed25519_dalek::{SigningKey, VerifyingKey};
use x25519_dalek::{StaticSecret,  PublicKey as X25519PublicKey};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub nonce: String,
    pub encrypted: String,
    pub sender_public_key: String,
}



#[derive(Debug)]
pub enum CryptoError {
    InvalidKey(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    SerializationError(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            CryptoError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            CryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            CryptoError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

pub struct SolanaEncryption;
    
impl SolanaEncryption {
    pub fn decrypt_from_sender(
        payload: &EncryptedPayload,
        recipient_private: &str,
    ) -> Result<String, CryptoError> {
        // TODO: Implement decryption logic
        unimplemented!("Decryption logic")
    }
    //pub fn encrypt_for_recipient () {}
    pub fn encrypt_for_recipient(
    message: &str,
    recipient_public: &str,
    sender_private: &str,
    sender_public: &str,
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

    pub(crate) fn validate_private_key(private_key: &str) -> Result<(), CryptoError> {
        // TODO: Implement private key validation logic
        unimplemented!("Private key validation not implemented")
    }

    pub(crate) fn validate_public_key(public_key: &str) -> Result<(), CryptoError> {
        // TODO: Implement public key validation logic
        unimplemented!("Public key validation not implemented")
    }
}





