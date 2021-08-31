use std::collections::HashSet;
use vox_core::positions::{ChunkPos, MetaChunkPos};
use vox_render::compute_renderer::renderer::Renderer;
use vox_render::compute_renderer::wgpu_state::WgpuState;
use vox_world::algorithms::noise_default::NoiseDefault;
use vox_world::big_world_renderer::BigWorldRenderer;
use vox_world::player::Player;
use vox_world::world::big_world::BigWorld;
use vox_world::world_gen::chunk_gen_thread::ChunkGenThread;
use winit::window::Window;
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
    pub fn new(_window: &Window, renderer: &Renderer, wgpu_state: &WgpuState) -> PersonalWorld {
        let world_renderer = BigWorldRenderer::new(wgpu_state, &renderer.texture_view);
        let world = BigWorld::new::<NoiseDefault>(0);
        world.upload_world(wgpu_state, &world_renderer);
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
    pub fn on_game_tick(&mut self, _dt: f32) {
        self.update();
    }

    pub fn render(&mut self, _window: &Window, _renderer: &mut Renderer) -> RenderResult {
        return RenderResult::Continue;
    }
}
