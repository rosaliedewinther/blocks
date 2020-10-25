use crate::chunk::BlockSides;
use crate::positions::{GlobalBlockPos, ObjectPos};
use crate::renderer::vertex::Vertex;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
    Grass,
    Water,
    Dirt,
    Stone,
    Sand,
    Air,
}

#[derive(Clone)]
pub struct Block {
    pub block_type: BlockType,
}

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.gen_range(1, 5) {
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
        }
    }

    pub fn rand_new() -> Block {
        return Block::new(rand::random());
    }

    pub fn should_render_against(&self) -> bool {
        if self.get_col()[3] != 255 {
            return true;
        }
        return false;
    }
    pub fn get_mesh(&self, pos: GlobalBlockPos, sides: &BlockSides) -> Vec<Vertex> {
        if self.block_type == BlockType::Air {
            return Vec::new();
        }
        let mut mesh = Vec::with_capacity(36);
        let posf = pos.get_block_centre();
        if sides.right {
            self.mesh_right(&posf, &mut mesh);
        }
        if sides.left {
            self.mesh_left(&posf, &mut mesh);
        }
        if sides.top {
            self.mesh_top(&posf, &mut mesh);
        }
        if sides.bot {
            self.mesh_bottom(&posf, &mut mesh);
        }
        if sides.back {
            self.mesh_back(&posf, &mut mesh);
        }
        if sides.front {
            self.mesh_front(&posf, &mut mesh);
        }
        return mesh;
    }
    pub fn mesh_front(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>) {
        vec.push(Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.get_col(),
            normal: [0f32, 0f32, 1f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z],
            color: self.get_col(),
            normal: [0f32, 0f32, 1f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [0f32, 0f32, 1f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z],
            color: self.get_col(),
            normal: [0f32, 0f32, 1f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [0f32, 0f32, 1f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [0f32, 0f32, 1f32],
        });
    }
    pub fn mesh_back(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>) {
        vec.push(Vertex {
            position: [pos.x, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 0f32, -1f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 0f32, -1f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 0f32, -1f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 0f32, -1f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 0f32, -1f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 0f32, -1f32],
        });
    }
    pub fn mesh_left(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>) {
        vec.push(Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.get_col(),
            normal: [-1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [-1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [-1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z + 1f32],
            color: self.get_col(),
            normal: [-1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [-1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [-1f32, 0f32, 0f32],
        });
    }
    pub fn mesh_right(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>) {
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z],
            color: self.get_col(),
            normal: [1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
            color: self.get_col(),
            normal: [1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [1f32, 0f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [1f32, 0f32, 0f32],
        });
    }
    pub fn mesh_top(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>) {
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [0f32, 1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [0f32, 1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y + 1f32, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, 1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y + 1f32, pos.z],
            color: self.get_col(),
            normal: [0f32, 1f32, 0f32],
        });
    }
    pub fn mesh_bottom(&self, pos: &ObjectPos, vec: &mut Vec<Vertex>) {
        vec.push(Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.get_col(),
            normal: [0f32, -1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, -1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z],
            color: self.get_col(),
            normal: [0f32, -1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, -1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x + 1f32, pos.y, pos.z],
            color: self.get_col(),
            normal: [0f32, -1f32, 0f32],
        });
        vec.push(Vertex {
            position: [pos.x, pos.y, pos.z + 1f32],
            color: self.get_col(),
            normal: [0f32, -1f32, 0f32],
        });
    }
}
