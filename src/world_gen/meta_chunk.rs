use crate::block::Block;
use crate::constants::{CHUNKSIZE, METACHUNKSIZE, VERTICALCHUNKS};
use crate::io::file_reader::read_meta_chunk_from_file;
use crate::io::file_writer::write_to_file;
use crate::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};
use crate::world_gen::basic::{floodfill_water, generate_empty_chunk, generate_landmass};
use crate::world_gen::chunk::Chunk;
use arr_macro::arr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetaChunk {
    pub chunks: Vec<Vec<Vec<Chunk>>>,
    pub pos: MetaChunkPos,
    pub seed: u32,
}

impl MetaChunk {
    pub fn load_or_gen(pos: MetaChunkPos, seed: u32) -> MetaChunk {
        let loaded = MetaChunk::load_from_disk(&pos);
        if loaded.is_some() {
            return loaded.unwrap();
        }

        let mut chunks: Vec<Vec<Vec<Chunk>>> = Vec::with_capacity(METACHUNKSIZE);
        for x in 0..METACHUNKSIZE {
            chunks.push(Vec::new());
            for y in 0..VERTICALCHUNKS {
                chunks[x].push(Vec::new());
                for z in 0..METACHUNKSIZE {
                    chunks[x][y].push(generate_empty_chunk());
                }
            }
        }

        for (x, cx) in chunks.iter_mut().enumerate() {
            for (y, cy) in cx.iter_mut().enumerate() {
                for (z, cz) in cy.iter_mut().enumerate() {
                    let pos = &ChunkPos {
                        x: x as i32,
                        y: y as i32,
                        z: z as i32,
                    };
                    generate_landmass(pos, seed, cz);
                    floodfill_water(cz, pos);
                }
            }
        }

        MetaChunk { pos, chunks, seed }
    }

    pub fn load_from_disk(pos: &MetaChunkPos) -> Option<MetaChunk> {
        let filename = format!("{}-{}-{}.txt", pos.x, pos.y, pos.z);
        return read_meta_chunk_from_file(filename.as_str());
    }

    pub fn save_to_disk(&self) {
        let filename = format!("{}-{}-{}.txt", self.pos.x, self.pos.y, self.pos.z);
        write_to_file(filename.as_str(), self);
    }

    pub fn set_block(&mut self, pos: GlobalBlockPos, block: Block) {}
    pub fn get_block(&self, pos: &GlobalBlockPos) -> Option<&Block> {
        let x = pos - (self.pos.x * CHUNKSIZE * METACHUNKSIZE) / (CHUNKSIZE * METACHUNKSIZE);
    }
}
