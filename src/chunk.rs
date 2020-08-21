use crate::block::{Block, BlockType};
use crate::{DrawInfo, block, Pos, Vertex};
use glium::VertexBuffer;
use std::time::Instant;
use crate::chunk_manager::{CHUNKSIZE, ChunkManager};
use log::{info, warn};

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
    pub fn generate() -> Chunk{
        println!("in chunk generation");
        let mut arr: Vec<Vec<Vec<Block>>> = Vec::new();
        for i in 0..CHUNKSIZE{
            arr.push(Vec::new());
            for j in 0..CHUNKSIZE{
                arr[i].push(Vec::new());
                for _ in 0..CHUNKSIZE {
                    if true {
                        arr[i][j].push(block::Block::rand_new());
                    } else {
                        arr[i][j].push(block::Block::new(BlockType::Air));
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

    pub fn get_vertex_buffer(&self, draw_info: &DrawInfo, chunk_pos: &Pos<i32>, chunk_manager: &ChunkManager) -> Option<VertexBuffer<Vertex>> {
        info!("redo meshes");
        let last_iteration = Instant::now();
        let mut temp_vertex_buffer = Vec::new();
        for x in 0..CHUNKSIZE{
            for y in 0..CHUNKSIZE{
                for z in 0..CHUNKSIZE {
                    let local_pos = Pos{x: x as i32,y: y as i32,z: z as i32 };
                    let global_pos = Pos{x: x as i32 + chunk_pos.x* CHUNKSIZE as i32,y: y as i32+chunk_pos.y* CHUNKSIZE as i32,z: z as i32+chunk_pos.z* CHUNKSIZE as i32 };
                    if self.get_blocktype(&local_pos) == BlockType::Air{
                        continue;
                    }
                    let mut sides = BlockSides::new();
                    if self.should_render_side(&global_pos.get_diff(1,0,0), chunk_manager){
                        sides.right = true;
                    }
                    if self.should_render_side(&global_pos.get_diff(-1,0,0), chunk_manager){
                        sides.left = true
                    }
                    if self.should_render_side(&global_pos.get_diff(0,1,0), chunk_manager){
                        sides.top = true
                    }
                    if self.should_render_side(&global_pos.get_diff(0,-1,0), chunk_manager){
                        sides.bot = true
                    }
                    if self.should_render_side(&global_pos.get_diff(0,0,1), chunk_manager){
                        sides.back = true
                    }
                    if self.should_render_side(&global_pos.get_diff(0,0,-1), chunk_manager){
                        sides.front = true
                    }
                    let block: &Block = &self.blocks[x][y][z];
                    temp_vertex_buffer.extend(block.get_mesh(&global_pos, &sides).iter());
                }
            }
        }
        info!("redid meshes in: {} seconds", last_iteration.elapsed().as_secs_f64());
        return Some(glium::VertexBuffer::new(&draw_info.display, &temp_vertex_buffer).unwrap());
    }
    pub fn get_blocktype(&self, pos: &Pos<i32>) -> BlockType{
        if pos.x < 0 || pos.x > (CHUNKSIZE - 1) as i32 || pos.y < 0 || pos.y > (CHUNKSIZE - 1) as i32 || pos.z < 0 || pos.z > (CHUNKSIZE - 1) as i32{
            return BlockType::Air;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize].block_type
    }
    pub fn set_block(&mut self, block: Block, pos: &Pos<i32>){
        if pos.x < 0 && pos.x > (CHUNKSIZE - 1) as i32 && pos.y < 0 && pos.y > (CHUNKSIZE - 1) as i32 && pos.z < 0 && pos.z > (CHUNKSIZE - 1) as i32{
            warn!("tried to place block outside chunk with pos: {:?}", pos);
            return;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] = block;
    }
    pub fn get_block(&self, pos: &Pos<i32>) -> Option<&Block>{
        let edited_pos = pos.wrap(CHUNKSIZE as i32);
        if edited_pos.x < 0 || edited_pos.x > (CHUNKSIZE - 1) as i32 || edited_pos.y < 0 || edited_pos.y > (CHUNKSIZE - 1) as i32 || edited_pos.z < 0 || edited_pos.z > (CHUNKSIZE - 1) as i32{
            warn!("tried to access invalid block: {:?}", edited_pos);
            return None;
        }
        return Some(&self.blocks[edited_pos.x as usize][edited_pos.y as usize][edited_pos.z as usize]);
    }
    pub fn should_render_side(&self, pos: &Pos<i32>, chunk_manager: &ChunkManager) -> bool {
        let block = chunk_manager.get_block(pos);
        if block.is_none(){
            return true;
        }
        if block.unwrap().block_type == BlockType::Air || block.unwrap().col[3] != 1.0 {
            return true;
        }
        return false;
    }
}