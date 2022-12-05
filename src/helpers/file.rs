use std::{fs::File, io::Read};

pub fn file_open(path: &str) -> String {
    let mut temp_buf: String = String::new();

    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut temp_buf).unwrap();

    temp_buf
}