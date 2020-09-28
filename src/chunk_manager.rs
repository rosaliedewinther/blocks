use crate::block::Block;
use crate::chunk::Chunk;
use crate::constants::CHUNKSIZE;
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos};
use crate::renderer::glium::{draw_vertices, DrawInfo};
use crate::renderer::vertex::Vertex;
use glium::{Frame, VertexBuffer};
use log::warn;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

pub struct SurroundingChunkSides {
    pub blocks: HashSet<LocalBlockPos>,
}

impl SurroundingChunkSides {
    pub fn generate(chunk_manager: &ChunkManager, chunk_pos: &ChunkPos) -> SurroundingChunkSides {
        let mut blocks = HashSet::new();
        let chunk1 = chunk_manager.chunks.get(&chunk_pos.get_diff(0, 0, 1));
        if chunk1.is_some() {
            let c = chunk1.unwrap();
            for x in 0..CHUNKSIZE {
                for y in 0..CHUNKSIZE {
                    let pos = LocalBlockPos {
                        x: x as i32,
                        y: y as i32,
                        z: CHUNKSIZE as i32,
                    };
                    if c.should_render_against_block(&pos) {
                        blocks.insert(pos);
                    }
                }
            }
        }
        let chunk2 = chunk_manager.chunks.get(&chunk_pos.get_diff(0, 0, -1));
        if chunk2.is_some() {
            let c = chunk2.unwrap();
            for x in 0..CHUNKSIZE {
                for y in 0..CHUNKSIZE {
                    let pos = LocalBlockPos {
                        x: x as i32,
                        y: y as i32,
                        z: -1,
                    };
                    if c.should_render_against_block(&pos) {
                        blocks.insert(pos);
                    }
                }
            }
        }
        let chunk3 = chunk_manager.chunks.get(&chunk_pos.get_diff(0, 1, 0));
        if chunk3.is_some() {
            let c = chunk3.unwrap();
            for x in 0..CHUNKSIZE {
                for z in 0..CHUNKSIZE {
                    let pos = LocalBlockPos {
                        x: x as i32,
                        y: CHUNKSIZE as i32,
                        z: z as i32,
                    };
                    if c.should_render_against_block(&pos) {
                        blocks.insert(pos);
                    }
                }
            }
        }
        let chunk4 = chunk_manager.chunks.get(&chunk_pos.get_diff(0, -1, 0));
        if chunk4.is_some() {
            let c = chunk4.unwrap();
            for x in 0..CHUNKSIZE {
                for z in 0..CHUNKSIZE {
                    let pos = LocalBlockPos {
                        x: x as i32,
                        y: -1,
                        z: z as i32,
                    };
                    if c.should_render_against_block(&pos) {
                        blocks.insert(pos);
                    }
                }
            }
        }
        let chunk5 = chunk_manager.chunks.get(&chunk_pos.get_diff(1, 0, 0));
        if chunk5.is_some() {
            let c = chunk5.unwrap();
            for y in 0..CHUNKSIZE {
                for z in 0..CHUNKSIZE {
                    let pos = LocalBlockPos {
                        x: CHUNKSIZE as i32,
                        y: y as i32,
                        z: z as i32,
                    };
                    if c.should_render_against_block(&pos) {
                        blocks.insert(pos);
                    }
                }
            }
        }
        let chunk6 = chunk_manager.chunks.get(&chunk_pos.get_diff(-1, 0, 0));
        if chunk6.is_some() {
            let c = chunk6.unwrap();
            for y in 0..CHUNKSIZE {
                for z in 0..CHUNKSIZE {
                    let pos = LocalBlockPos {
                        x: -1,
                        y: y as i32,
                        z: z as i32,
                    };
                    if c.should_render_against_block(&pos) {
                        blocks.insert(pos);
                    }
                }
            }
        }
        return SurroundingChunkSides { blocks };
    }
}

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
    pub vertex_generator_requester: Sender<(Chunk, SurroundingChunkSides, ChunkPos)>,
    pub vertex_generator_receiver: Receiver<(ChunkPos, Vec<Vertex>)>,
    pub vertex_generator_thread: JoinHandle<()>,
    pub world_seed: u32,
}

impl ChunkManager {
    pub fn new(seed: u32) -> ChunkManager {
        let (gen_chunk_request, gen_chunk_receiver) = mpsc::channel();
        let (gen_chunk_request_done, gen_chunk_receiver_done) = mpsc::channel();
        let chunk_gen_thread = thread::spawn(move || loop {
            let message: Result<ChunkPos, RecvError> = gen_chunk_receiver.recv();
            if message.is_err() {
                return;
            } else {
                let pos = message.unwrap();
                gen_chunk_request_done.send((Chunk::generate(&pos, &seed), pos));
            }
        });

        let (gen_vertex_request, gen_vertex_receiver) = mpsc::channel();
        let (gen_vertex_request_done, gen_vertex_receiver_done) = mpsc::channel();
        let vertex_gen_thread = thread::spawn(move || loop {
            let message: Result<(Chunk, SurroundingChunkSides, ChunkPos), RecvError> =
                gen_vertex_receiver.recv();
            if message.is_err() {
                return;
            } else {
                let mes = message.unwrap();
                gen_vertex_request_done.send((mes.2.clone(), mes.0.get_vertex_buffer(&mes.2)));
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
            vertex_generator_requester: gen_vertex_request,
            vertex_generator_receiver: gen_vertex_receiver_done,
            chunk_generator_requester: gen_chunk_request,
            chunk_generator_receiver: gen_chunk_receiver_done,
            world_seed: seed,
            vertex_generator_thread: vertex_gen_thread,
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
        loop {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }
            let possibly_generated_vertex = self.vertex_generator_receiver.try_recv();
            if possibly_generated_vertex.is_ok() {
                let vertices = possibly_generated_vertex.unwrap();
                let buffer =
                    Some(glium::VertexBuffer::new(&draw_info.display, &vertices.1).unwrap());
                self.vertex_buffers.insert(vertices.0, buffer);
            } else {
                break;
            }
        }
        started = Instant::now();
        for (pos, chunk) in &self.chunks {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none() || vertex_buffer_opt.unwrap().is_none() {
                let mut started2 = Instant::now();
                let sides = SurroundingChunkSides::generate(self, pos);
                self.vertex_generator_requester
                    .send((chunk.clone(), sides, pos.clone()));
                println!("{}", started2.elapsed().as_secs_f64());
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
