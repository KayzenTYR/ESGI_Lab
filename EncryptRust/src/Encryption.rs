use crate::{File_Manager, Interaction_User, Utils};

use aes_gcm::{
    aead::{Aead, KeyInit}, Aes128Gcm, Aes256Gcm, Key, Nonce
};

fn aes_encrypt_content(content: &Vec<u8>, key: &[u8]) -> Vec<u8> {
    let nonce = Nonce::from_slice(&[0u8; 12]);

    match key.len() {
        16 => {
            let key_symetrics = Key::<Aes128Gcm>::from_slice(&key);
            let cipher = Aes128Gcm::new(key_symetrics);
            match cipher.encrypt(nonce, content.as_ref()) {
                Ok(r) => r,
                Err(_) => { 
                    eprintln!("Encryption failed !");
                    Vec::new()
                }
            }
        },
        32 => {
            let key_symetrics = Key::<Aes256Gcm>::from_slice(&key);
            let cipher = Aes256Gcm::new(key_symetrics);
            match cipher.encrypt(nonce, content.as_ref()) {
                Ok(r) => r,
                Err(_) => { 
                    eprintln!("Encryption failed !");
                    Vec::new()
                }
            }
        }
        _ => {
            println!("No encryption for length : {}", key.len());
            Vec::new()
        }
    }
    
}

fn ask_aes_size() -> usize {
    let msgs = [
        "Select the Aes key sizes : ",
        "Choose an option : ",
        "1. 128",
        "3. 256"
    ];
    Interaction_User::display_severage_msgs(&msgs);
    
    let mut aes_key_size = 0;
    let mut error_nb = 0;
    
    loop {
    
        match Interaction_User::pick_up_input_i32("Enter a option here :") {
            1 => aes_key_size = 16,
            2 => aes_key_size = 32,
            _ => {
                Interaction_User::display_single_msg("Wrong option ...");
                error_nb +=1;
            } 
        }

        if error_nb == 3 {
            Interaction_User::display_single_msg("To much errors, exiting process ...");
            break;
        } else if aes_key_size != 0 {
            break;
        }
    }

    return aes_key_size as usize;
}

fn aes_encrypt (path: &str, content: &Vec<u8>) {
    let aes_size = ask_aes_size();
    
    let aes_key = Utils::generate_key(aes_size);

    let ciphertext = aes_encrypt_content(content, &aes_key);

    if File_Manager::create(&format!("{}.aes", path), ciphertext) {
        File_Manager::create(&format!("{}_key.aes", path), aes_key.clone());

        match File_Manager::add_line("aes_keys.txt", &Utils::from_vec_to_str(&Utils::hash(aes_key.clone()))) {
            Ok(_) => {},
            Err(_) => { println!("Error : cant add a line to the file");}
        };

        println!("Find your new files to : {}", path);
    }
}

fn rsa_encrypt (path: &str, content: &Vec<u8>) {
    println!("function not implemented yet");
}

fn chacha20_encrypt (path: &str, content: &Vec<u8>) {
    println!("function not implemented yet");
}

pub fn start (path: &str, content: &Vec<u8>) {
    let msgs = [
        "Select an Algorithm : ",
        "Choose an option : ",
        "1. AES",
        "2. RSA",
        "3. ChaCha 20"
    ];
    Interaction_User::display_severage_msgs(&msgs);

    let mut error_nb = 0;
    
    loop {
        let error_nb_clone = error_nb.clone();
    
        match Interaction_User::pick_up_input_i32("Enter a option here :") {
            1 => aes_encrypt(path, content),
            2 => rsa_encrypt(path, content),
            3 => chacha20_encrypt(path, content),
            _ => {
                Interaction_User::display_single_msg("Wrong option ...");
                error_nb +=1;
            } 
        }

        if error_nb == 3 {
            Interaction_User::display_single_msg("To much errors, exiting process ...");
            break;
        } else if error_nb == error_nb_clone {
            break;
        }

    }
}