use crate::{Common_Crypt, File_Manager, Interaction_User, Utils};

use rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;

use aes_gcm::{
    aead::{Aead, KeyInit}, Aes128Gcm, Aes256Gcm, Key, Nonce
};

use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use sha2::digest::generic_array::GenericArray;

fn rsa_decrypt_content(content: &Vec<u8>, key: RsaPrivateKey) -> Vec<u8> {
    match key
        .decrypt(Pkcs1v15Encrypt, content) {
            Ok(r) => r,
            Err(e) => {
                println!("Failed to Encrypt message : {}", e);
                Vec::new()
            }
        }
}


fn aes_decrypt (path: &str, content: &Vec<u8>) {

}

fn rsa_decrypt (path: &str, content: &Vec<u8>) {

}

fn chacha20_decrypt (path: &str, content: &Vec<u8>) {

}

pub fn start (path: &str, content: &Vec<u8>) {
    Interaction_User::display_single_msg("Decryption Algorithm Starting ...");

    let choose = Common_Crypt::start();

    match choose.as_str() {
        "AES" => aes_decrypt(path, content),
        "RSA" => rsa_decrypt(path, content),
        "ChaCha 20" => chacha20_decrypt(path, content),
        _ => println!("Not a Valid option for encryption algorithm")
    };
}