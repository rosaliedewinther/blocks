use crate::algorithms::noise_abstraction::Noise;
use crate::big_world_renderer::BigWorldRenderer;
use crate::blocks::block::{get_blockid, BlockId};
use crate::blocks::block_type::BlockType;
use crate::player::Player;
use crate::world_gen::chunk::Chunk;
use crate::world_gen::meta_chunk::MetaChunk;
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
use vox_core::constants::{
    BRICKMAPSIZE, BRICKSIZE, MAX_AMOUNT_OF_BRICKS, METACHUNKSIZE, METACHUNK_GEN_RANGE,
};
use vox_core::positions::{ChunkPos, GlobalBlockPos, MetaChunkPos};
use vox_io::io::file_reader::read_struct_from_file;
use vox_io::io::file_writer::write_struct_to_file;
use vox_render::compute_renderer::wgpu_state::WgpuState;

pub struct BigWorld {
    //meta_chunk_locations: [[i32; 3]; 27], //one brickmap which the playes is currently in and all around the player
    brickmap: Box<[u32; BRICKMAPSIZE.pow(3) * 27]>, //assumes brickmaps with size 4^3
    bricks: Box<[[u8; BRICKSIZE.pow(3)]; MAX_AMOUNT_OF_BRICKS as usize]>, //brick with size 8^3
    pub loading_chunks: HashSet<MetaChunkPos>,
    pub world_seed: u32,
    pub time: f64,
    start_time: Instant,
    amount_of_bricks: u32,
}

impl BigWorld {
    #[inline]
    pub fn get_block(&self, pos: GlobalBlockPos) -> Option<BlockId> {
        return None;
    }
    pub fn new<T: Noise>(seed: u32) -> BigWorld {
        let noise = T::new(2, 0, 6.0);

        let mut brickmap: Box<[u32; BRICKMAPSIZE.pow(3) * 27]> =
            vec![0xFFFFFFFFu32; BRICKMAPSIZE.pow(3) * 27]
                .into_boxed_slice()
                .try_into()
                .unwrap();
        let mut bricks: Box<[[u8; BRICKSIZE.pow(3)]; MAX_AMOUNT_OF_BRICKS as usize]> =
            vec![[0u8; BRICKSIZE.pow(3)]; MAX_AMOUNT_OF_BRICKS as usize]
                .into_boxed_slice()
                .try_into()
                .unwrap();
        let mut brick_index = 0;
        for meta_x in 0..3 {
            for meta_y in 0..3 {
                for meta_z in 0..3 {
                    warn!("working on metachunk: {} {} {}", meta_x, meta_y, meta_z);
                    for brick_x in 0..BRICKMAPSIZE {
                        for brick_y in 0..BRICKMAPSIZE {
                            for brick_z in 0..BRICKMAPSIZE {
                                let mut temp_brick: Option<[u8; BRICKSIZE.pow(3)]> = None;
                                match temp_brick {
                                    None => {}
                                    Some(b) => {
                                        brickmap[meta_x * BRICKMAPSIZE.pow(3)
                                            + meta_y * BRICKMAPSIZE.pow(3) * 3
                                            + meta_z * BRICKMAPSIZE.pow(3) * 9
                                            + brick_x
                                            + brick_y * BRICKMAPSIZE
                                            + brick_z * BRICKMAPSIZE * BRICKMAPSIZE] = brick_index;
                                        bricks[brick_index as usize] = b;
                                        brick_index += 1;
                                    }
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
            amount_of_bricks: brick_index,
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
        warn!("bricks len: {:?}", self.amount_of_bricks);
        warn!("brickmap len: {:?}", self.brickmap.len());
        for i in 0..self.amount_of_bricks {
            world_renderer.set_brick(i as u32, &self.bricks[i as usize], wgpu_state);
        }
        for i in 0..27 {
            world_renderer.set_brickmap(i, self.get_slice_of_brickmap(i), wgpu_state);
        }
        warn!("queued all GPU uploads");
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
