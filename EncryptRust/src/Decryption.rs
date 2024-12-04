use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

use crate::{Common_Crypt::{self, aes_crypt_content}, File_Manager, Interaction_User};

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

fn decrypt (
    path: &str,
    content: &Vec<u8>,
    crypt_content: fn(&Vec<u8>, &[u8], bool) -> Vec<u8>,
    extension: &str
)
{
    let (_, key) = Interaction_User::ask_path_to_file("key");
    
    let ciphertext = crypt_content(content, &key, false);

    File_Manager::create(&format!("{}.{}", path, extension), ciphertext);
}

fn aes_decrypt (path: &str, content: &Vec<u8>) {
    decrypt(path, content, aes_crypt_content, "daes");
}

fn rsa_decrypt (path: &str, content: &Vec<u8>) {

}

fn chacha20_decrypt (path: &str, content: &Vec<u8>) {
    decrypt(path, content, aes_crypt_content, "dchacha20");
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