use crate::block::Block;
use crate::chunk::Chunk;
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos};
use crate::renderer::glium::{draw_vertices, DrawInfo};
use crate::renderer::vertex::Vertex;
use glium::{Frame, VertexBuffer};
use log::warn;
use std::collections::HashMap;
use std::time::Instant;

pub struct ChunkManager {
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub to_load: Vec<ChunkPos>,
    pub to_unload: Vec<ChunkPos>,
    pub to_rebuild: Vec<ChunkPos>,
    pub visible: Vec<ChunkPos>,
    pub vertex_buffers: HashMap<ChunkPos, Option<VertexBuffer<Vertex>>>,
}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunks: HashMap::new(),
            to_load: Vec::new(),
            to_unload: Vec::new(),
            to_rebuild: Vec::new(),
            visible: Vec::new(),
            vertex_buffers: HashMap::new(),
        }
    }
    pub fn get_block(&self, pos: &GlobalBlockPos) -> Option<&Block> {
        return match self.chunks.get(&pos.get_chunk_pos()) {
            Some(c) => c.get_block(&pos.get_local_pos()),
            None => None,
        };
    }
    pub fn load_chunk(&mut self, pos: ChunkPos) {
        self.to_load.push(pos);
    }
    pub fn update(&mut self, dt: &f32, draw_info: &DrawInfo, seed: &u32) {
        let started = Instant::now();
        while started.elapsed().as_secs_f32() < 0.001 {
            self.gen_chunk(seed);
        }
        for (pos, chunk) in &mut self.chunks {
            if started.elapsed().as_secs_f32() < 0.001 {
                break;
            }
            if chunk.update(dt) {
                self.vertex_buffers.insert(pos.clone(), None);
            }
        }

        for (pos, chunk) in &self.chunks {
            if started.elapsed().as_secs_f32() < 0.001 {
                break;
            }
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none() || vertex_buffer_opt.unwrap().is_none() {
                self.vertex_buffers
                    .insert(pos.clone(), chunk.get_vertex_buffer(draw_info, pos, &self));
            }
        }
    }
    pub fn gen_chunk(&mut self, seed: &u32) {
        if self.to_load.len() == 0 {
            return;
        }
        let pos = self.to_load.pop().unwrap();
        if self.chunks.contains_key(&pos) {
            warn!(
                "chunk already exists, not generating a new one at: {:?}",
                &pos
            );
            return;
        }
        self.chunks.insert(pos.clone(), Chunk::generate(&pos, seed));
        self.reset_surronding_vertex_buffers(&pos);
    }
    pub fn reset_surronding_vertex_buffers(&mut self, pos: &ChunkPos) {
        if self.vertex_buffers.contains_key(&pos.get_diff(0, 0, 1)) {
            self.vertex_buffers.insert(pos.get_diff(0, 0, 1), None);
        }
        if self.vertex_buffers.contains_key(&pos.get_diff(0, 0, -1)) {
            self.vertex_buffers.insert(pos.get_diff(0, 0, -1), None);
        }
        if self.vertex_buffers.contains_key(&pos.get_diff(0, 1, 0)) {
            self.vertex_buffers.insert(pos.get_diff(0, 1, 0), None);
        }
        if self.vertex_buffers.contains_key(&pos.get_diff(0, -1, 0)) {
            self.vertex_buffers.insert(pos.get_diff(0, -1, 0), None);
        }
        if self.vertex_buffers.contains_key(&pos.get_diff(1, 0, 0)) {
            self.vertex_buffers.insert(pos.get_diff(1, 0, 0), None);
        }
        if self.vertex_buffers.contains_key(&pos.get_diff(-1, 0, 0)) {
            self.vertex_buffers.insert(pos.get_diff(-1, 0, 0), None);
        }
    }
    pub fn render_chunks(
        &self,
        mut draw_info: &mut DrawInfo,
        mut frame: &mut Frame,
        player: &Player,
    ) {
        for (pos, _) in &self.chunks {
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none() {
                warn!(
                    "chunk doesnt have associated vertex buffer entry in pos: {:?}",
                    pos
                );
                continue;
            }
            let vertex_buffer = vertex_buffer_opt.unwrap();
            if vertex_buffer.is_none() {
                warn!("chunk does not have vertex buffer in pos: {:?}", pos);
                continue;
            }
            let real_vertex_buffer = vertex_buffer.as_ref().unwrap();
            if real_vertex_buffer.len() == 0 {
                continue;
            }
            draw_vertices(&mut draw_info, &mut frame, &real_vertex_buffer, player);
        }
    }
    pub fn count_verticecs(&self) -> i64 {
        let mut counter = 0i64;
        for (_, buffer) in &self.vertex_buffers {
            if buffer.is_none() {
                continue;
            }
            counter += buffer.as_ref().unwrap().len() as i64;
        }
        return counter;
    }
}
