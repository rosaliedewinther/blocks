use crate::positions::MetaChunkPos;
use crate::world_gen::meta_chunk::MetaChunk;
use std::time::Instant;

#[test]
fn meta_chunk_gen() {
    let now = Instant::now();
    let c = MetaChunk::load_or_gen(MetaChunkPos { x: 0, z: 0 }, 0, true);
    println!("meta_chunk_gen: {}", now.elapsed().as_secs_f64());
    let now = Instant::now();
    c.save_to_disk();
    println!("meta_chunk_save: {}", now.elapsed().as_secs_f64());
    let now = Instant::now();
    MetaChunk::load_or_gen(MetaChunkPos { x: 0, z: 0 }, 0, false);
    println!("meta_chunk_load: {}", now.elapsed().as_secs_f64());
}
