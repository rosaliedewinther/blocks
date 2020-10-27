use crate::block::Block;
use crate::constants::{METACHUNKSIZE, VERTICALCHUNKS};
use crate::io::file_reader::read_meta_chunk_from_file;
use crate::io::file_writer::write_to_file;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalChunkPos, MetaChunkPos};
use crate::world_gen::basic::{floodfill_water, generate_empty_chunk, generate_landmass};
use crate::world_gen::chunk::Chunk;
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

    pub fn set_block(&mut self, pos: &GlobalBlockPos, block: Block) {}
    pub fn get_block(&self, pos: &GlobalBlockPos) -> Option<&Block> {
        None
    }
    /*pub fn iter(self) -> MetaChunkIterator {
        let pos = LocalChunkPos { x: 0, y: 0, z: 0 };
        MetaChunkIterator {
            meta_chunk: self,
            pos,
        }
    }*/
    pub fn for_each(&mut self, f: &dyn Fn(Chunk, ChunkPos)) {
        for x in 0..METACHUNKSIZE as i32 {
            for y in 0..VERTICALCHUNKS as i32 {
                for z in 0..METACHUNKSIZE as i32 {
                    f.call(self.get_chunk(&LocalChunkPos { x, y, z }));
                }
            }
        }
    }
    pub fn get_chunk(&self, pos: &LocalChunkPos) -> Option<&Chunk> {
        return Some(&self.chunks[pos.x as usize][pos.y as usize][pos.z as usize]);
    }
}
/*
pub struct MetaChunkIterator {
    pub meta_chunk: MetaChunk,
    pub pos: LocalChunkPos,
}

impl Iterator for MetaChunkIterator {
    type Item = (Chunk, ChunkPos);

    fn next(&mut self) -> Option<(&Chunk, ChunkPos)> {
        let &chunk =
            &self.meta_chunk.chunks[self.pos.x as usize][self.pos.y as usize][self.pos.z as usize];
        self.pos.x += 1;
        if self.pos.x == METACHUNKSIZE as i32 {
            self.pos.x = 0;
            self.pos.y += 1;
            if self.pos.y == VERTICALCHUNKS as i32 {
                self.pos.y = 0;
                self.pos.z += 1;
                if self.pos.z == METACHUNKSIZE as i32 {
                    return None;
                }
            }
        }
        let pos = ChunkPos {
            x: self.pos.x + self.meta_chunk.pos.x * METACHUNKSIZE as i32,
            y: self.pos.y + self.meta_chunk.pos.x * METACHUNKSIZE as i32,
            z: self.pos.z + self.meta_chunk.pos.x * METACHUNKSIZE as i32,
        };
        return Some((&chunk, pos));
    }
}*/
