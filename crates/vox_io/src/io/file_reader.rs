use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

pub fn read_meta_chunk_from_file<T: for<'de> Deserialize<'de>>(filename: &str) -> Option<T> {
    let f = File::open(filename);
    if f.is_ok() {
        let reader = BufReader::new(f.unwrap());
        let obj = bincode::deserialize_from(reader).unwrap();
        return Some(obj);
    }
    return None;
}
