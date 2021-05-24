use crate::blocks::block::BlockId;
use crate::player::Player;
use crate::world_gen::chunk::Chunk;
use crate::world_gen::meta_chunk::MetaChunk;
use vox_core::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};

pub trait World {
    fn new(seed: u32) -> Self
    where
        Self: Sized;
    fn set_block(&mut self, block: BlockId, pos: GlobalBlockPos);
    fn get_block(&self, pos: GlobalBlockPos) -> Option<BlockId>;
    fn filter_chunks(&mut self, player: &Player);
    fn update(&mut self);
}
