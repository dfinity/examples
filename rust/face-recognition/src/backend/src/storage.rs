use bytes::Bytes;
use std::io::Write;

pub fn bytes(filename: &str) -> Bytes {
    std::fs::read(filename).unwrap().into()
}

pub fn append_bytes(filename: &str, bytes: Vec<u8>) {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    file.write_all(&bytes).unwrap();
}

pub fn clear_bytes(filename: &str) {
    // Don't fail if the file doesn't exist.
    let _ = std::fs::remove_file(filename);
}
