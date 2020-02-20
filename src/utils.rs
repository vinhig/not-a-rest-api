use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Open and read completely given file.
pub fn read_all(path: &str) -> String {
    let file = File::open(path).expect(&format!("Couldn't open {}", path));
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader
        .read_to_string(&mut content)
        .expect(&format!("Couldn't read {}", path));
    return content + "\0";
}
