#![feature(test)]

extern crate test;

use crate::positions::MetaChunkPos;
use crate::world::world::World;
use crate::world_gen::meta_chunk::MetaChunk;
use crate::world_gen::vertex_generation::get_chunk_vertices;
use test::Bencher;

#[bench]
fn bench_meta_chunk_generation(b: &mut Bencher) {
    b.iter(|| MetaChunk::load_or_gen(MetaChunkPos { x: 0, z: 0 }, 0, false));
}
#[bench]
fn bench_vertex_generation(b: &mut Bencher) {
    let mut w = World::new(0);
    let c = MetaChunk::load_or_gen(MetaChunkPos { x: 0, z: 0 }, 0, false);
    w.add_chunk(MetaChunkPos { x: 0, z: 0 }, c);
    b.iter(|| {
        let (_, chunk) = &w.get_all_chunks()[0];
        for (_, pos) in chunk.get_iter() {
            let data = get_chunk_vertices(&w, &pos);
        }
    });
}
