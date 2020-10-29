use crate::block::{Block, BlockType};
use crate::constants::{CHUNKSIZE, METACHUNK_UNLOAD_RADIUS};
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos, MetaChunkPos};
use crate::renderer::glium::{draw_vertices, DrawInfo};
use crate::renderer::vertex::Vertex;
use crate::world::World;
use crate::world_gen::chunk::Chunk;
use crate::world_gen::chunk_gen_thread::ChunkGenThread;
use glium::{Frame, VertexBuffer};
use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryInto;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct ChunkManager {
    pub world_data: World,
    pub chunk_gen_thread: ChunkGenThread,
    pub vertex_buffers: HashMap<ChunkPos, Option<VertexBuffer<Vertex>>>,
}

impl ChunkManager {
    pub fn new(seed: u32) -> ChunkManager {
        ChunkManager {
            world_data: World::new(seed),
            chunk_gen_thread: ChunkGenThread::new(),
            vertex_buffers: HashMap::new(),
        }
    }
    pub fn load_chunk(&mut self, pos: MetaChunkPos) {
        if self.world_data.chunk_exists_or_generating(&pos) {
            return;
        }
        self.world_data.loading_chunks.insert(pos.clone());
        self.chunk_gen_thread
            .request(pos, self.world_data.world_seed);
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
        self.load_generated_chunks();
    }
    pub fn gen_vertex_buffers(&mut self, draw_info: &DrawInfo, player: &Player) {
        let mut started = Instant::now();
        let mut to_render = Arc::new(Mutex::new(BTreeMap::new()));
        let vertex_buffers = Box::new(&self.vertex_buffers);
        for (_, meta_chunk) in &mut self.world_data.chunks {
            meta_chunk.for_each(|chunk, pos| {
                if started.elapsed().as_secs_f32() > 0.01 {
                    return;
                }
                let distance = pos.get_distance(&player.position.get_chunk());
                if distance > player.render_distance {
                    return;
                }
                //if !self.surrounding_chunks_exist(pos) {
                //    continue;
                //}
                let vertex_buffer_opt = vertex_buffers.get(&pos);
                if vertex_buffer_opt.is_none() || vertex_buffer_opt.unwrap().is_none() {
                    to_render
                        .lock()
                        .unwrap()
                        .deref_mut()
                        .insert((distance * 10000f32) as i32, pos.clone());
                }
            });
        }
        let to_render = Arc::try_unwrap(to_render).unwrap().into_inner().unwrap();
        started = Instant::now();
        for (_, pos) in to_render.iter() {
            if started.elapsed().as_secs_f32() > 0.01 {
                break;
            }

            let c = self.world_data.chunks.get(&pos.get_meta_chunk_pos());
            if c.is_none() {
                continue;
            }
            let local_pos = pos.get_local_chunk_pos();
            let vertices = self.get_chunk_vertices(c.unwrap().get_chunk(&local_pos).unwrap(), pos);
            let vert_buffer = glium::VertexBuffer::new(&draw_info.display, &vertices).unwrap();
            self.vertex_buffers.insert(pos.clone(), Some(vert_buffer));
        }
    }
    /*pub fn surrounding_chunks_exist(&self, pos: &ChunkPos) -> bool {
        self.world_data.chunks(&pos.get_diff(0, 0, 1))
            && self.world_data.chunks.contains_key(&pos.get_diff(0, 0, -1))
            && (pos.y + 2 > VERTICALCHUNKS as i32
                || self.world_data.chunks.contains_key(&pos.get_diff(0, 1, 0)))
            && (pos.y <= 0 || self.world_data.chunks.contains_key(&pos.get_diff(0, -1, 0)))
            && self.world_data.chunks.contains_key(&pos.get_diff(1, 0, 0))
            && self.world_data.chunks.contains_key(&pos.get_diff(-1, 0, 0))
    }*/
    pub fn load_generated_chunks(&mut self) {
        let message = self.chunk_gen_thread.get();
        if message.is_none() {
            return;
        }
        let (chunk, pos) = message.unwrap();
        self.world_data.loading_chunks.remove(&pos);
        self.world_data.chunks.insert(pos, chunk);
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
        let mut draw_info_ptr = Arc::new(Mutex::new(draw_info));
        let mut frame_ptr = Arc::new(Mutex::new(frame));
        for (_, meta_chunk) in &self.world_data.chunks {
            meta_chunk.for_each(|chunk, pos| {
                if !player.chunk_in_view_distance(&pos) {
                    return;
                }
                let vertex_buffer_opt = self.vertex_buffers.get(&pos);
                if vertex_buffer_opt.is_none() {
                    return;
                }
                let vertex_buffer = vertex_buffer_opt.unwrap();
                if vertex_buffer.is_none() {
                    return;
                }
                let real_vertex_buffer = vertex_buffer.as_ref().unwrap();
                if real_vertex_buffer.len() == 0 {
                    return;
                }
                draw_vertices(
                    &mut draw_info_ptr.lock().unwrap(),
                    &mut frame_ptr.lock().unwrap(),
                    real_vertex_buffer,
                    player,
                );
            });
        }
    }
    pub fn meta_chunk_should_be_loaded(player: &Player, pos: &MetaChunkPos) -> bool {
        let player_chunk_pos = player.position.get_meta_chunk();
        pos.x <= player_chunk_pos.x + METACHUNK_UNLOAD_RADIUS as i32
            && pos.x >= player_chunk_pos.x - METACHUNK_UNLOAD_RADIUS as i32
            && pos.y <= player_chunk_pos.y + METACHUNK_UNLOAD_RADIUS as i32
            && pos.y >= player_chunk_pos.y - METACHUNK_UNLOAD_RADIUS as i32
            && pos.z <= player_chunk_pos.z + METACHUNK_UNLOAD_RADIUS as i32
            && pos.z >= player_chunk_pos.z - METACHUNK_UNLOAD_RADIUS as i32
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
