use crate::block::{Block, BlockType};
use crate::{DrawInfo, draw_vertices, block, Pos, Vertex};
use glium::{Frame, VertexBuffer};
use std::time::Instant;
use crate::chunk_manager::CHUNKSIZE;
use log::{info, warn};
use crate::player::Player;

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
    pub blocks: [[[Block;CHUNKSIZE];CHUNKSIZE];CHUNKSIZE],
    pub vertex_buffer: Option<VertexBuffer<Vertex>>,
    pub update_vertices: bool
}

impl Chunk{
    pub fn render(&mut self, mut draw_info: &mut DrawInfo, mut target: & mut Frame, chunk_pos: &Pos, player: &Player) {
        if self.vertex_buffer.is_none(){
            warn!("chunk does not have vertex buffer in pos: {:?}", chunk_pos);
            return;
        }
        let vert_buffer = &self.vertex_buffer.as_ref().unwrap();
        if vert_buffer.len() == 0{
            return;
        }
        draw_vertices(&mut draw_info, &mut target, vert_buffer, player);
    }
    pub fn get_total_vertices(&self) -> i32 {
        if self.vertex_buffer.is_none(){
            warn!("chunk does not have vertex buffer");
            return 0;
        }
        self.vertex_buffer.as_ref().unwrap().len() as i32
    }
    pub fn generate() -> Chunk{
        let mut arr: [[[Block;CHUNKSIZE];CHUNKSIZE];CHUNKSIZE] = Default::default();
        for i in 0..CHUNKSIZE{
            for j in 0..CHUNKSIZE{
                for k in 0..CHUNKSIZE {
                    if i%2==0 && j%2 == 0 && k%2==0 {
                        arr[i][j][k] = block::Block::rand_new();
                    }
                }
            }
        }
        let c = Chunk{blocks: arr, vertex_buffer: None, update_vertices: false};
        return c;
    }

    pub fn update(&mut self, dt: &f32){
        return;
    }

    pub fn redo_meshes(&mut self, draw_info: &DrawInfo, chunk_pos: &Pos) {
        info!("redo matches");
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
                    if self.get_blocktype(&local_pos.get_diff(1,0,0)) == BlockType::Air{
                        sides.right = true;
                    }
                    if self.get_blocktype(&local_pos.get_diff(-1,0,0)) == BlockType::Air{
                        sides.left = true
                    }
                    if self.get_blocktype(&local_pos.get_diff(0,1,0)) == BlockType::Air{
                        sides.top = true
                    }
                    if self.get_blocktype(&local_pos.get_diff(0,-1,0)) == BlockType::Air{
                        sides.bot = true
                    }
                    if self.get_blocktype(&local_pos.get_diff(0,0,1)) == BlockType::Air{
                        sides.back = true
                    }
                    if self.get_blocktype(&local_pos.get_diff(0,0,-1)) == BlockType::Air{
                        sides.front = true
                    }
                    temp_vertex_buffer.extend(self.blocks[x][y][z].get_mesh(&global_pos, &sides).iter());
                }
            }
        }
        self.vertex_buffer = Some(glium::VertexBuffer::new(&draw_info.display, &temp_vertex_buffer).unwrap());
        info!("redid meshes in: {} seconds", last_iteration.elapsed().as_secs_f64());
    }
    pub fn get_blocktype(&self, pos: &Pos) -> BlockType{
        if pos.x < 0 || pos.x > (CHUNKSIZE - 1) as i32 || pos.y < 0 || pos.y > (CHUNKSIZE - 1) as i32 || pos.z < 0 || pos.z > (CHUNKSIZE - 1) as i32{
            return BlockType::Air;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize].block_type
    }
    pub fn set_block(&mut self, block: Block, pos: &Pos){
        if pos.x < 0 && pos.x > (CHUNKSIZE - 1) as i32 && pos.y < 0 && pos.y > (CHUNKSIZE - 1) as i32 && pos.z < 0 && pos.z > (CHUNKSIZE - 1) as i32{
            warn!("tried to place block outside chunk with pos: {:?}", pos);
            return;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] = block;
        self.vertex_buffer = None;
    }
}