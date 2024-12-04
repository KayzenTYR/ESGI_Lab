use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Read, Write};

pub fn get_content(path: &str) -> Vec<u8> {
    let mut file = match File::open(path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Vec::new()
        },
    };
    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer) {
        Ok(_) => {},
        Err(e) => eprintln!("Failed to read the file: {}", e),
    };

    return buffer
}

pub fn create (name: &str, content: Vec<u8>) -> bool{
    let _ = match File::create(&name) {
        Ok(mut file) => match file.write_all(&content) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error writing to the file: {}", e);
                Err(e)
            }
        },
        Err(e) => {
            eprintln!("Error creating the file: {}", e);
            Err(e)
        }
    };

    return true;
}

pub fn add_line(file_path: &str, line: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(file_path)?;

    writeln!(file, "{}", line)?;

    Ok(())
}

pub fn compare_lines_with_str(file_path: &str, target: &str) -> bool {
    let file = match File::open(file_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return false
        },
    };

    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(content) = line {
            if content == target {
                return true;
            }
        }
    }

    false
}
