use crate::algorithms::noise_abstraction::Noise;
use crate::algorithms::noise_default::NoiseDefault;
use crate::big_world_renderer::BigWorldRenderer;
use crate::blocks::block::BlockId;
use crate::player::Player;
use crate::world_gen::generator::WorldGenerator;
use crate::world_gen::hills::HillsWorldGenerator;
use std::time::Instant;
use vox_core::constants::WORLD_SIZE;
use vox_core::positions::{GlobalBlockPos, MetaChunkPos};
use vox_render::compute_renderer::wgpu_state::WgpuState;

pub struct BigWorld {
    world: Box<[BlockId]>,
    pub world_seed: u32,
    pub time: f64,
    start_time: Instant,
}

impl BigWorld {
    #[inline]
    pub fn get_block(&self, _pos: GlobalBlockPos) -> Option<BlockId> {
        None
    }
    pub fn new<T: Noise>(seed: u32) -> BigWorld {
        let generator = HillsWorldGenerator::<NoiseDefault>::new();
        let world = generator.generate_area(0, 0, 0, WORLD_SIZE);

        BigWorld {
            world,
            world_seed: seed,
            time: 0.0,
            start_time: Instant::now(),
        }
    }
    pub fn set_block(&mut self, _block: u8, _pos: GlobalBlockPos) {
        todo!()
    }
    pub fn filter_chunks(&mut self, _player: &Player) {
        todo!()
    }
    pub fn upload_world(&self, wgpu_state: &WgpuState, world_renderer: &BigWorldRenderer) {
        world_renderer.upload_world(&self.world, wgpu_state);
        world_renderer.rebuild_sdf(wgpu_state);
    }
    pub fn upload_sdf(&self, _wgpu_state: &WgpuState, _world_renderer: &BigWorldRenderer) {
        todo!()
    }
    pub fn update(&mut self) {
        self.time = self.start_time.elapsed().as_secs_f64();
    }
    pub fn chunk_exists_or_generating(&self, _pos: &MetaChunkPos) -> bool {
        todo!()
    }
}
