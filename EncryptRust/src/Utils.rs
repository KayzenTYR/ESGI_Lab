use sha2::{Sha256, Digest};

use rand::Rng;

pub fn from_vec_to_str (vec: &Vec<u8>) -> String {
    String::from_utf8_lossy(vec).to_string()
}

pub fn remove_extension(file_path: &str) -> &str {
    match file_path.rsplit_once('.') {
        Some((name, _)) => name,
        None => file_path, // Return the original string if no extension exists
    }
}

pub fn hash(input: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    
    hasher.update(&input);
    
    hasher.finalize().to_vec()
}


pub fn generate_key (size: usize) -> Vec<u8> {
    let mut key = Vec::with_capacity(size);
    
    for _ in 0..size {
        key.push(rand::thread_rng().gen());
    };

    return key;
}
