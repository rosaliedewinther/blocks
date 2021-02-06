use crate::positions::{GlobalBlockPos, ObjectPos};
use crate::renderer::vertex::{vertex, Vertex};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct BlockSides {
    pub top: bool,
    pub bot: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}

impl BlockSides {
    pub fn new() -> BlockSides {
        BlockSides {
            top: false,
            bot: false,
            left: false,
            right: false,
            front: false,
            back: false,
        }
    }
    pub fn set_all(&mut self, b: bool) {
        self.top = b;
        self.bot = b;
        self.left = b;
        self.right = b;
        self.front = b;
        self.back = b;
    }
}

#[derive(PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum BlockType {
    Grass,
    Water,
    Dirt,
    Stone,
    Sand,
    Air,
    Leaf,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Block {
    pub block_type: BlockType,
}

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.gen_range(1..5) {
            0 => BlockType::Water,
            1 => BlockType::Dirt,
            2 => BlockType::Sand,
            3 => BlockType::Stone,
            4 => BlockType::Grass,
            _ => BlockType::Stone,
        }
    }
}

impl Default for Block {
    fn default() -> Block {
        Block::new(BlockType::Air)
    }
}

impl Block {
    pub fn new(block_type: BlockType) -> Block {
        Block { block_type }
    }
    pub fn get_col(&self) -> [u8; 4] {
        match self.block_type {
            BlockType::Grass => [0, 255, 0, 255],
            BlockType::Water => [0, 0, 255, 128],
            BlockType::Dirt => [255, 64, 64, 255],
            BlockType::Stone => [128, 128, 128, 255],
            BlockType::Sand => [255, 0, 0, 255],
            BlockType::Air => [255, 0, 255, 0],
            BlockType::Leaf => [0, 128, 0, 255],
        }
    }

    pub fn rand_new() -> Block {
        return Block::new(rand::random());
    }

    pub fn should_render_against(&self, block: &Block) -> bool {
        if self.block_type == block.block_type {
            return false;
        }
        if self.get_col()[3] == 255 {
            return false;
        }
        return true;
    }
    pub fn get_mesh(&self, pos: &GlobalBlockPos, sides: &BlockSides) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::with_capacity(8);
        let mut indices = Vec::with_capacity(36);
        if self.block_type == BlockType::Air {
            return (Vec::new(), Vec::new());
        }
        let posf = pos.get_block_centre();
        if sides.right {
            self.mesh_right(&posf, &mut vertices, &mut indices);
        }
        if sides.left {
            self.mesh_left(&posf, &mut vertices, &mut indices);
        }
        if sides.top {
            self.mesh_top(&posf, &mut vertices, &mut indices);
        }
        if sides.bot {
            self.mesh_bottom(&posf, &mut vertices, &mut indices);
        }
        if sides.back {
            self.mesh_back(&posf, &mut vertices, &mut indices);
        }
        if sides.front {
            self.mesh_front(&posf, &mut vertices, &mut indices);
        }
        return (vertices, indices);
    }
    pub fn mesh_front(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        indices.push((vec.len() + 0) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 2) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 3) as u32);
        indices.push((vec.len() + 2) as u32);
        vec.push(vertex(
            [pos.x, pos.y, pos.z],
            self.get_col(),
            [0f32, 0f32, 1f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y, pos.z],
            self.get_col(),
            [0f32, 0f32, 1f32],
        ));
        vec.push(vertex(
            [pos.x, pos.y + 1f32, pos.z],
            self.get_col(),
            [0f32, 0f32, 1f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y + 1f32, pos.z],
            self.get_col(),
            [0f32, 0f32, 1f32],
        ));
    }
    pub fn mesh_back(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        indices.push((vec.len() + 0) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 2) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 3) as u32);
        indices.push((vec.len() + 2) as u32);
        vec.push(vertex(
            [pos.x, pos.y, pos.z + 1f32],
            self.get_col(),
            [0f32, 0f32, -1f32],
        ));
        vec.push(vertex(
            [pos.x, pos.y + 1f32, pos.z + 1f32],
            self.get_col(),
            [0f32, 0f32, -1f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y, pos.z + 1f32],
            self.get_col(),
            [0f32, 0f32, -1f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
            self.get_col(),
            [0f32, 0f32, -1f32],
        ));
    }
    pub fn mesh_left(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        indices.push((vec.len() + 0) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 2) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 3) as u32);
        indices.push((vec.len() + 2) as u32);
        vec.push(vertex(
            [pos.x, pos.y, pos.z],
            self.get_col(),
            [-1f32, 0f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x, pos.y + 1f32, pos.z],
            self.get_col(),
            [-1f32, 0f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x, pos.y, pos.z + 1f32],
            self.get_col(),
            [-1f32, 0f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x, pos.y + 1f32, pos.z + 1f32],
            self.get_col(),
            [-1f32, 0f32, 0f32],
        ));
    }
    pub fn mesh_right(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        indices.push((vec.len() + 0) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 2) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 3) as u32);
        indices.push((vec.len() + 2) as u32);
        vec.push(vertex(
            [pos.x + 1f32, pos.y, pos.z],
            self.get_col(),
            [1f32, 0f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y, pos.z + 1f32],
            self.get_col(),
            [1f32, 0f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y + 1f32, pos.z],
            self.get_col(),
            [1f32, 0f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
            self.get_col(),
            [1f32, 0f32, 0f32],
        ));
    }
    pub fn mesh_top(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        indices.push((vec.len() + 0) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 2) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 3) as u32);
        indices.push((vec.len() + 2) as u32);
        vec.push(vertex(
            [pos.x, pos.y + 1f32, pos.z],
            self.get_col(),
            [0f32, 1f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y + 1f32, pos.z],
            self.get_col(),
            [0f32, 1f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x, pos.y + 1f32, pos.z + 1f32],
            self.get_col(),
            [0f32, 1f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
            self.get_col(),
            [0f32, 1f32, 0f32],
        ));
    }
    pub fn mesh_bottom(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        indices.push((vec.len() + 0) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 2) as u32);
        indices.push((vec.len() + 1) as u32);
        indices.push((vec.len() + 3) as u32);
        indices.push((vec.len() + 2) as u32);
        vec.push(vertex(
            [pos.x, pos.y, pos.z],
            self.get_col(),
            [0f32, -1f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x, pos.y, pos.z + 1f32],
            self.get_col(),
            [0f32, -1f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y, pos.z],
            self.get_col(),
            [0f32, -1f32, 0f32],
        ));
        vec.push(vertex(
            [pos.x + 1f32, pos.y, pos.z + 1f32],
            self.get_col(),
            [0f32, -1f32, 0f32],
        ));
    }
}
