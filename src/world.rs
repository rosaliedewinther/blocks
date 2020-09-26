use crate::chunk_manager::ChunkManager;

pub struct World {
    pub chunk_manager: ChunkManager,
    pub seed: u32,
}
