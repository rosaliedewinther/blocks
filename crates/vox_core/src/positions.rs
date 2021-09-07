use crate::constants::{CHUNKSIZE, METACHUNKSIZE};
use crate::utils::{wrap, wrapf};
use core::ops;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GlobalBlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
//block pos within A chunk
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct LocalBlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
//position of entities
#[derive(Debug, PartialEq)]
pub struct ObjectPos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
//position of global chunk
#[derive(Hash, PartialEq, Debug, Clone, PartialOrd, Ord)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
//meta chunk location
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct MetaChunkPos {
    pub x: i32,
    pub z: i32,
}
//chunk pos within A metachunk
#[derive(Debug, PartialEq, Eq)]
pub struct LocalChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl LocalChunkPos {
    pub fn get_chunk_pos(&self, pos: &MetaChunkPos) -> ChunkPos {
        ChunkPos {
            x: pos.x * METACHUNKSIZE as i32 + self.x,
            y: self.y,
            z: pos.z * METACHUNKSIZE as i32 + self.z,
        }
    }
}

impl Eq for ChunkPos {}

impl GlobalBlockPos {
    pub fn get_local_chunk(&self) -> LocalChunkPos {
        LocalChunkPos {
            x: wrapf(self.x as f32 / (CHUNKSIZE as f32), METACHUNKSIZE as f32).floor() as i32,
            y: wrapf(self.y as f32 / (CHUNKSIZE as f32), METACHUNKSIZE as f32).floor() as i32,
            z: wrapf(self.z as f32 / (CHUNKSIZE as f32), METACHUNKSIZE as f32).floor() as i32,
        }
    }
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> GlobalBlockPos {
        GlobalBlockPos {
            x: self.x + x_diff,
            y: self.y + y_diff,
            z: self.z + z_diff,
        }
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
            x: self.x as f64 + 0.5,
            y: self.y as f64 + 0.5,
            z: self.z as f64 + 0.5,
        }
    }
    pub fn get_block_pos(&self) -> ObjectPos {
        ObjectPos {
            x: self.x as f64,
            y: self.y as f64,
            z: self.z as f64,
        }
    }
    pub fn get_meta_chunk_pos(&self) -> MetaChunkPos {
        MetaChunkPos {
            x: (self.x as f32 / (CHUNKSIZE as f32 * METACHUNKSIZE as f32)).floor() as i32,
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
    pub fn in_fov(&self) -> bool {
        true
    }

    pub fn get_distance(&self, pos: &ChunkPos) -> f64 {
        ((((self.x - pos.x) as f64).powf(2.0)
            + ((self.y - pos.y) as f64).powf(2.0)
            + ((self.z - pos.z) as f64).powf(2.0)) as f64)
            .sqrt()
            * CHUNKSIZE as f64
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
            x: (self.x as f32 / METACHUNKSIZE as f32).floor() as i32,
            z: (self.z as f32 / METACHUNKSIZE as f32).floor() as i32,
        }
    }
    pub fn get_center_pos(&self) -> ObjectPos {
        ObjectPos {
            x: self.x as f64 * CHUNKSIZE as f64 + CHUNKSIZE as f64 / 2.0,
            y: self.y as f64 * CHUNKSIZE as f64 + CHUNKSIZE as f64 / 2.0,
            z: self.z as f64 * CHUNKSIZE as f64 + CHUNKSIZE as f64 / 2.0,
        }
    }
}

impl MetaChunkPos {
    pub fn get_diff(&self, x_diff: i32, z_diff: i32) -> MetaChunkPos {
        MetaChunkPos {
            x: self.x + x_diff,
            z: self.z + z_diff,
        }
    }
    pub fn get_distance_to_object(&self, pos: &ObjectPos) -> f32 {
        let center_pos = self.get_center_pos();
        ((center_pos.x - pos.x).powf(2.0) as f32 + (center_pos.z - pos.z).powf(2.0) as f32).sqrt()
    }
    pub fn get_center_pos(&self) -> ObjectPos {
        ObjectPos {
            x: self.x as f64 * METACHUNKSIZE as f64 * CHUNKSIZE as f64 + METACHUNKSIZE as f64 / 2.0,
            y: 0f64,
            z: self.z as f64 * METACHUNKSIZE as f64 * CHUNKSIZE as f64 + METACHUNKSIZE as f64 / 2.0,
        }
    }
}

impl ObjectPos {
    pub fn get_block(&self) -> GlobalBlockPos {
        GlobalBlockPos {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
        }
    }

    pub fn get_chunk(&self) -> ChunkPos {
        ChunkPos {
            x: self.x as i32 / CHUNKSIZE as i32,
            y: self.y as i32 / CHUNKSIZE as i32,
            z: self.z as i32 / CHUNKSIZE as i32,
        }
    }
    pub fn get_distance(&self, pos: &ObjectPos) -> f32 {
        ((self.x - pos.x).powf(2.0) as f32
            + (self.y - pos.y).powf(2.0) as f32
            + (self.z - pos.z).powf(2.0) as f32)
            .sqrt()
    }
    pub fn get_meta_chunk(&self) -> MetaChunkPos {
        MetaChunkPos {
            x: self.x as i32 / (CHUNKSIZE as i32 * METACHUNKSIZE as i32),
            z: self.z as i32 / (CHUNKSIZE as i32 * METACHUNKSIZE as i32),
        }
    }
    pub fn get_diff(&self, x_diff: f64, y_diff: f64, z_diff: f64) -> ObjectPos {
        ObjectPos {
            x: (self.x + x_diff),
            y: (self.y + y_diff),
            z: (self.z + z_diff),
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
