use crate::block::Block;
use crate::constants::{METACHUNKSIZE, VERTICALCHUNKS};
use crate::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};
use crate::world_gen::basic::{generate_empty_chunk, generate_landmass};
use crate::world_gen::chunk::Chunk;

struct MetaChunk {
    pub chunks: [[[Chunk; METACHUNKSIZE]; VERTICALCHUNKS]; METACHUNKSIZE],
    pub pos: MetaChunkPos,
    pub seed: u32,
}

impl MetaChunk {
    /*pub fn load_or_gen(pos: MetaChunkPos, seed: u32) -> MetaChunk {

        let mut chunks = [[[generate_empty_chunk(); METACHUNKSIZE]; VERTICALCHUNKS]; METACHUNKSIZE];

        for (x, cx) in chunks.iter_mut().enumerate() {
            for (y, cy) in cx.iter_mut().enumerate() {
                for (z, cz) in cy.iter_mut().enumerate() {
                    generate_landmass(
                        &ChunkPos {
                            x: x as i32,
                            y: y as i32,
                            z: z as i32,
                        },
                        seed,
                        cz,
                    );
                }
            }
        }

        MetaChunk { pos, chunks, seed }
    }
    pub fn set_block(&mut self, pos: GlobalBlockPos, block: Block) {}*/
    pub fn get_block(&self, pos: GlobalBlockPos) -> Option<&Block> {
        None
    }
}
