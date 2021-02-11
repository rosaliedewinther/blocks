use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;

pub fn write_to_file<T: Serialize>(filename: &str, obj: &T) {
    let file = File::create(filename).unwrap();
    let writer = BufWriter::new(file);
    bincode::serialize_into(writer, obj).unwrap();
}
