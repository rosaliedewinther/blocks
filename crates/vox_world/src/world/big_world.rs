use crate::algorithms::noise_abstraction::Noise;
use crate::algorithms::noise_default::NoiseDefault;
use crate::big_world_renderer::BigWorldRenderer;
use crate::blocks::block::{get_blockid, BlockId};
use crate::blocks::block_type::BlockType;
use crate::player::Player;
use crate::world_gen::chunk::Chunk;
use crate::world_gen::generator::WorldGenerator;
use crate::world_gen::hills::HillsWorldGenerator;
use crate::world_gen::meta_chunk::MetaChunk;
use crate::world_gen::standard::StandardWorldGenerator;
use log::warn;
use nalgebra::Vector3;
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSliceMut;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::ops::Range;
use std::path::Path;
use std::sync::{RwLock, RwLockWriteGuard};
use std::time::Instant;
use vox_core::constants::{METACHUNKSIZE, METACHUNK_GEN_RANGE, WORLD_SIZE};
use vox_core::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};
use vox_io::io::file_reader::read_struct_from_file;
use vox_io::io::file_writer::write_struct_to_file;
use vox_render::compute_renderer::wgpu_state::WgpuState;

pub struct BigWorld {
    world: Box<[BlockId]>,
    pub world_seed: u32,
    pub time: f64,
    start_time: Instant,
}

impl BigWorld {
    #[inline]
    pub fn get_block(&self, pos: GlobalBlockPos) -> Option<BlockId> {
        return None;
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
    pub fn set_block(&mut self, block: u8, pos: GlobalBlockPos) {
        todo!()
    }
    pub fn filter_chunks(&mut self, player: &Player) {
        todo!()
    }
    pub fn upload_world(&self, wgpu_state: &WgpuState, world_renderer: &BigWorldRenderer) {
        world_renderer.upload_world(&self.world, wgpu_state);
        world_renderer.rebuild_sdf(wgpu_state);
    }
    pub fn upload_sdf(&self, wgpu_state: &WgpuState, world_renderer: &BigWorldRenderer) {
        todo!()
    }
    pub fn update(&mut self) {
        self.time = self.start_time.elapsed().as_secs_f64();
    }
    pub fn chunk_exists_or_generating(&self, pos: &MetaChunkPos) -> bool {
        todo!()
    }
}
