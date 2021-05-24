use crate::world_gen::chunk::Chunk;
use crate::world_gen::meta_chunk::MetaChunk;
use vox_core::positions::{ChunkPos, MetaChunkPos};

pub trait MeshableChunk {
    fn get_all_chunks(&self) -> &Vec<(MetaChunkPos, MetaChunk)>;
    fn get_chunk(&self, pos: &ChunkPos) -> Option<&Chunk>;
    fn chunk_exists_or_generating(&self, pos: &MetaChunkPos) -> bool;
}
