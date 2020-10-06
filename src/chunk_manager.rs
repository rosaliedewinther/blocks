use crate::block::Block;
use crate::chunk::Chunk;
use crate::constants::CHUNKSIZE;
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos};
use crate::renderer::glium::{draw_vertices, DrawInfo};
use crate::renderer::vertex::Vertex;
use glium::{Frame, VertexBuffer};
use log::warn;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvError, Sender, TryRecvError};
use std::thread::JoinHandle;
use std::time::Instant;
use std::{thread, time};

pub struct ChunkManager {
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub loading_chunks: HashSet<ChunkPos>,
    pub to_unload: Vec<ChunkPos>,
    pub to_rebuild: Vec<ChunkPos>,
    pub visible: Vec<ChunkPos>,
    pub vertex_buffers: HashMap<ChunkPos, Option<VertexBuffer<Vertex>>>,
    pub chunk_generator_requester: Sender<ChunkPos>,
    pub chunk_generator_receiver: Receiver<(Chunk, ChunkPos)>,
    pub chunk_generator_thread: JoinHandle<()>,
    pub world_seed: u32,
}

impl ChunkManager {
    pub fn new(seed: u32) -> ChunkManager {
        let (gen_chunk_request, gen_chunk_receiver) = mpsc::channel();
        let (gen_chunk_request_done, gen_chunk_receiver_done) = mpsc::channel();
        let mut queue_chunk_gen: VecDeque<ChunkPos> = VecDeque::new();
        let chunk_gen_thread = thread::spawn(move || loop {
            loop {
                let message: Result<ChunkPos, TryRecvError> = gen_chunk_receiver.try_recv();
                if message.is_err() {
                    if message.err().unwrap() == TryRecvError::Disconnected {
                        return;
                    } else {
                        break;
                    }
                } else {
                    queue_chunk_gen.push_back(message.unwrap());
                }
            }
            if !queue_chunk_gen.is_empty() {
                let pos = queue_chunk_gen.pop_front().unwrap();
                gen_chunk_request_done.send((Chunk::generate(&pos, &seed), pos));
            } else {
                thread::sleep(time::Duration::from_millis(100));
            }
        });

        ChunkManager {
            chunks: HashMap::new(),
            loading_chunks: HashSet::new(),
            to_unload: Vec::new(),
            to_rebuild: Vec::new(),
            visible: Vec::new(),
            vertex_buffers: HashMap::new(),
            chunk_generator_thread: chunk_gen_thread,
            chunk_generator_requester: gen_chunk_request,
            chunk_generator_receiver: gen_chunk_receiver_done,
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
        if self.chunks.get(pos).is_none() && !self.loading_chunks.contains(pos) {
            return false;
        }
        return true;
    }
    pub fn load_chunk(&mut self, pos: ChunkPos) {
        if self.chunk_exists_or_generating(&pos) {
            return;
        }
        self.loading_chunks.insert(pos.clone());
        self.chunk_generator_requester.send(pos);
    }
    pub fn update(&mut self, dt: &f32) {
        let started = Instant::now();
        self.load_generated_chunks();
        for (pos, chunk) in &mut self.chunks {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }
            if chunk.update(dt) {
                self.vertex_buffers.insert(pos.clone(), None);
            }
        }
    }
    pub fn gen_vertex_buffers(&mut self, draw_info: &DrawInfo) {
        let mut started = Instant::now();
        for (pos, chunk) in &mut self.chunks {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none() || vertex_buffer_opt.unwrap().is_none() {
                let mut started2 = Instant::now();
                let vertices = chunk.get_vertex_buffer(pos, self);
                let buffer = Some(glium::VertexBuffer::new(&draw_info.display, &vertices).unwrap());
                self.vertex_buffers.insert(pos.clone(), buffer);
                println!("gen: {} sec", started2.elapsed().as_secs_f64());
            }
        }
    }
    pub fn load_generated_chunks(&mut self) {
        loop {
            let possibly_generated_chunk = self.chunk_generator_receiver.try_recv();
            if possibly_generated_chunk.is_ok() {
                let (chunk, pos) = possibly_generated_chunk.unwrap();
                self.chunks.insert(pos.clone(), chunk);
                self.loading_chunks.remove(&pos);
                self.reset_surronding_vertex_buffers(&pos);
            } else {
                break;
            }
        }
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
                continue;
            }
            let vertex_buffer = vertex_buffer_opt.unwrap();
            if vertex_buffer.is_none() {
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
    pub fn count_vertex_buffers(&self) -> i64 {
        let mut counter = 0i64;
        for (_, buffer) in &self.vertex_buffers {
            if buffer.is_none() {
                continue;
            }
            counter += 1;
        }
        return counter;
    }
}
