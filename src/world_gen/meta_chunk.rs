use crate::block::Block;
use crate::constants::{METACHUNKSIZE, VERTICALCHUNKS};
use crate::io::file_reader::read_meta_chunk_from_file;
use crate::io::file_writer::write_to_file;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalChunkPos, MetaChunkPos};
use crate::world_gen::basic::{floodfill_water, generate_empty_chunk, generate_landmass};
use crate::world_gen::chunk::Chunk;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;

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

    pub fn set_block(&mut self, pos: &GlobalBlockPos, block: Block) {
        let chunk_pos = pos.get_local_chunk();
        let chunk = self.get_chunk_mut(&chunk_pos);
        match chunk {
            Some(c) => c.set_block(block, &pos.get_local_pos()),
            None => {}
        }
    }
    pub fn get_block(&self, pos: &GlobalBlockPos) -> Option<&Block> {
        let chunk_pos = pos.get_local_chunk();
        let chunk = self.get_chunk(&chunk_pos);
        match chunk {
            Some(c) => c.get_block(&pos.get_local_pos()),
            None => None,
        }
    }
    pub fn for_each_mut(&mut self, f: impl Fn(&mut Chunk, ChunkPos)) {
        for x in 0..METACHUNKSIZE as i32 {
            for y in 0..VERTICALCHUNKS as i32 {
                for z in 0..METACHUNKSIZE as i32 {
                    let pos = ChunkPos {
                        x: self.pos.x * METACHUNKSIZE as i32 + x,
                        y,
                        z: self.pos.z * METACHUNKSIZE as i32 + z,
                    };
                    f.call((self.get_chunk_mut(&LocalChunkPos { x, y, z }).unwrap(), pos));
                }
            }
        }
    }
    pub fn for_each(&self, f: impl Fn(&Chunk, ChunkPos)) {
        for x in 0..METACHUNKSIZE as i32 {
            for y in 0..VERTICALCHUNKS as i32 {
                for z in 0..METACHUNKSIZE as i32 {
                    let pos = ChunkPos {
                        x: self.pos.x * METACHUNKSIZE as i32 + x,
                        y,
                        z: self.pos.z * METACHUNKSIZE as i32 + z,
                    };
                    f.call((self.get_chunk(&LocalChunkPos { x, y, z }).unwrap(), pos));
                }
            }
        }
    }
    pub fn get_chunk_mut(&mut self, pos: &LocalChunkPos) -> Option<&mut Chunk> {
        return Some(self.chunks[pos.x as usize][pos.y as usize][pos.z as usize].borrow_mut());
    }
    pub fn get_chunk(&self, pos: &LocalChunkPos) -> Option<&Chunk> {
        return Some(&self.chunks[pos.x as usize][pos.y as usize][pos.z as usize]);
    }
}
