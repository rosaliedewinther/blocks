use crate::{Vertex, Color, Pos};
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use crate::chunk::BlockSides;

#[derive(PartialEq, Clone, Copy)]
pub enum BlockType{
    Grass,
    Water,
    Dirt,
    Stone,
    Sand,
    Air
}

#[derive(Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub col: Color,
}

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.gen_range(0, 6) {
            0 => BlockType::Grass,
            1 => BlockType::Dirt,
            2 => BlockType::Sand,
            3 => BlockType::Stone,
            4 => BlockType::Water,
            5 => BlockType::Air,
            _ => BlockType::Stone
        }
    }
}

impl Default for Block{
    fn default() -> Block {
        Block::new(BlockType::Air)
    }
}

impl Block {

    pub fn new(block_type: BlockType) -> Block {
        Block {
            col:match &block_type {
                BlockType::Grass => [0.0,1.0,0.0,1.0f32],
                BlockType::Water => [0.0,0.0,1.0,0.1f32],
                BlockType::Dirt => [0.5,0.25,0.0,1.0f32],
                BlockType::Stone => [1.0,0.8,0.8,1.0f32],
                BlockType::Sand => [1.0,0.1,0.0,1.0f32],
                BlockType::Air => [1.0,0.7,0.8,1.0f32],
            },
            block_type: block_type
        }
    }

    pub fn rand_new() -> Block {
        return Block::new(rand::random());
    }

    pub fn get_mesh(&mut self, pos: &Pos, sides: & BlockSides) -> Vec<Vertex> {
        let mut mesh = Vec::new();
        if sides.right{
            self.mesh_right(&pos, &mut mesh);
        }
        if sides.left{
            self.mesh_left(&pos, &mut mesh);
        }
        if sides.top{
            self.mesh_top(&pos, &mut mesh);
        }
        if sides.bot{
            self.mesh_bottom(&pos, &mut mesh);
        }
        if sides.back{
            self.mesh_back(&pos, &mut mesh);
        }
        if sides.front{
            self.mesh_front(&pos, &mut mesh);
        }
        return mesh;
    }
    pub fn mesh_front(&mut self, pos: &Pos, vec: &mut Vec<Vertex>){
        vec.push(Vertex { position: [pos.x as f32-0.5,      pos.y as f32-0.5,      pos.z as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,      pos.z as f32-0.5], color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,      (pos.y+1) as f32-0.5,  pos.z as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,      pos.z as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  (pos.y+1) as f32-0.5,  pos.z as f32-0.5], color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,      (pos.y+1) as f32-0.5,  pos.z as f32-0.5], color: self.col });
    }
    pub fn mesh_back(&mut self, pos: &Pos, vec: &mut Vec<Vertex>){
        vec.push(Vertex { position: [pos.x as f32-0.5,      pos.y as f32-0.5,      (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,      (pos.y+1) as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,      (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,      (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,      (pos.y+1) as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  (pos.y+1) as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
    }
    pub fn mesh_left(&mut self, pos: &Pos, vec: &mut Vec<Vertex>){
        vec.push(Vertex { position: [pos.x as f32-0.5,  pos.y as f32-0.5,      pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,  (pos.y+1) as f32-0.5,  pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,  pos.y as f32-0.5,      (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,  (pos.y+1) as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,  pos.y as f32-0.5,      (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,  (pos.y+1) as f32-0.5,  pos.z as f32-0.5],     color: self.col });
    }
    pub fn mesh_right(&mut self, pos: &Pos, vec: &mut Vec<Vertex>){
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,      pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,      (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  (pos.y+1) as f32-0.5,  pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  (pos.y+1) as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  (pos.y+1) as f32-0.5,  pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,      (pos.z+1) as f32-0.5], color: self.col });
    }
    pub fn mesh_top(&mut self, pos: &Pos, vec: &mut Vec<Vertex>){
        vec.push(Vertex { position: [pos.x as f32-0.5,      (pos.y+1) as f32-0.5,  pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  (pos.y+1) as f32-0.5,  pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,      (pos.y+1) as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  (pos.y+1) as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,      (pos.y+1) as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  (pos.y+1) as f32-0.5,  pos.z as f32-0.5],     color: self.col });
    }
    pub fn mesh_bottom(&mut self, pos: &Pos, vec: &mut Vec<Vertex>){
        vec.push(Vertex { position: [pos.x as f32-0.5,      pos.y as f32-0.5,  pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,      pos.y as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,  pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
        vec.push(Vertex { position: [(pos.x+1) as f32-0.5,  pos.y as f32-0.5,  pos.z as f32-0.5],     color: self.col });
        vec.push(Vertex { position: [pos.x as f32-0.5,      pos.y as f32-0.5,  (pos.z+1) as f32-0.5], color: self.col });
    }
}