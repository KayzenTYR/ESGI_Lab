use aes_gcm::{
    aead::{Aead, KeyInit}, Aes128Gcm, Aes256Gcm, Key, Nonce
};

use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;

use sha2::{Sha256, Digest};

use rand::{rngs::OsRng, RngCore};

use crate::{Common_Crypt, File_Manager, Interaction_User, Utils};

pub fn aes_crypt_content(content: &Vec<u8>, key: &[u8], encrypt: bool) -> Vec<u8> {
    fn process_cipher<C: Aead>(
        cipher: C,
        content: &[u8],
        encrypt: bool,
    ) -> Vec<u8> {
        let nonce = Nonce::from_slice(&[0u8; 12]);
        if encrypt {
            cipher.encrypt(nonce, content).unwrap_or_else(|_| {
                eprintln!("Encryption failed!");
                Vec::new()
            })
        } else {
            cipher.decrypt(nonce, content).unwrap_or_else(|_| {
                eprintln!("Decryption failed!");
                Vec::new()
            })
        }
    }

    if !encrypt {
        let key_hash = Utils::from_vec_to_str(&Common_Crypt::hash(key.to_vec()));

        if !File_Manager::compare_lines_with_str("./keys/aes_keys.txt", &key_hash) {
            return Vec::new();
        }
    }
    
    match key.len() {
        16 => {
            let key_symetrics = Key::<Aes128Gcm>::from_slice(key);
            let cipher = Aes128Gcm::new(key_symetrics);
            process_cipher(cipher, content, encrypt)
        }
        32 => {
            let key_symetrics = Key::<Aes256Gcm>::from_slice(key);
            let cipher = Aes256Gcm::new(key_symetrics);
            process_cipher(cipher, content, encrypt)
        }
        _ => {
            eprintln!("Invalid key length: {}", key.len());
            Vec::new()
        }
    }
}

pub fn chacha20_crypt_content(content: &Vec<u8>, key: &[u8], encrypt: bool) -> Vec<u8> {
    let nonce = Nonce::from_slice(&[0u8; 12]);

    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut buffer = content.to_vec(); // Create a mutable buffer for encryption
    cipher.apply_keystream(&mut buffer); // Encrypt in place
    buffer
}

pub fn hash(input: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    
    hasher.update(&input);
    
    hasher.finalize().to_vec()
}

pub fn generate_key (size: usize) -> Vec<u8> {
    let mut key = vec![0u8; size];
    
    OsRng.fill_bytes(&mut key);

    key
}

pub fn start () -> String {
    Interaction_User::select_option(
        &[(1, "AES"), (2, "RSA"), (3, "ChaCha 20")],
        "Select an Algorithm :",
        3,
    )
}