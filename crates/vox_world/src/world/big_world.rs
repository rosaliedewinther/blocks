use crate::big_world_renderer::BigWorldRenderer;
use crate::blocks::block::{get_blockid, BlockId};
use crate::blocks::block_type::BlockType;
use crate::player::Player;
use crate::world_gen::chunk::Chunk;
use crate::world_gen::meta_chunk::MetaChunk;
use nalgebra::Vector3;
use noise::{MultiFractal, NoiseFn, Seedable};
use rand::Rng;
use rayon::prelude::ParallelSliceMut;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::ops::Range;
use std::time::Instant;
use vox_core::constants::{BRICKMAPSIZE, BRICKSIZE, METACHUNKSIZE, METACHUNK_GEN_RANGE};
use vox_core::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};
use vox_render::compute_renderer::wgpu_state::WgpuState;

pub struct BigWorld {
    //meta_chunk_locations: [[i32; 3]; 27], //one brickmap which the playes is currently in and all around the player
    brickmap: Box<[u32; BRICKMAPSIZE.pow(3) * 27]>, //assumes brickmaps with size 4^3
    bricks: Vec<[u8; BRICKSIZE.pow(3)]>,            //brick with size 8^3
    pub loading_chunks: HashSet<MetaChunkPos>,
    pub world_seed: u32,
    pub time: f64,
    start_time: Instant,
}

impl BigWorld {
    #[inline]
    pub fn get_block(&self, pos: GlobalBlockPos) -> Option<BlockId> {
        return None;
    }
    pub fn new(seed: u32) -> BigWorld {
        let noise = noise::Fbm::new()
            .set_seed(0)
            .set_octaves(4)
            .set_frequency(6.0);
        let mut rng = rand::thread_rng();

        let mut brickmap: Box<[u32; BRICKMAPSIZE.pow(3) * 27]> =
            vec![0xFFFFFFFFu32; BRICKMAPSIZE.pow(3) * 27]
                .into_boxed_slice()
                .try_into()
                .unwrap();
        let mut bricks = vec![];
        for meta_x in 0..3 {
            for meta_y in 0..3 {
                for meta_z in 0..3 {
                    for brick_x in 0..BRICKMAPSIZE {
                        for brick_y in 0..BRICKMAPSIZE {
                            for brick_z in 0..BRICKMAPSIZE {
                                let mut temp_brick = None;
                                for x in 0..BRICKSIZE {
                                    for y in 0..BRICKSIZE {
                                        for z in 0..BRICKSIZE {
                                            let noise_index = [
                                                (meta_x * BRICKMAPSIZE * BRICKSIZE
                                                    + brick_x * BRICKSIZE
                                                    + x)
                                                    as f64
                                                    / (BRICKMAPSIZE * BRICKSIZE * 3) as f64,
                                                (meta_y * BRICKMAPSIZE * BRICKSIZE
                                                    + brick_y * BRICKSIZE
                                                    + y)
                                                    as f64
                                                    / (BRICKMAPSIZE * BRICKSIZE * 3) as f64,
                                                (meta_z * BRICKMAPSIZE * BRICKSIZE
                                                    + brick_z * BRICKSIZE
                                                    + z)
                                                    as f64
                                                    / (BRICKMAPSIZE * BRICKSIZE * 3) as f64,
                                            ];
                                            if noise.get(noise_index) > 0.3 {
                                                match &mut temp_brick {
                                                    None => {
                                                        let mut b = [0u8; BRICKSIZE.pow(3)];
                                                        b[x + y * BRICKSIZE
                                                            + z * BRICKSIZE * BRICKSIZE] =
                                                            ((z % 8) + 1) as u8;
                                                        temp_brick = Some(b);
                                                    }
                                                    Some(b) => {
                                                        b[x + y * BRICKSIZE
                                                            + z * BRICKSIZE * BRICKSIZE] =
                                                            ((z % 8) + 1) as u8;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if temp_brick.is_some() {
                                    brickmap[meta_x * BRICKMAPSIZE.pow(3)
                                        + meta_y * BRICKMAPSIZE.pow(3) * 3
                                        + meta_z * BRICKMAPSIZE.pow(3) * 9
                                        + brick_x
                                        + brick_y * BRICKMAPSIZE
                                        + brick_z * BRICKMAPSIZE * BRICKMAPSIZE] =
                                        bricks.len() as u32;
                                    bricks.push(temp_brick.unwrap());
                                }
                            }
                        }
                    }
                }
            }
        }
        BigWorld {
            brickmap,
            bricks,
            loading_chunks: HashSet::new(),
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

    pub fn upload_all_brickmaps(&self, wgpu_state: &WgpuState, world_renderer: &BigWorldRenderer) {
        //println!("bricks: {:?}", self.bricks);
        //println!("brickmap: {:?}", self.brickmap);
        println!("bricks len: {:?}", self.bricks.len());
        println!("brickmap len: {:?}", self.brickmap.len());
        for i in 0..self.bricks.len() {
            world_renderer.set_brick(i as u32, &self.bricks[i], wgpu_state);
        }
        for i in 0..27 {
            world_renderer.set_brickmap(i, self.get_slice_of_brickmap(i), wgpu_state);
        }
    }
    fn get_slice_of_brickmap(&self, i: u32) -> &[u32] {
        let s: &[u32] = &self.brickmap
            [(i as usize * BRICKMAPSIZE.pow(3))..((i + 1) as usize * BRICKMAPSIZE.pow(3))];
        return s;
    }
    pub fn update(&mut self) {
        self.time = self.start_time.elapsed().as_secs_f64();
    }
    pub fn chunk_exists_or_generating(&self, pos: &MetaChunkPos) -> bool {
        todo!()
    }
}
