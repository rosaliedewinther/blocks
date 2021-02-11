use crate::world_gen::meta_chunk::MetaChunk;
use std::fs::File;
use std::io::BufReader;

pub fn read_meta_chunk_from_file(filename: &str) -> Option<MetaChunk> {
    let f = File::open(filename);
    if f.is_ok() {
        let reader = BufReader::new(f.unwrap());
        let obj: MetaChunk = bincode::deserialize_from(reader).unwrap();
        return Some(obj);
    }
    return None;
}
