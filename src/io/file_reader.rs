use crate::world_gen::meta_chunk::MetaChunk;
use serde::{de, Deserialize};
use std::fs::File;
use std::io::BufReader;
use zstd::Decoder;

pub fn read_meta_chunk_from_file(filename: &str) -> Option<MetaChunk> {
    let f = File::open(filename);
    if f.is_ok() {
        let reader = BufReader::new(f.unwrap());
        let decoder = Decoder::new(reader).unwrap();
        let obj: MetaChunk = serde_cbor::from_reader(decoder).unwrap();
        return Some(obj);
    }
    return None;
}
