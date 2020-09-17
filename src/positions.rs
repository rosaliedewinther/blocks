use core::ops;
use crate::constants::CHUNKSIZE;

pub struct GlobalBlockPos{
    pub x: i32,
    pub y: i32,
    pub z: i32
}
#[derive(Debug)]
pub struct LocalBlockPos{
    pub x: i32,
    pub y: i32,
    pub z: i32
}
pub struct ObjectPos{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
#[derive(Hash, PartialEq, Debug, Clone)]
pub struct ChunkPos{
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Eq for ChunkPos {}

impl GlobalBlockPos{
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> GlobalBlockPos{
        GlobalBlockPos {x: self.x+x_diff, y: self.y+y_diff, z: self.z+z_diff}
    }
    pub fn new()-> GlobalBlockPos{
        GlobalBlockPos{x:0,y:0,z:0}
    }
    pub fn get_local_pos(&self) -> LocalBlockPos{
        LocalBlockPos {x:self.x% CHUNKSIZE as i32, y:self.y% CHUNKSIZE as i32, z:self.z% CHUNKSIZE as i32 }
    }
    pub fn get_chunk_pos(&self) -> ChunkPos{
        ChunkPos {x:self.x/ CHUNKSIZE as i32, y:self.y/ CHUNKSIZE as i32, z:self.z/ CHUNKSIZE as i32 }
    }
    pub fn get_block_centre(&self) -> ObjectPos{
        ObjectPos {x:self.x as f32-0.5, y:self.y as f32-0.5, z:self.z as f32-0.5}
    }
    pub fn new_from_chunk_local(chunk_pos: &ChunkPos, local_pos: &LocalBlockPos)->GlobalBlockPos{
        GlobalBlockPos{x:chunk_pos.x* CHUNKSIZE as i32 +local_pos.x, y:chunk_pos.y* CHUNKSIZE as i32 +local_pos.y, z:chunk_pos.z* CHUNKSIZE as i32 +local_pos.z}
    }
}
impl ChunkPos{
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> ChunkPos{
        ChunkPos {x: self.x+x_diff, y: self.y+y_diff, z: self.z+z_diff}
    }
}
impl LocalBlockPos{

}

impl ops::Sub<i32> for GlobalBlockPos {
    type Output = GlobalBlockPos;

    fn sub(self, val: i32) -> GlobalBlockPos {
        GlobalBlockPos {x: self.x-val, y: self.y-val, z: self.z-val}
    }
}