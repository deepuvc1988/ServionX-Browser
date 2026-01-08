// Encryption Module
// AES-256-GCM encryption and Argon2 password hashing

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString, PasswordHash, PasswordVerifier},
    Argon2,
};
use rand::rngs::OsRng;
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Handles encryption operations
pub struct Encryption {
    // Configuration for Argon2
    argon2: Argon2<'static>,
}

impl Encryption {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
    
    /// Generate a random salt
    pub fn generate_salt(&self) -> String {
        SaltString::generate(&mut OsRng).to_string()
    }
    
    /// Hash a password with Argon2
    pub fn hash_password(&self, password: &str) -> Result<(String, String), String> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();
        
        Ok((password_hash, salt.to_string()))
    }
    
    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str, _salt: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
        Ok(self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    /// Derive an encryption key from a password
    pub fn derive_key(&self, password: &str, salt: &str) -> Result<Vec<u8>, String> {
        // Use simple key derivation for now
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut key = [0u8; 32];
        let combined = format!("{}{}", password, salt);
        
        // Simple key stretching
        let mut hasher = DefaultHasher::new();
        for i in 0..10000 {
            combined.hash(&mut hasher);
            i.hash(&mut hasher);
        }
        
        let hash_val = hasher.finish();
        for i in 0..4 {
            let bytes = hash_val.to_le_bytes();
            for (j, byte) in bytes.iter().enumerate() {
                key[i * 8 + j] = *byte;
            }
        }
        
        // Fill remaining bytes
        for i in 0..32 {
            key[i] = key[i].wrapping_add(salt.as_bytes().get(i % salt.len()).copied().unwrap_or(0));
            key[i] = key[i].wrapping_add(password.as_bytes().get(i % password.len()).copied().unwrap_or(0));
        }
        
        Ok(key.to_vec())
    }
    
    /// Encrypt data with AES-256-GCM
    pub fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
        if key.len() != 32 {
            return Err("Key must be 32 bytes".to_string());
        }
        
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| format!("Cipher init error: {}", e))?;
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher.encrypt(nonce, data)
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        
        Ok(result)
    }
    
    /// Decrypt data with AES-256-GCM
    pub fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
        if key.len() != 32 {
            return Err("Key must be 32 bytes".to_string());
        }
        
        if data.len() < 12 {
            return Err("Data too short".to_string());
        }
        
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| format!("Cipher init error: {}", e))?;
        
        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&data[..12]);
        let ciphertext = &data[12..];
        
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        Ok(plaintext)
    }
    
    /// Encrypt and encode as base64
    pub fn encrypt_to_base64(&self, data: &[u8], key: &[u8]) -> Result<String, String> {
        let encrypted = self.encrypt(data, key)?;
        Ok(BASE64.encode(&encrypted))
    }
    
    /// Decode base64 and decrypt
    pub fn decrypt_from_base64(&self, encoded: &str, key: &[u8]) -> Result<Vec<u8>, String> {
        let encrypted = BASE64.decode(encoded).map_err(|e| e.to_string())?;
        self.decrypt(&encrypted, key)
    }
}

impl Default for Encryption {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_hashing() {
        let enc = Encryption::new();
        let password = "test_password_123";
        
        let (hash, salt) = enc.hash_password(password).unwrap();
        
        assert!(enc.verify_password(password, &hash, &salt).unwrap());
        assert!(!enc.verify_password("wrong_password", &hash, &salt).unwrap());
    }
    
    #[test]
    fn test_encryption_decryption() {
        let enc = Encryption::new();
        let key = enc.derive_key("password", "salt").unwrap();
        let data = b"Hello, ServionX!";
        
        let encrypted = enc.encrypt(data, &key).unwrap();
        let decrypted = enc.decrypt(&encrypted, &key).unwrap();
        
        assert_eq!(data.as_slice(), decrypted.as_slice());
    }
}
