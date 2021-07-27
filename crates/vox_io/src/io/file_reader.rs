use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_struct_from_file<T: bytemuck::Pod + bytemuck::Zeroable>(
    filename: &str,
) -> Option<Box<T>> {
    let f = File::open(filename);
    if f.is_ok() {
        let mut reader = BufReader::new(f.unwrap());
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf);
        return Some(Box::new(*bytemuck::from_bytes(buf.as_slice())));
    }
    return None;
}
