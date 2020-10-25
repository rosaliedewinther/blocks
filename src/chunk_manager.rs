use crate::block::{Block, BlockSides, BlockType};
use crate::chunk::Chunk;
use crate::constants::{CHUNKSIZE, CHUNK_UNLOAD_RADIUS, VERTICALCHUNKS};
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos};
use crate::renderer::glium::{draw_vertices, DrawInfo};
use crate::renderer::vertex::Vertex;
use glium::{Frame, VertexBuffer};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

pub struct WorldData {
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub loading_chunks: HashSet<ChunkPos>,
    pub to_unload: Vec<ChunkPos>,
    pub to_rebuild: Vec<ChunkPos>,
    pub visible: Vec<ChunkPos>,
    pub world_seed: u32,
}

pub struct ChunkManager {
    pub world_data: WorldData,
    pub chunk_generator_requester: Sender<ChunkPos>,
    pub chunk_generator_receiver: Receiver<(Chunk, ChunkPos)>,
    pub chunk_generator_thread: JoinHandle<()>,
    pub vertex_buffers: HashMap<ChunkPos, Option<VertexBuffer<Vertex>>>,
}

impl WorldData {
    pub fn new(seed: u32) -> WorldData {
        WorldData {
            chunks: HashMap::new(),
            loading_chunks: HashSet::new(),
            to_unload: Vec::new(),
            to_rebuild: Vec::new(),
            visible: Vec::new(),
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

    pub fn sides_to_render(&self, global_pos: &GlobalBlockPos) -> BlockSides {
        let mut sides = BlockSides::new();
        if self.should_render_against_block(&global_pos.get_diff(1, 0, 0)) {
            sides.right = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(-1, 0, 0)) {
            sides.left = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 1, 0)) {
            sides.top = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, -1, 0)) {
            sides.bot = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 0, 1)) {
            sides.back = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 0, -1)) {
            sides.front = true;
        }
        return sides;
    }
    pub fn should_render_against_block(&self, pos: &GlobalBlockPos) -> bool {
        let real_chunk_pos = pos.get_chunk_pos();
        if !self.chunks.contains_key(&real_chunk_pos) {
            return false;
        }
        let block = self.get_block(&pos);
        if block.is_some() {
            return block.unwrap().should_render_against();
        }
        return true;
    }
}

