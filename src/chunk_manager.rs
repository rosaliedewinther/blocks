use crate::block::Block;
use crate::chunk::Chunk;
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos};
use crate::renderer::glium::{draw_vertices, DrawInfo};
use crate::renderer::vertex::Vertex;
use glium::{Frame, VertexBuffer};
use log::warn;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::thread;
use std::thread::{JoinHandle, Thread};
use std::time::Instant;

pub struct ChunkManager {
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub to_load: Vec<ChunkPos>,
    pub to_unload: Vec<ChunkPos>,
    pub to_rebuild: Vec<ChunkPos>,
    pub visible: Vec<ChunkPos>,
    pub vertex_buffers: HashMap<ChunkPos, Option<VertexBuffer<Vertex>>>,
    pub chunk_generator_requester: Sender<ChunkPos>,
    pub chunk_generator_receiver: Receiver<Chunk>,
    pub chunk_generator_thread: JoinHandle<()>,
    pub world_seed: u32,
}

impl ChunkManager {
    pub fn new(seed: u32) -> ChunkManager {
        let (gen_chunk_request, gen_chunk_receiver) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        let mut genthread = thread::spawn(move || loop {
            let message: Result<ChunkPos, RecvError> = gen_chunk_receiver.recv();
            if message.is_err() {
                return;
            } else {
                tx.send(Chunk::generate(&message.unwrap(), &seed));
            }
        });
        ChunkManager {
            chunks: HashMap::new(),
            to_load: Vec::new(),
            to_unload: Vec::new(),
            to_rebuild: Vec::new(),
            visible: Vec::new(),
            vertex_buffers: HashMap::new(),
            chunk_generator_thread: genthread,
            chunk_generator_requester: gen_chunk_request,
            chunk_generator_receiver: rx,
            world_seed: seed,
        }
    }
    pub fn get_block(&self, pos: &GlobalBlockPos) -> Option<&Block> {
        return match self.chunks.get(&pos.get_chunk_pos()) {
            Some(c) => c.get_block(&pos.get_local_pos()),
            None => None,
        };
    }
    pub fn chunk_exists_or_generating(&self, pos: &ChunkPos) -> bool {
        if self.chunks.get(pos).is_none() && !self.to_load.contains(pos) {
            return false;
        }
        return true;
    }
    pub fn load_chunk(&mut self, pos: ChunkPos) {
        self.to_load.push(pos);
    }
    pub fn update(&mut self, dt: &f32) {
        let started = Instant::now();
        while started.elapsed().as_secs_f32() < 0.01 {
            if self.to_load.len() == 0 {
                return;
            }
            self.gen_chunk();
        }
        for (pos, chunk) in &mut self.chunks {
            if started.elapsed().as_secs_f32() < 0.01 {
                break;
            }
            if chunk.update(dt) {
                self.vertex_buffers.insert(pos.clone(), None);
            }
        }
    }
    pub fn gen_vertex_buffers(&mut self, draw_info: &DrawInfo) {
        for (pos, chunk) in &self.chunks {
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none() || vertex_buffer_opt.unwrap().is_none() {
                self.vertex_buffers
                    .insert(pos.clone(), chunk.get_vertex_buffer(draw_info, pos, &self));
            }
        }
    }
    pub fn gen_chunk(&mut self) {
        let pos = self.to_load.pop().unwrap();
        if self.chunks.contains_key(&pos) {
            warn!(
                "chunk already exists, not generating a new one at: {:?}",
                &pos
            );
            return;
        }
        self.chunks
            .insert(pos.clone(), Chunk::generate(&pos, &self.world_seed));
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
        render_distance: u32,
    ) {
        for (pos, _) in &self.chunks {
            let player_chunk_pos = player.position.get_chunk();
            if pos.x >= player_chunk_pos.x + render_distance as i32
                || pos.x <= player_chunk_pos.x - render_distance as i32
                || pos.z >= player_chunk_pos.z + render_distance as i32
                || pos.z <= player_chunk_pos.z - render_distance as i32
            {
                continue;
            }
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
