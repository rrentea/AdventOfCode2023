use std::fs::File;
use std::io::{self, Read};

pub fn read_file(file_path: &str) -> io::Result<String> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
