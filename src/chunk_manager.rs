use crate::block::{Block, BlockType};
use crate::chunk::{BlockSides, Chunk};
use crate::constants::CHUNKSIZE;
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos};
use crate::renderer::glium::{draw_vertices, DrawInfo};
use crate::renderer::vertex::Vertex;
use glium::{Frame, VertexBuffer};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
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
    pub fn should_render_against_chunk_known(&self, pos: &LocalBlockPos, chunk: &Chunk) -> bool {
        let block = chunk.get_block(pos);
        if block.is_none() {
            println!("{:?}", pos);
            return true;
        }
        block.unwrap().should_render_against()
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
    pub fn gen_vertex_buffers(&mut self, draw_info: &DrawInfo, player: &Player) {
        let mut started = Instant::now();
        let mut to_render = BTreeMap::new();
        for (pos, chunk) in &self.chunks {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }
            let distance = pos.get_distance(&player.position.get_chunk());
            if distance > player.render_distance {
                continue;
            }
            let vertex_buffer_opt = self.vertex_buffers.get(pos);
            if vertex_buffer_opt.is_none() || vertex_buffer_opt.unwrap().is_none() {
                to_render.insert((distance * 10000f32) as i32, pos.clone());
            }
        }
        for pos in to_render.iter() {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }
            let mut started2 = Instant::now();
            let c = self.chunks.get(pos.1);
            if c.is_none() {
                continue;
            }
            let vertices = self.get_chunk_vertices(c.unwrap(), pos.1);
            let buffer = Some(glium::VertexBuffer::new(&draw_info.display, &vertices).unwrap());
            self.vertex_buffers.insert(pos.1.clone(), buffer);
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
    pub fn get_chunk_vertices(&self, chunk: &Chunk, chunk_pos: &ChunkPos) -> Vec<Vertex> {
        let mut temp_vertex_buffer = Vec::with_capacity(20000);
        for x in 0..CHUNKSIZE {
            for y in 0..CHUNKSIZE {
                for z in 0..CHUNKSIZE {
                    let global_pos = GlobalBlockPos {
                        x: x as i32 + chunk_pos.x * CHUNKSIZE as i32,
                        y: y as i32 + chunk_pos.y * CHUNKSIZE as i32,
                        z: z as i32 + chunk_pos.z * CHUNKSIZE as i32,
                    };

                    let block = chunk.get_block(&global_pos.get_local_pos());
                    if block.is_some() && block.unwrap().block_type == BlockType::Air {
                        continue;
                    }
                    let sides = self.sides_to_render(&global_pos, chunk, chunk_pos);

                    let block: &Block = &chunk.blocks[x][y][z];
                    temp_vertex_buffer.extend(block.get_mesh(&global_pos, &sides).iter());
                }
            }
        }
        return temp_vertex_buffer;
    }

    pub fn sides_to_render(
        &self,
        global_pos: &GlobalBlockPos,
        chunk: &Chunk,
        chunk_pos: &ChunkPos,
    ) -> BlockSides {
        let mut sides = BlockSides::new();
        if self.should_render_against_block(&global_pos.get_diff(1, 0, 0), chunk, chunk_pos) {
            sides.right = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(-1, 0, 0), chunk, chunk_pos) {
            sides.left = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 1, 0), chunk, chunk_pos) {
            sides.top = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, -1, 0), chunk, chunk_pos) {
            sides.bot = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 0, 1), chunk, chunk_pos) {
            sides.back = true;
        }
        if self.should_render_against_block(&global_pos.get_diff(0, 0, -1), chunk, chunk_pos) {
            sides.front = true;
        }
        return sides;
    }
    pub fn should_render_against_block(
        &self,
        pos: &GlobalBlockPos,
        chunk: &Chunk,
        chunk_local_pos: &ChunkPos,
    ) -> bool {
        let real_chunk_pos = pos.get_chunk_pos();
        if &real_chunk_pos == chunk_local_pos {
            return self.should_render_against_chunk_known(&pos.get_local_pos(), chunk);
        }
        let block = self.get_block(pos);
        if block.is_some() {
            return block.unwrap().should_render_against();
        }
        return true;
    }

    pub fn render_chunks(
        &self,
        mut draw_info: &mut DrawInfo,
        mut frame: &mut Frame,
        player: &Player,
    ) {
        for (pos, _) in &self.chunks {
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
            draw_vertices(&mut draw_info, &mut frame, &real_vertex_buffer, player);
        }
    }
    pub fn player_is_in_range(
        &self,
        player_pos: &ChunkPos,
        chunk_pos: &ChunkPos,
        max_dist: f32,
    ) -> bool {
        return player_pos.get_distance(chunk_pos) > max_dist;
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
    pub fn count_chunks(&self) -> i64 {
        return self.chunks.len() as i64;
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
