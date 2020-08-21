use crate::{Pos, DrawInfo, Vertex, draw_vertices};
use crate::chunk::Chunk;
use std::collections::HashMap;
use crate::block::Block;
use glium::{Frame, VertexBuffer};
use log::{info, warn};
use std::time::Instant;
use crate::player::Player;

pub const CHUNKSIZE: usize = 16;

pub struct ChunkManager{
    pub chunks: HashMap<Pos<i32>, Chunk>,
    pub to_load: Vec<Pos<i32>>,
    pub to_unload: Vec<Pos<i32>>,
    pub to_rebuild: Vec<Pos<i32>>,
    pub visible: Vec<Pos<i32>>,
    pub vertex_buffers: HashMap<Pos<i32>, Option<VertexBuffer<Vertex>>>
}

impl ChunkManager{
    pub fn new() -> ChunkManager{
        ChunkManager{
            chunks: HashMap::new(),
            to_load: Vec::new(),
            to_unload: Vec::new(),
            to_rebuild: Vec::new(),
            visible: Vec::new(),
            vertex_buffers: HashMap::new()
        }
    }
    pub fn get_block(&self, pos: &Pos<i32>) -> Option<&Block>{
        let chunk_pos = Pos{x:pos.x/ CHUNKSIZE as i32, y:pos.y/ CHUNKSIZE as i32, z:pos.z/ CHUNKSIZE as i32 };
        let local_pos = Pos{x:pos.x% CHUNKSIZE as i32, y:pos.y% CHUNKSIZE as i32, z:pos.z% CHUNKSIZE as i32 };
        return match self.chunks.get(&chunk_pos) {
            Some(c) => Some (&c.blocks[local_pos.x as usize][local_pos.y as usize][local_pos.z as usize]),
            None => {
                warn!("could not get block from position: {:?}", pos);
                None
            }
        }
    }
    pub fn load_chunk(&mut self, pos: Pos<i32>){
        self.to_load.push(pos);
    }

    pub fn update(&mut self, dt: &f32,  draw_info: &DrawInfo){
        self.gen_chunks();
        for (pos, chunk) in &mut self.chunks {
            if chunk.update(dt){
                self.vertex_buffers.insert(*pos, None);
            }
        }

        for (pos, chunk) in &self.chunks {
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none(){
                self.vertex_buffers.insert(*pos, chunk.get_vertex_buffer(draw_info, pos));
            } else if vertex_buffer_opt.unwrap().is_none(){
                self.vertex_buffers.insert(*pos, chunk.get_vertex_buffer(draw_info, pos));
            }
        }
    }

    pub fn gen_chunks(&mut self){
        println!("gen_chunks");
        let started = Instant::now();
        while started.elapsed().as_secs_f64() < 0.01{
            println!("starting chunk gen");
            if self.to_load.len() == 0 {
                return
            }
            let pos = self.to_load.pop().unwrap();
            if self.chunks.contains_key(&pos){
                warn!("chunk already exists, not generating a new one at: {:?}", &pos);
                return;
            }
            info!("generating chunk at: {:?}", &pos);
            self.chunks.insert(pos.clone(), Chunk::generate());
            info!("done generating chunk at:  {:?}", &pos);
        }
    }
    pub fn render_chunks(&self, mut draw_info: &mut DrawInfo, mut frame: &mut Frame, player: &Player){
        for (pos, _) in &self.chunks {
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none(){
                warn!("chunk doesnt have associated vertex buffer entry in pos: {:?}", pos);
                continue;
            }
            let vertex_buffer = vertex_buffer_opt.unwrap();
            if vertex_buffer.is_none(){
                warn!("chunk does not have vertex buffer in pos: {:?}", pos);
                continue;
            }
            let real_vertex_buffer = vertex_buffer.as_ref().unwrap();
            if real_vertex_buffer.len() == 0{
                continue;
            }
            draw_vertices(&mut draw_info, &mut frame, &real_vertex_buffer, player);
        }
    }
    pub fn count_verticecs(&self) -> i64{
        let mut counter = 0i64;
        for (_, buffer) in &self.vertex_buffers {
            if buffer.is_none(){
                continue;
            }
            counter += buffer.as_ref().unwrap().len() as i64;
        }
        return counter;
    }
}