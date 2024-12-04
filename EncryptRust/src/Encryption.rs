use crate::{File_Manager, Interaction_User, Utils, Common_Crypt};

use rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;

use aes_gcm::{
    aead::{Aead, KeyInit}, Aes128Gcm, Aes256Gcm, Key, Nonce
};

use chacha20::cipher::{self, KeyIvInit, StreamCipher};
use chacha20::ChaCha20;

fn rsa_encrypt_content(mut rng: OsRng, content: &Vec<u8>, key: RsaPublicKey) -> Vec<u8> {
    match key
        .encrypt(&mut rng, Pkcs1v15Encrypt, content) {
            Ok(r) => r,
            Err(e) => {
                println!("Failed to Encrypt message : {}", e);
                Vec::new()
            }
        }
}

fn ask_aes_size() -> usize {
    Utils::string_to_usize(
        Interaction_User::select_option(
            &[(1, "16"), (2, "32")],
            "Select the AES key sizes:",
            3,
        )    
    )
     
    // let msgs = [
    //     "Select the Aes key sizes : ",
    //     "Choose an option : ",
    //     "1. 128",
    //     "3. 256"
    // ];
    // Interaction_User::display_severage_msgs(&msgs);
    
    // let mut aes_key_size = 0;
    // let mut error_nb = 0;
    
    // loop {
    
    //     match Interaction_User::pick_up_input_i32("Enter a option here :") {
    //         1 => aes_key_size = 16,
    //         2 => aes_key_size = 32,
    //         _ => {
    //             Interaction_User::display_single_msg("Wrong option ...");
    //             error_nb +=1;
    //         } 
    //     }

    //     if error_nb == 3 {
    //         Interaction_User::display_single_msg("To much errors, exiting process ...");
    //         break;
    //     } else if aes_key_size != 0 {
    //         break;
    //     }
    // }

    // return aes_key_size as usize;
}

fn ask_rsa_size() -> usize {
    Utils::string_to_usize(
        Interaction_User::select_option(
            &[(1, "1024"), (2, "2048")],
            "Select the RSA key sizes:",
            3,
        )
    )
    // let msgs = [
    //     "Select the Aes key sizes : ",
    //     "Choose an option : ",
    //     "1. 1024",
    //     "3. 2048"
    // ];
    // Interaction_User::display_severage_msgs(&msgs);
    
    // let mut rsa_key_size = 0;
    // let mut error_nb = 0;
    
    // loop {
    
    //     match Interaction_User::pick_up_input_i32("Enter a option here :") {
    //         1 => rsa_key_size = 1024,
    //         2 => rsa_key_size = 2048,
    //         _ => {
    //             Interaction_User::display_single_msg("Wrong option ...");
    //             error_nb +=1;
    //         } 
    //     }

    //     if error_nb == 3 {
    //         Interaction_User::display_single_msg("To much errors, exiting process ...");
    //         break;
    //     } else if rsa_key_size != 0 {
    //         break;
    //     }
    // }

    // return rsa_key_size as usize;
}

fn encrypt_with_algorithm<F>(
    path: &str,
    content: &Vec<u8>,
    key_size: usize,
    extension: &str,
    generate_key: F,
    encrypt_content: fn(&Vec<u8>, &[u8], bool) -> Vec<u8>,
) where
    F: Fn(usize) -> Vec<u8>,
{
    let key = generate_key(key_size);

    let ciphertext = encrypt_content(content, &key, true);

    if File_Manager::create(&format!("{}.{}", path, extension), ciphertext) {
        File_Manager::create(&format!("{}_key.{}", path, extension), key.clone());

        let key_hash = Utils::from_vec_to_str(&Common_Crypt::hash(key.clone()));
        match File_Manager::add_line(&format!("./keys/{}_keys.txt", extension), &key_hash) {
            Ok(_) => {}
            Err(_) => println!("Error: Cannot add a line to the file."),
        };

        println!("Find your new files at: {}", path);
    }
}

fn aes_encrypt (path: &str, content: &Vec<u8>) {
    let aes_size = ask_aes_size();
    println!("{}", aes_size);
    
    encrypt_with_algorithm(
        path,
        content,
        aes_size,
        "aes",
        Common_Crypt::generate_key,
        Common_Crypt::aes_crypt_content,
    );
}

fn rsa_encrypt (path: &str, content: &Vec<u8>) {
    let rsa_size = ask_rsa_size();

    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, rsa_size).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    let encrypted_data = rsa_encrypt_content(rng, content, public_key.clone());

    println!("Encrypted data: {:?}", encrypted_data);

    if File_Manager::create(&format!("{}.rsa", path), encrypted_data) {
        let public_key_vec = public_key.to_pkcs1_der().expect("Failed to encode key").as_bytes().to_vec();
        let private_key_vec = private_key.to_pkcs1_der().expect("Failed to encode key").as_bytes().to_vec();

        println!("Private key {:?}", public_key_vec);
        println!("Public key {:?}", public_key_vec);

        File_Manager::create(&format!("{}_key.rsa", path), private_key_vec);

        match File_Manager::add_line("./keys/rsa_keys.txt", &Utils::from_vec_to_str(&Common_Crypt::hash(public_key_vec))) {
            Ok(_) => {},
            Err(_) => { println!("Error : cant add a line to the file");}
        };

        println!("Find your new files to : {}", path);
    }
}

fn chacha20_encrypt (path: &str, content: &Vec<u8>) {
    encrypt_with_algorithm(
        path,
        content,
        32,
        "chacha20",
        Common_Crypt::generate_key,
        Common_Crypt::chacha20_crypt_content,
    );
}

pub fn start (path: &str, content: &Vec<u8>) {
    Interaction_User::display_single_msg("Encryption Algorithm Starting ...");

    let choose = Common_Crypt::start();

    match choose.as_str() {
        "AES" => aes_encrypt(path, content),
        "RSA" => rsa_encrypt(path, content),
        "ChaCha 20" => chacha20_encrypt(path, content),
        _ => println!("Not a Valid option for encryption algorithm")
    };
}
