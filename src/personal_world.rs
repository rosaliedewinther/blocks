use crate::constants::{METACHUNK_GEN_RANGE, METACHUNK_UNLOAD_RADIUS};
use crate::main_loop::MainLoop;
use crate::player::Player;
use crate::positions::{ChunkPos, MetaChunkPos};
use crate::renderer::chunk_render_data::ChunkRenderData;
use crate::renderer::renderer::Renderer;
use crate::world::World;
use crate::world_gen::chunk_gen_thread::ChunkGenThread;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::sync::Mutex;
use std::time::Instant;
use wgpu::Device;
use winit::event_loop::ControlFlow;
use winit::window::Window;

pub struct PersonalWorld {
    pub world: World,
    pub chunk_render_data: HashMap<ChunkPos, ChunkRenderData>,
    pub player: Player,
    pub chunk_gen_thread: ChunkGenThread,
    pub loading_chunks: HashSet<MetaChunkPos>,
    pub renderer: Renderer,
    pub reload_vertex_load_order: bool,
    pub to_generate: BinaryHeap<(i32, ChunkPos)>,
}

impl PersonalWorld {
    pub fn new(window: &Window) -> PersonalWorld {
        PersonalWorld {
            renderer: Renderer::new(&window),
            world: World::new(1),
            chunk_render_data: HashMap::new(),
            player: Player::new(),
            chunk_gen_thread: ChunkGenThread::new(),
            loading_chunks: HashSet::new(),
            reload_vertex_load_order: false,
            to_generate: BinaryHeap::new(),
        }
    }
    pub fn update(&mut self) {
        self.world.update();
        self.check_vertices_to_generate();
    }
    pub fn on_game_tick(&mut self, dt: f32) {
        self.player.update(&dt);
        self.load_generated_chunks();
        self.update();
        if self.player.generated_chunks_for != self.player.position.get_chunk()
            || self.reload_vertex_load_order
        {
            self.on_player_moved_chunks();
            self.player.generated_chunks_for = self.player.position.get_chunk();
            self.reload_vertex_load_order = false;
        }
    }
    pub fn vertex_buffers_to_generate(&self) -> BinaryHeap<(i32, ChunkPos)> {
        let to_render = Mutex::new(BinaryHeap::new());
        for (_, meta_chunk) in &self.world.chunks {
            meta_chunk.for_each(|_, pos| {
                if self.should_generate_vertex_buffers(pos.clone()) {
                    let distance = pos.get_distance(&self.player.position.get_chunk());
                    to_render
                        .lock()
                        .unwrap()
                        .push(((-distance * 10000f32) as i32, pos.clone()));
                }
            });
        }
        return to_render.into_inner().unwrap();
    }
    pub fn should_generate_vertex_buffers(&self, pos: ChunkPos) -> bool {
        let distance = pos.get_distance(&self.player.position.get_chunk());
        if distance > self.player.render_distance {
            return false;
        }
        if self.chunk_render_data.contains_key(&pos) {
            return false;
        }
        return true;
    }
    pub fn meta_chunk_should_be_loaded(player: &Player, pos: &MetaChunkPos) -> bool {
        let player_chunk_pos = player.position.get_meta_chunk();
        pos.x <= player_chunk_pos.x + METACHUNK_UNLOAD_RADIUS as i32
            && pos.x >= player_chunk_pos.x - METACHUNK_UNLOAD_RADIUS as i32
            && pos.z <= player_chunk_pos.z + METACHUNK_UNLOAD_RADIUS as i32
            && pos.z >= player_chunk_pos.z - METACHUNK_UNLOAD_RADIUS as i32
    }
    pub fn load_chunk(&mut self, pos: MetaChunkPos) {
        if self.loading_chunks.contains(&pos) {
            return;
        }
        self.loading_chunks.insert(pos.clone());
        let chunk_request_result = self.chunk_gen_thread.request(pos, self.world.world_seed);
        match chunk_request_result {
            Ok(_) => (),
            Err(e) => println!("error while trying to load a chunk: {}", e),
        }
    }
    pub fn on_player_moved_chunks(&mut self) {
        self.check_chunks_to_generate();
        self.to_generate = self.vertex_buffers_to_generate();
        let player = &self.player;
        self.world
            .chunks
            .retain(|pos, _| PersonalWorld::meta_chunk_should_be_loaded(&player, pos));
    }
    pub fn check_chunks_to_generate(&mut self) {
        let current_chunk = self.player.position.get_meta_chunk();
        let mut to_load = BinaryHeap::new();
        for x in current_chunk.x - METACHUNK_GEN_RANGE as i32 - 1
            ..current_chunk.x + METACHUNK_GEN_RANGE as i32 + 1
        {
            for z in current_chunk.z - METACHUNK_GEN_RANGE as i32 - 1
                ..current_chunk.z + METACHUNK_GEN_RANGE as i32 + 1
            {
                if PersonalWorld::meta_chunk_should_be_loaded(&self.player, &MetaChunkPos { x, z })
                    && !self.loading_chunks.contains(&MetaChunkPos { x, z })
                    && !self
                        .chunk_render_data
                        .contains_key(&MetaChunkPos { x, z }.get_center_pos().get_chunk())
                {
                    let chunk_pos = MetaChunkPos { x, z };
                    to_load.push((
                        (chunk_pos.get_distance_to_object(&self.player.position) * 10f32) as i64
                            * -1,
                        chunk_pos,
                    ));
                }
            }
        }
        while !to_load.is_empty() {
            self.load_chunk(to_load.pop().unwrap().1);
        }
    }
    pub fn check_vertices_to_generate(&mut self) {
        let lag_timer = Instant::now();
        while !self.to_generate.is_empty() {
            let (_, pos) = self.to_generate.pop().unwrap();
            //let c = self.world.get_chunk(pos.clone());
            let timer = Instant::now();
            println!("started generating vertices for: {:?}", &pos);
            //let data = chunk.generate_vertex_buffers(&self.renderer.wgpu.device);
            let data = ChunkRenderData::new(
                self.world.chunks.get(&pos.get_meta_chunk_pos()).unwrap(),
                &pos.get_local_chunk_pos(),
                &self.renderer.wgpu.device,
            );
            self.chunk_render_data.insert(pos.clone(), data);
            println!(
                "done generating vertices for: {:?} in: {} sec",
                &pos,
                timer.elapsed().as_secs_f32()
            );
            /*match c {
                Some(chunk) => {

                }
                None => (),
            }*/
            if lag_timer.elapsed().as_secs_f32() > 0.01 {
                return;
            }
        }
    }
    pub fn load_generated_chunks(&mut self) {
        let message = self.chunk_gen_thread.get();
        match message {
            Ok((chunk, pos)) => {
                self.loading_chunks.remove(&pos);
                self.world.chunks.insert(pos, chunk);
                self.reload_vertex_load_order = true;
            }
            Err(_) => return,
        }
    }
    pub fn render(&mut self, control_flow: &mut ControlFlow) {
        let main_pipeline = self.renderer.pipelines.get_mut("main").unwrap();
        main_pipeline.uniforms.update_view_proj(
            &self.player,
            (
                self.renderer.wgpu.size.width,
                self.renderer.wgpu.size.height,
            ),
            self.world.time,
        );
        let render_data = &self.chunk_render_data;
        main_pipeline.set_uniform_buffer(&self.renderer.wgpu.queue, main_pipeline.uniforms);
        self.renderer.do_render_pass(render_data);
        match self.renderer.do_render_pass(render_data) {
            Ok(_) => {}
            // Recreate the swap_chain if lost
            Err(wgpu::SwapChainError::Lost) => {
                MainLoop::resize(self.renderer.wgpu.size, &mut self.renderer.wgpu)
            }
            // The system is out of memory, we should probably quit
            Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
