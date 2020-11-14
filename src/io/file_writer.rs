use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use zstd::Encoder;

pub fn write_to_file<T: Serialize>(filename: &str, obj: &T) -> serde_cbor::Result<()> {
    let file = File::create(filename).unwrap();
    let writer = BufWriter::new(file);
    let encoder = Encoder::new(writer, 1).unwrap().auto_finish();
    serde_cbor::to_writer(encoder, obj)
}
