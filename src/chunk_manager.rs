use crate::{Pos, DrawInfo};
use crate::chunk::Chunk;
use std::collections::HashMap;
use crate::block::Block;
use glium::Frame;
use log::{info, warn};
use std::time::Instant;
use crate::player::Player;

pub const CHUNKSIZE: usize = 16;

pub struct ChunkManager{
    pub chunks: HashMap<Pos, Chunk>,
    pub to_load: Vec<Pos>,
    pub to_unload: Vec<Pos>,
    pub to_rebuild: Vec<Pos>,
    pub visible: Vec<Pos>,
}

impl ChunkManager{
    pub fn new() -> ChunkManager{
        ChunkManager{
            chunks: HashMap::new(),
            to_load: Vec::new(),
            to_unload: Vec::new(),
            to_rebuild: Vec::new(),
            visible: Vec::new()
        }
    }
    pub fn get_block(&self, pos: &Pos) -> Option<&Block>{
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
    pub fn load_chunk(&mut self, pos: Pos){
        self.to_load.push(pos);
    }

    pub fn update(&mut self, dt: &f32){
        self.gen_chunks();
        for (pos, chunk) in &mut self.chunks {
            chunk.update(dt);
        }
    }

    pub fn gen_chunks(&mut self){
        let mut started = Instant::now();
        while started.elapsed().as_secs_f64() < 0.01{
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
    pub fn render_chunks(&mut self, draw_info: &mut DrawInfo, frame: &mut Frame, player: &Player){
        for (pos, chunk) in &mut self.chunks {
            if chunk.vertex_buffer.is_none(){
                chunk.redo_meshes(draw_info, pos);
            }
            chunk.render(draw_info, frame, pos, player);
        }
    }
    pub fn count_verticecs(&self) -> i64{
        let mut counter = 0i64;
        for (pos, chunk) in &self.chunks {
            counter += chunk.get_total_vertices() as i64;
        }
        return counter;
    }
}