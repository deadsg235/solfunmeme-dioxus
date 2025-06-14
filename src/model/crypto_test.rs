//crypto_test.rs
// test
#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_keys() -> (String, String, String, String) {
        // These are dummy 32-byte base58-encoded keys for testing only.
        // In real tests, use valid Solana keypairs.
        let sender_private = bs58::encode([1u8; 32]).into_string();
        let sender_public = bs58::encode([2u8; 32]).into_string();
        let recipient_private = bs58::encode([3u8; 32]).into_string();
        let recipient_public = bs58::encode([4u8; 32]).into_string();
        (sender_private, sender_public, recipient_private, recipient_public)
    }

    #[test]
    fn test_validate_public_key_success() {
        let (_, sender_public, _, _) = get_sample_keys();
        assert!(SolanaEncryption::validate_public_key(&sender_public).is_ok());
    }

    #[test]
    fn test_validate_public_key_failure() {
        let invalid_key = "short";
        assert!(SolanaEncryption::validate_public_key(invalid_key).is_err());
    }

    #[test]
    fn test_validate_private_key_success() {
        let (sender_private, _, _, _) = get_sample_keys();
        assert!(SolanaEncryption::validate_private_key(&sender_private).is_ok());
    }

    #[test]
    fn test_validate_private_key_failure() {
        let invalid_key = "short";
        assert!(SolanaEncryption::validate_private_key(invalid_key).is_err());
    }

    #[test]
    fn test_encrypt_and_decrypt_round_trip() {
        let (sender_private, sender_public, recipient_private, recipient_public) = get_sample_keys();
        let message = "Test message for encryption";

        let encrypted = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        );
        assert!(encrypted.is_ok());
        let payload = encrypted.unwrap();

        let decrypted = SolanaEncryption::decrypt_from_sender(&payload, &recipient_private);
        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), message);
    }

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let (sender_private, sender_public, _recipient_private, recipient_public) = get_sample_keys();
        let message = "Secret message";

        let payload = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        )
        .unwrap();

        // Use a different (invalid) private key for decryption
        let wrong_private = bs58::encode([9u8; 32]).into_string();
        let result = SolanaEncryption::decrypt_from_sender(&payload, &wrong_private);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_nonce_in_payload() {
        let (sender_private, sender_public, recipient_private, recipient_public) = get_sample_keys();
        let message = "Another message";

        let mut payload = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        )
        .unwrap();

        // Corrupt the nonce
        payload.nonce = "invalidnonce".to_string();
        let result = SolanaEncryption::decrypt_from_sender(&payload, &recipient_private);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_encrypted_data_in_payload() {
        let (sender_private, sender_public, recipient_private, recipient_public) = get_sample_keys();
        let message = "Another message";

        let mut payload = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        )
        .unwrap();

        // Corrupt the encrypted data
        payload.encrypted = "invaliddata".to_string();
        let result = SolanaEncryption::decrypt_from_sender(&payload, &recipient_private);
        assert!(result.is_err());
    }
}