impl ChunkManager {
    pub fn new(seed: u32) -> ChunkManager {
        let (gen_chunk_request, gen_chunk_receiver) = mpsc::channel();
        let (gen_chunk_request_done, gen_chunk_receiver_done) = mpsc::channel();
        let chunk_gen_thread = thread::spawn(move || loop {
            let mut queue_chunk_gen: VecDeque<ChunkPos> = VecDeque::new();
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
            let chunks = Arc::new(Mutex::new(Some(Vec::new())));
            queue_chunk_gen.into_par_iter().for_each(|pos| {
                let chunk = Chunk::generate(&pos, &seed);
                let mut m = chunks.lock().unwrap();
                m.as_mut().unwrap().push((chunk, pos));
            });
            let taken_value = chunks.lock().unwrap().take();
            for c in taken_value.unwrap().into_iter() {
                gen_chunk_request_done.send(c);
            }
        });

        ChunkManager {
            world_data: WorldData::new(seed),
            chunk_generator_thread: chunk_gen_thread,
            chunk_generator_requester: gen_chunk_request,
            chunk_generator_receiver: gen_chunk_receiver_done,
            vertex_buffers: HashMap::new(),
        }
    }
    pub fn load_chunk(&mut self, pos: ChunkPos) {
        if self.world_data.chunk_exists_or_generating(&pos) {
            return;
        }
        self.world_data.loading_chunks.insert(pos.clone());
        self.chunk_generator_requester.send(pos);
    }
    pub fn reset_surrounding_vertex_buffers(&mut self, pos: &ChunkPos) {
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
    pub fn update(&mut self, dt: &f32) {
        let started = Instant::now();
        self.load_generated_chunks();
        for (pos, chunk) in &mut self.world_data.chunks {
            break;
            if chunk.update(dt) {
                self.vertex_buffers.insert(pos.clone(), None);
            }
        }
    }
    pub fn gen_vertex_buffers(&mut self, draw_info: &DrawInfo, player: &Player) {
        let mut started = Instant::now();
        let mut to_render = BTreeMap::new();
        for (pos, _) in &self.world_data.chunks {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }
            let distance = pos.get_distance(&player.position.get_chunk());
            if distance > player.render_distance {
                continue;
            }
            if !self.surrounding_chunks_exist(pos) {
                continue;
            }
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none() || vertex_buffer_opt.unwrap().is_none() {
                to_render.insert((distance * 10000f32) as i32, pos.clone());
            }
        }
        started = Instant::now();
        for pos in to_render.iter() {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }
            let c = self.world_data.chunks.get(pos.1);
            if c.is_none() {
                continue;
            }
            let vertices = self.get_chunk_vertices(c.unwrap(), pos.1);
            let vert_buffer = glium::VertexBuffer::new(&draw_info.display, &vertices).unwrap();
            self.vertex_buffers.insert(pos.1.clone(), Some(vert_buffer));
        }
    }
    pub fn surrounding_chunks_exist(&self, pos: &ChunkPos) -> bool {
        self.world_data.chunks.contains_key(&pos.get_diff(0, 0, 1))
            && self.world_data.chunks.contains_key(&pos.get_diff(0, 0, -1))
            && (pos.y + 2 > VERTICALCHUNKS as i32
                || self.world_data.chunks.contains_key(&pos.get_diff(0, 1, 0)))
            && (pos.y <= 0 || self.world_data.chunks.contains_key(&pos.get_diff(0, -1, 0)))
            && self.world_data.chunks.contains_key(&pos.get_diff(1, 0, 0))
            && self.world_data.chunks.contains_key(&pos.get_diff(-1, 0, 0))
    }
    pub fn load_generated_chunks(&mut self) {
        let mut started = Instant::now();
        loop {
            if started.elapsed().as_secs_f32() > 0.001 {
                break;
            }
            let possibly_generated_chunk = self.chunk_generator_receiver.try_recv();
            if possibly_generated_chunk.is_ok() {
                let (chunk, pos) = possibly_generated_chunk.unwrap();
                self.world_data.chunks.insert(pos.clone(), chunk);
                self.world_data.loading_chunks.remove(&pos);
                self.reset_surrounding_vertex_buffers(&pos);
            } else {
                break;
            }
        }
    }

    pub fn get_chunk_vertices(&self, chunk: &Chunk, chunk_pos: &ChunkPos) -> Vec<Vertex> {
        let mut temp_vertex_buffer: Vec<Vertex> = Vec::with_capacity(20000);
        for x in 0..CHUNKSIZE as i32 {
            for y in 0..CHUNKSIZE as i32 {
                for z in 0..CHUNKSIZE as i32 {
                    let global_pos = GlobalBlockPos {
                        x: x + (chunk_pos.x * CHUNKSIZE as i32),
                        y: y + (chunk_pos.y * CHUNKSIZE as i32),
                        z: z + (chunk_pos.z * CHUNKSIZE as i32),
                    };

                    let block = chunk.get_block(&LocalBlockPos { x, y, z });
                    if block.is_some() && block.unwrap().block_type == BlockType::Air {
                        continue;
                    }
                    let sides = self.world_data.sides_to_render(&global_pos);

                    let block: &Block = &chunk.blocks[x as usize][y as usize][z as usize];
                    let buffers = block.get_mesh(&global_pos, &sides);
                    {
                        temp_vertex_buffer.extend(buffers);
                    }
                }
            }
        }
        return temp_vertex_buffer;
    }

    pub fn render_chunks(
        &self,
        mut draw_info: &mut DrawInfo,
        mut frame: &mut Frame,
        player: &Player,
    ) {
        for (pos, _) in &self.world_data.chunks {
            if self.player_is_in_range(
                &player.position.get_chunk(),
                pos,
                player.render_distance as f32,
            ) {
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
            draw_vertices(&mut draw_info, &mut frame, real_vertex_buffer, player);
        }
    }
    pub fn chunk_should_be_loaded(player: &Player, pos: &ChunkPos) -> bool {
        let player_chunk_pos = player.position.get_chunk();
        pos.x < player_chunk_pos.x + CHUNK_UNLOAD_RADIUS as i32
            && pos.x > player_chunk_pos.x - CHUNK_UNLOAD_RADIUS as i32
            && pos.y < player_chunk_pos.y + CHUNK_UNLOAD_RADIUS as i32
            && pos.y > player_chunk_pos.y - CHUNK_UNLOAD_RADIUS as i32
            && pos.z < player_chunk_pos.z + CHUNK_UNLOAD_RADIUS as i32
            && pos.z > player_chunk_pos.z - CHUNK_UNLOAD_RADIUS as i32
    }
    pub fn player_is_in_range(
        &self,
        player_pos: &ChunkPos,
        chunk_pos: &ChunkPos,
        max_dist: f32,
    ) -> bool {
        return player_pos.get_distance(chunk_pos) > max_dist;
    }
    pub fn count_vertices(&self) -> i64 {
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
    pub fn count_chunks(&self) -> i64 {
        return self.world_data.chunks.len() as i64;
    }
    pub fn count_vertex_buffers_in_range(&self, player: &Player) -> i64 {
        let mut counter = 0i64;
        for (pos, buffer) in &self.vertex_buffers {
            if buffer.is_none()
                || player.position.get_chunk().get_distance(pos) > player.render_distance
            {
                continue;
            }
            counter += 1;
        }
        return counter;
    }
}
