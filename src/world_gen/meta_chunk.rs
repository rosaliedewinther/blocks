use crate::block::Block;
use crate::constants::{METACHUNKSIZE, VERTICALCHUNKS};
use crate::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};
use crate::world_gen::basic::{floodfill_water, generate_empty_chunk, generate_landmass};
use crate::world_gen::chunk::Chunk;
use arr_macro::arr;

struct MetaChunk {
    pub chunks: [[[Chunk; METACHUNKSIZE]; VERTICALCHUNKS]; METACHUNKSIZE],
    pub pos: MetaChunkPos,
    pub seed: u32,
}

impl MetaChunk {
    pub fn load_or_gen(pos: MetaChunkPos, seed: u32) -> MetaChunk {
        let mut chunks: [[[Chunk; METACHUNKSIZE]; VERTICALCHUNKS]; METACHUNKSIZE] =
            MetaChunk::genx();

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

    pub fn genz() -> [Chunk; METACHUNKSIZE] {
        arr![generate_empty_chunk(); 32]
    }
    pub fn geny() -> [[Chunk; METACHUNKSIZE]; VERTICALCHUNKS] {
        arr![MetaChunk::genz(); 4]
    }
    pub fn genx() -> [[[Chunk; METACHUNKSIZE]; VERTICALCHUNKS]; METACHUNKSIZE] {
        arr![MetaChunk::geny(); 32]
    }

    pub fn set_block(&mut self, pos: GlobalBlockPos, block: Block) {}
    pub fn get_block(&self, pos: GlobalBlockPos) -> Option<&Block> {
        None
    }
}
