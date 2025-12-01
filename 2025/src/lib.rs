use std::fs;

pub fn read_from_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read input file")
}