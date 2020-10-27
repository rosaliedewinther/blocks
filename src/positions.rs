use crate::constants::{CHUNKSIZE, METACHUNKSIZE};
use crate::utils::wrap;
use core::ops;
use num_traits::Pow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
pub struct GlobalBlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct LocalBlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
pub struct ObjectPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Hash, PartialEq, Debug, Clone)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub struct MetaChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
pub struct LocalChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Eq for ChunkPos {}

impl GlobalBlockPos {
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> GlobalBlockPos {
        GlobalBlockPos {
            x: self.x + x_diff,
            y: self.y + y_diff,
            z: self.z + z_diff,
        }
    }
    pub fn new() -> GlobalBlockPos {
        GlobalBlockPos { x: 0, y: 0, z: 0 }
    }
    pub fn get_local_pos(&self) -> LocalBlockPos {
        LocalBlockPos {
            x: wrap(self.x, CHUNKSIZE as i32),
            y: wrap(self.y, CHUNKSIZE as i32),
            z: wrap(self.z, CHUNKSIZE as i32),
        }
    }
    pub fn get_chunk_pos(&self) -> ChunkPos {
        ChunkPos {
            x: (self.x as f32 / CHUNKSIZE as f32).floor() as i32,
            y: (self.y as f32 / CHUNKSIZE as f32).floor() as i32,
            z: (self.z as f32 / CHUNKSIZE as f32).floor() as i32,
        }
    }
    pub fn get_block_centre(&self) -> ObjectPos {
        ObjectPos {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }
    pub fn get_meta_chunk_pos(&self) -> MetaChunkPos {
        MetaChunkPos {
            x: (self.x as f32 / (CHUNKSIZE as f32 * METACHUNKSIZE as f32)).floor() as i32,
            y: (self.y as f32 / (CHUNKSIZE as f32 * METACHUNKSIZE as f32)).floor() as i32,
            z: (self.z as f32 / (CHUNKSIZE as f32 * METACHUNKSIZE as f32)).floor() as i32,
        }
    }
}
impl ChunkPos {
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> ChunkPos {
        ChunkPos {
            x: self.x + x_diff,
            y: self.y + y_diff,
            z: self.z + z_diff,
        }
    }
    pub fn get_distance(&self, pos: &ChunkPos) -> f32 {
        ((((self.x - pos.x) as f32).pow(2)
            + ((self.y - pos.y) as f32).pow(2)
            + ((self.z - pos.z) as f32).pow(2)) as f32)
            .sqrt()
            * CHUNKSIZE as f32
    }
    pub fn get_local_chunk_pos(&self) -> LocalChunkPos {
        LocalChunkPos {
            x: wrap(self.x, METACHUNKSIZE as i32),
            y: wrap(self.y, METACHUNKSIZE as i32),
            z: wrap(self.z, METACHUNKSIZE as i32),
        }
    }
    pub fn get_meta_chunk_pos(&self) -> MetaChunkPos {
        MetaChunkPos {
            x: self.x / METACHUNKSIZE as i32,
            y: self.y / METACHUNKSIZE as i32,
            z: self.z / METACHUNKSIZE as i32,
        }
    }
}
impl MetaChunkPos {
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> MetaChunkPos {
        MetaChunkPos {
            x: self.x + x_diff,
            y: self.y + y_diff,
            z: self.z + z_diff,
        }
    }
}

impl ObjectPos {
    pub fn get_chunk(&self) -> ChunkPos {
        ChunkPos {
            x: self.x as i32 / CHUNKSIZE as i32,
            y: self.y as i32 / CHUNKSIZE as i32,
            z: self.z as i32 / CHUNKSIZE as i32,
        }
    }
    pub fn get_meta_chunk(&self) -> MetaChunkPos {
        MetaChunkPos {
            x: self.x as i32 / (CHUNKSIZE as i32 * METACHUNKSIZE as i32),
            y: self.y as i32 / (CHUNKSIZE as i32 * METACHUNKSIZE as i32),
            z: self.z as i32 / (CHUNKSIZE as i32 * METACHUNKSIZE as i32),
        }
    }
}
impl LocalBlockPos {
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> LocalBlockPos {
        LocalBlockPos {
            x: (self.x + x_diff),
            y: (self.y + y_diff),
            z: (self.z + z_diff),
        }
    }
}

impl ops::Sub<i32> for GlobalBlockPos {
    type Output = GlobalBlockPos;

    fn sub(self, val: i32) -> GlobalBlockPos {
        GlobalBlockPos {
            x: self.x - val,
            y: self.y - val,
            z: self.z - val,
        }
    }
}
