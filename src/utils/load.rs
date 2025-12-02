use std::{fs, io};

pub fn load_from_file(path: &str) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(path)?;

    let lines: Vec<String> = contents
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    Ok(lines)
}
