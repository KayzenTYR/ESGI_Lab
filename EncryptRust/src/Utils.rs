pub fn from_vec_to_str (vec: &Vec<u8>) -> String {
    String::from_utf8_lossy(vec).to_string()
}

pub fn string_to_usize(input: String) -> usize {
    match input.trim().parse::<usize>() {
        Ok(value) => value,
        Err(_) => 0, // Return None if parsing fails
    }
}

pub fn remove_extension(file_path: &str) -> &str {
    match file_path.rsplit_once('.') {
        Some((name, _)) => name,
        None => file_path, // Return the original string if no extension exists
    }
}
