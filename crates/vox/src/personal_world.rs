use cgmath::{InnerSpace, Vector3};
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use rayon::prelude::ParallelSliceMut;
use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use vox_core::constants::{
    CHUNKSIZE, METACHUNKSIZE, METACHUNK_GEN_RANGE, METACHUNK_UNLOAD_RADIUS, SEED,
};
use vox_core::positions::{ChunkPos, MetaChunkPos};
use vox_render::compute_renderer::renderer::Renderer;
use vox_render::compute_renderer::wgpu_state::WgpuState;
use vox_world::algorithms::noise_bracket::NoiseBracket;
use vox_world::algorithms::noise_default::NoiseDefault;
use vox_world::big_world_renderer::BigWorldRenderer;
use vox_world::player::Player;
use vox_world::world::big_world::BigWorld;
use vox_world::world::small_world::SmallWorld;
use vox_world::world_gen::chunk_gen_thread::ChunkGenThread;
use vox_world::world_gen::meta_chunk::MetaChunk;
use winit::event::Event;
use winit::event_loop::ControlFlow;
use winit::window::Window;
use winit_window_control::input::input::Input;
use winit_window_control::main_loop::RenderResult;

pub struct PersonalWorld {
    pub world: BigWorld,
    pub world_render_data: BigWorldRenderer,
    //pub chunk_render_data: HashMap<ChunkPos, ChunkRenderData>,
    pub player: Player,
    pub chunk_gen_thread: ChunkGenThread,
    pub loading_chunks: HashSet<MetaChunkPos>,
    pub reload_vertex_load_order: bool,
    pub to_generate: Vec<(f32, ChunkPos)>,
}

impl PersonalWorld {
    pub fn new(window: &Window, renderer: &Renderer, wgpu_state: &WgpuState) -> PersonalWorld {
        let world_renderer = BigWorldRenderer::new(wgpu_state, &renderer.texture_view);
        let world = BigWorld::new::<NoiseBracket>(0);
        world.upload_all_brickmaps(wgpu_state, &world_renderer);
        PersonalWorld {
            world: world,
            //chunk_render_data: HashMap::new(),
            world_render_data: world_renderer,
            player: Player::new(),
            chunk_gen_thread: ChunkGenThread::new(),
            loading_chunks: HashSet::new(),
            reload_vertex_load_order: false,
            to_generate: Vec::new(),
        }
    }
    pub fn update(&mut self) {
        self.world.update();
    }
    pub fn on_game_tick(&mut self, dt: f32) {
        //self.player.update(&dt, &self.world);
        self.update();
        //self.load_generated_chunks();
        //self.to_generate = self.vertex_buffers_to_generate();
        /*if self.player.generated_chunks_for != self.player.position.get_chunk()
            || self.reload_vertex_load_order
        {
            self.on_player_moved_chunks();
            self.player.generated_chunks_for = self.player.position.get_chunk();
            self.reload_vertex_load_order = false;
        }*/
    }

    /*pub fn vertex_buffers_to_generate(&self) -> Vec<(f32, ChunkPos)> {
        let mut to_render = Vec::with_capacity(9 * METACHUNKSIZE * METACHUNKSIZE * METACHUNKSIZE);
        for (_, meta_chunk) in self.world.get_all_chunks() {
            for (_, pos) in meta_chunk.get_iter() {
                let (should_gen, additional_weight) =
                    self.should_generate_vertex_buffers(pos.clone());
                if should_gen {
                    let distance = pos.get_distance(&self.player.position.get_chunk());
                    to_render.push((distance - additional_weight, pos.clone()));
                }
            }
        }
        to_render.par_sort_unstable_by(|val1, val2| {
            if val1 > val2 {
                return Ordering::Less;
            } else if val2 > val1 {
                return Ordering::Greater;
            }
            return Ordering::Equal;
        });
        return to_render;
    }
    pub fn should_generate_vertex_buffers(&self, pos: ChunkPos) -> (bool, f32) {
        let distance = pos.get_distance(&self.player.position.get_chunk());
        if distance > self.player.render_distance {
            return (false, 0.0);
        }

        if self.world.get_chunk(&pos.get_diff(0, 0, 1)).is_none()
            || self.world.get_chunk(&pos.get_diff(0, 0, -1)).is_none()
            || (self.world.get_chunk(&pos.get_diff(0, 1, 0)).is_none()
                && pos.y + 1 != METACHUNKSIZE as i32)
            || (self.world.get_chunk(&pos.get_diff(0, -1, 0)).is_none() && pos.y - 1 >= 0)
            || self.world.get_chunk(&pos.get_diff(1, 0, 0)).is_none()
            || self.world.get_chunk(&pos.get_diff(-1, 0, 0)).is_none()
        {
            return (false, 0.0);
        }
        //if self.chunk_render_data.contains_key(&pos) {
        //    return (false, 0.0);
        //}
        let view_dir = Vector3::new(
            self.player.direction.x,
            self.player.direction.y,
            self.player.direction.z,
        );
        let viewer_pos = Vector3::new(
            self.player.position.x,
            self.player.position.y,
            self.player.position.z,
        );
        let chunk_pos = Vector3::new(
            (pos.x * CHUNKSIZE as i32) as f32,
            (pos.y * CHUNKSIZE as i32) as f32,
            (pos.z * CHUNKSIZE as i32) as f32,
        );
        let difference = viewer_pos - chunk_pos;

        if view_dir.dot(difference) / (view_dir.magnitude() * difference.magnitude()) < -0.5 {
            return (true, 1000.0);
        }
        return (true, 0.0);
    }
    pub fn meta_chunk_should_be_loaded(player: &Player, pos: &MetaChunkPos) -> bool {
        let player_chunk_pos = player.position.get_meta_chunk();
        pos.x <= player_chunk_pos.x + METACHUNK_UNLOAD_RADIUS as i32
            && pos.x >= player_chunk_pos.x - METACHUNK_UNLOAD_RADIUS as i32
            && pos.z <= player_chunk_pos.z + METACHUNK_UNLOAD_RADIUS as i32
            && pos.z >= player_chunk_pos.z - METACHUNK_UNLOAD_RADIUS as i32
    }
    pub fn load_chunk(&mut self, pos: MetaChunkPos) {
        if self.world.chunk_exists_or_generating(&pos) {
            return;
        }
        self.loading_chunks.insert(pos.clone());
        let chunk_request_result = self.chunk_gen_thread.request(pos, SEED);
        match chunk_request_result {
            Ok(_) => (),
            Err(e) => println!("error while trying to load A chunk: {}", e),
        }
    }
    pub fn on_player_moved_chunks(&mut self) {
        self.check_chunks_to_generate();
        self.world.filter_chunks(&self.player);
        let player = &self.player;
    }
    pub fn check_chunks_to_generate(&mut self) {}
    pub fn check_vertices_to_generate(&mut self, renderer: &Renderer) -> i32 {
        return 1;
    }
    pub fn load_generated_chunks(&mut self) {
        let message = self.chunk_gen_thread.get();
        match message {
            Ok((chunk, pos)) => {
                self.loading_chunks.remove(&pos);
                self.world.add_chunk(pos, chunk);
                self.reload_vertex_load_order = true;
            }
            Err(_) => return,
        }
    }*/
    pub fn render(&mut self, window: &Window, renderer: &mut Renderer) -> RenderResult {
        return RenderResult::Continue;
    }
}
