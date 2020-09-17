use crate::block::{Block, BlockType};
use crate::{block};
use glium::VertexBuffer;
use std::time::Instant;
use crate::chunk_manager::ChunkManager;
use log::{info, warn};
use noise::{Perlin, NoiseFn};
use crate::positions::{ChunkPos, LocalBlockPos, GlobalBlockPos};
use crate::renderer::{DrawInfo, Vertex};
use crate::constants::CHUNKSIZE;

#[derive(Debug, Clone)]
pub struct BlockSides{
    pub top: bool,
    pub bot: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool
}

impl BlockSides{
    pub fn new() -> BlockSides{
        BlockSides{top:false,bot:false,left:false,right:false,front:false,back:false}
    }
}

pub struct Chunk{
    pub blocks: Vec<Vec<Vec<Block>>>
}

impl Chunk{
    pub fn generate(pos: &ChunkPos) -> Chunk{
        println!("in chunk generation");
        let mut arr: Vec<Vec<Vec<Block>>> = Vec::new();
        let perlin = Perlin::new();
        for x in 0..CHUNKSIZE{
            arr.push(Vec::new());
            for y in 0..CHUNKSIZE{
                arr[x].push(Vec::new());
                for z in 0..CHUNKSIZE {
                    let local_pos = LocalBlockPos{x: x as i32,y: y as i32,z: z as i32};
                    let global_pos = GlobalBlockPos::new_from_chunk_local(&pos, &local_pos);
                    if perlin.get([global_pos.x as f64/16f64, global_pos.y as f64/16f64, global_pos.z as f64/16f64]) > 0.2f64 {
                        arr[x][y].push(block::Block::rand_new());
                    } else {
                        arr[x][y].push(block::Block::new(BlockType::Air));
                    }
                }
            }
        }
        let c = Chunk{blocks: arr};
        return c;
    }

    pub fn update(&mut self, dt: &f32) -> bool{
        return false;
    }

    pub fn get_vertex_buffer(&self, draw_info: &DrawInfo, chunk_pos: &ChunkPos, chunk_manager: &ChunkManager) -> Option<VertexBuffer<Vertex>> {
        info!("redo meshes");
        let last_iteration = Instant::now();
        let mut temp_vertex_buffer = Vec::new();
        for x in 0..CHUNKSIZE{
            for y in 0..CHUNKSIZE{
                for z in 0..CHUNKSIZE {
                    let global_pos = GlobalBlockPos {x: x as i32 + chunk_pos.x* CHUNKSIZE as i32,y: y as i32+chunk_pos.y* CHUNKSIZE as i32,z: z as i32+chunk_pos.z* CHUNKSIZE as i32 };
                    let local_pos = global_pos.get_local_pos();
                    if self.get_blocktype(&local_pos) == BlockType::Air{
                        continue;
                    }
                    let mut sides = BlockSides::new();
                    if self.should_render_side(&global_pos.get_diff(1,0,0), chunk_manager){
                        sides.right = true;
                    }
                    if self.should_render_side(&global_pos.get_diff(-1,0,0), chunk_manager){
                        sides.left = true;
                    }
                    if self.should_render_side(&global_pos.get_diff(0,1,0), chunk_manager){
                        sides.top = true;
                    }
                    if self.should_render_side(&global_pos.get_diff(0,-1,0), chunk_manager){
                        sides.bot = true;
                    }
                    if self.should_render_side(&global_pos.get_diff(0,0,1), chunk_manager){
                        sides.back = true;
                    }
                    if self.should_render_side(&global_pos.get_diff(0,0,-1), chunk_manager){
                        sides.front = true;
                    }
                    let block: &Block = &self.blocks[x][y][z];
                    temp_vertex_buffer.extend(block.get_mesh(&global_pos, &sides).iter());
                }
            }
        }
        info!("redid meshes in: {} seconds", last_iteration.elapsed().as_secs_f64());
        return Some(glium::VertexBuffer::new(&draw_info.display, &temp_vertex_buffer).unwrap());
    }
    pub fn get_blocktype(&self, pos: &LocalBlockPos) -> BlockType{
        let maybe_block_type = self.get_block(pos);
        if maybe_block_type.is_none(){
            return BlockType::Air;
        }
        return maybe_block_type.unwrap().block_type;
    }
    pub fn set_block(&mut self, block: Block, pos: &LocalBlockPos){
        if pos.x < 0 && pos.x > (CHUNKSIZE - 1) as i32 && pos.y < 0 && pos.y > (CHUNKSIZE - 1) as i32 && pos.z < 0 && pos.z > (CHUNKSIZE - 1) as i32{
            warn!("tried to place block outside chunk with pos: {:?}", &pos);
            return;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] = block;
    }
    pub fn get_block(&self, pos: &LocalBlockPos) -> Option<&Block>{
        if pos.x < 0 || pos.x > (CHUNKSIZE - 1) as i32 || pos.y < 0 || pos.y > (CHUNKSIZE - 1) as i32 || pos.z < 0 || pos.z > (CHUNKSIZE - 1) as i32{
            return None;
        }
        return Some(&self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]);
    }
    pub fn should_render_side(&self, pos: &GlobalBlockPos, chunk_manager: &ChunkManager) -> bool {
        let block = chunk_manager.get_block(pos);
        if block.is_none(){
            return true;
        }
        if block.unwrap().block_type == BlockType::Air{
            return true;
        }
        if block.unwrap().col[3] != 1.0 {
            return true;
        }
        return false;
    }
}