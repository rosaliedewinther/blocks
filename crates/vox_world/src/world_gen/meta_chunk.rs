use crate::algorithms::bfs_world::bfs_world_air;

use crate::blocks::block::{get_blockid, get_blocktype, BlockId};
use crate::blocks::block_type::BlockType;
use crate::player::Player;
use crate::structures::square::place_square;
use crate::structures::tree::place_tree;
use crate::world_gen::basic::ChunkGenerator;
use crate::world_gen::chunk::Chunk;
use rand::distributions::{Distribution, Standard, Uniform};
use rand::prelude::*;
use rand_distr::Normal;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use vox_core::constants::{CHUNKSIZE, METACHUNKSIZE, METACHUNK_GEN_RANGE};
use vox_core::positions::{ChunkPos, GlobalBlockPos, LocalChunkPos, MetaChunkPos};
use vox_core::utils::{to_sign_of, wrap};
use vox_io::io::file_reader::read_meta_chunk_from_file;
use vox_io::io::file_writer::write_to_file;

pub struct MetaChunk {
    chunks: Vec<Chunk<4, 2, 8>>,
    pub pos: MetaChunkPos,
    pub seed: u32,
}

impl MetaChunk {
    pub fn load_or_gen(pos: MetaChunkPos, seed: u32, force_gen: bool) -> MetaChunk {
        let chunk_generator = ChunkGenerator::new(seed);

        let mut chunks: Vec<Chunk<4,2,8>> =
            Vec::with_capacity(METACHUNKSIZE * METACHUNKSIZE * METACHUNKSIZE);
        for z in 0..METACHUNKSIZE {
            for y in 0..METACHUNKSIZE {
                for x in 0..METACHUNKSIZE {
                    let local_pos = &ChunkPos {
                        x: x as i32 + pos.x * METACHUNKSIZE as i32,
                        y: y as i32,
                        z: z as i32 + pos.z * METACHUNKSIZE as i32,
                    };
                    chunks.push(Chunk::generate(&chunk_generator, local_pos));
                }
            }
        }

        let mut chunk = MetaChunk { pos, chunks, seed };

        let structure_x = pos.x * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + 20;
        let structure_z = pos.z * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + 20;
        let structure_y = chunk.first_above_land_y(structure_x, structure_z);
        let global_center_pos = GlobalBlockPos {
            x: structure_x,
            y: structure_y,
            z: structure_z,
        };
        bfs_world_air(
            &global_center_pos,
            5,
            &mut chunk,
            get_blockid(BlockType::Sand),
        );

        let structure_x = pos.x * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + 3;
        let structure_z = pos.z * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + 60;
        let structure_y = chunk.first_above_land_y(structure_x, structure_z);
        let global_center_pos = GlobalBlockPos {
            x: structure_x,
            y: structure_y,
            z: structure_z,
        };
        place_square(&global_center_pos, 10, &mut chunk);

        let mut rng = rand::thread_rng();
        let location_range = Uniform::from(5..(METACHUNKSIZE * CHUNKSIZE) - 5);
        let normal_distribution = Normal::new(0f32, 50f32).unwrap();
        let x_offset = location_range.sample(&mut rng) as i32;
        let z_offset = location_range.sample(&mut rng) as i32;
        for _ in 0..300 {
            let x_diff: i32 = normal_distribution.sample(&mut rng) as i32;
            let z_diff: i32 = normal_distribution.sample(&mut rng) as i32;

            let structure_x = pos.x * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + x_offset + x_diff;
            let structure_z = pos.z * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + z_offset + z_diff;
            let structure_y = chunk.first_above_land_y(structure_x, structure_z);
            let tree_pos = GlobalBlockPos {
                x: structure_x,
                y: structure_y,
                z: structure_z,
            };
            match chunk.get_block(&tree_pos.get_diff(0, -1, 0)) {
                255 => {}
                b => {
                    if get_blocktype(b) == BlockType::Grass {
                        place_tree(&tree_pos, &mut chunk);
                    }
                }
            }
        }

        let structure_x = pos.x * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + pos.x;
        let structure_z = pos.z * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + pos.z;
        for y in chunk.first_above_land_y(structure_x, structure_z)
            ..chunk.first_above_land_y(structure_x, structure_z) + 10
        {
            let global_center_pos = GlobalBlockPos {
                x: structure_x,
                y,
                z: structure_z,
            };
            chunk.set_block(&global_center_pos, get_blockid(BlockType::Sand));
        }

        return chunk;
    }
    pub fn first_above_land_y(&self, x: i32, z: i32) -> i32 {
        let mut y = METACHUNKSIZE as i32 * CHUNKSIZE as i32 - 1;
        while let b = self.get_block(&GlobalBlockPos { x, y, z }) {
            if b == 255{
                return y+1;
            }
            let b_type = get_blocktype(b);
            if b_type == BlockType::Grass
                || b_type == BlockType::Water
                || b_type == BlockType::Dirt
                || b_type == BlockType::Stone
            {
                return y + 1;
            }
            y -= 1;
        }
        return y;
    }

    pub fn set_block(&mut self, pos: &GlobalBlockPos, block: BlockId) {
        let chunk_pos = pos.get_local_chunk();
        let chunk = self.get_chunk_mut(&chunk_pos);
        match chunk {
            Some(c) => c.set_block(block, &pos.get_local_pos()),
            None => {}
        }
    }

    pub fn get_block(&self, pos: &GlobalBlockPos) -> BlockId {
        debug_assert!(pos.x >= self.pos.x * METACHUNKSIZE as i32 * CHUNKSIZE as i32
            && pos.x < (self.pos.x + 1) * METACHUNKSIZE as i32 * CHUNKSIZE as i32
            && pos.z >= self.pos.z * METACHUNKSIZE as i32 * CHUNKSIZE as i32
            && pos.z < (self.pos.z + 1) * METACHUNKSIZE as i32 * CHUNKSIZE as i32);
        let chunk_pos = pos.get_local_chunk();
        let chunk = self.get_chunk(&chunk_pos);
        match chunk {
            Some(c) => c.get_block(&pos.get_local_pos()),
            None => get_blockid(BlockType::Unknown),
        }
    }
    pub fn for_each_mut(&mut self, f: impl Fn(&mut Chunk<4,2,8>, ChunkPos)) {
        for x in 0..METACHUNKSIZE as i32 {
            for y in 0..METACHUNKSIZE as i32 {
                for z in 0..METACHUNKSIZE as i32 {
                    let pos = ChunkPos {
                        x: self.pos.x * METACHUNKSIZE as i32 + x,
                        y,
                        z: self.pos.z * METACHUNKSIZE as i32 + z,
                    };
                    f(self.get_chunk_mut(&LocalChunkPos { x, y, z }).unwrap(), pos);
                }
            }
        }
    }
    pub fn for_each(&self, f: fn(&Chunk<4,2,8>, ChunkPos)) {
        for x in 0..METACHUNKSIZE as i32 {
            for y in 0..METACHUNKSIZE as i32 {
                for z in 0..METACHUNKSIZE as i32 {
                    let pos = ChunkPos {
                        x: self.pos.x * METACHUNKSIZE as i32 + x,
                        y,
                        z: self.pos.z * METACHUNKSIZE as i32 + z,
                    };
                    f(self.get_chunk(&LocalChunkPos { x, y, z }).unwrap(), pos);
                }
            }
        }
    }
    pub fn get_chunk_mut(&mut self, pos: &LocalChunkPos) -> Option<&mut Chunk<4,2,8>> {
        return Some(
            self.chunks[pos.x as usize
                + pos.y as usize * METACHUNKSIZE as usize
                + pos.z as usize * METACHUNKSIZE as usize * METACHUNKSIZE as usize]
                .borrow_mut(),
        );
    }
    pub fn get_chunk(&self, pos: &LocalChunkPos) -> Option<&Chunk<4,2,8>> {
        return Some(
            &self.chunks[pos.x as usize
                + pos.y as usize * METACHUNKSIZE as usize
                + pos.z as usize * METACHUNKSIZE as usize * METACHUNKSIZE as usize],
        );
    }
    #[inline]
    pub fn retain_meta_chunk(player: &Player, pos: MetaChunkPos) -> bool {
        let current_chunk = player.position.get_meta_chunk();
        pos.x > current_chunk.x - METACHUNK_GEN_RANGE as i32 - 2
            && pos.x < current_chunk.x + METACHUNK_GEN_RANGE as i32 + 2
            && pos.z > current_chunk.z - METACHUNK_GEN_RANGE as i32 - 2
            && pos.z < current_chunk.z + METACHUNK_GEN_RANGE as i32 + 2
    }
    pub fn get_chunk_pos(&self, pos: &LocalChunkPos) -> ChunkPos {
        let x = self.pos.x * METACHUNKSIZE as i32
            + wrap(to_sign_of(self.pos.x, pos.x), METACHUNKSIZE as i32);
        let y = pos.y;
        let z = self.pos.z * METACHUNKSIZE as i32
            + wrap(to_sign_of(self.pos.z, pos.z), METACHUNKSIZE as i32);
        ChunkPos { x, y, z }
    }
    pub fn get_iter(&self) -> MetaChunkIterator {
        MetaChunkIterator {
            meta_chunk: &self,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

pub struct MetaChunkIterator<'a> {
    meta_chunk: &'a MetaChunk,
    x: u32,
    y: u32,
    z: u32,
}

impl<'a> Iterator for MetaChunkIterator<'a> {
    type Item = (&'a Chunk<4,2,8>, ChunkPos);

    fn next(&mut self) -> Option<(&'a Chunk<4,2,8>, ChunkPos)> {
        if self.x == (METACHUNKSIZE - 1) as u32
            && self.y == (METACHUNKSIZE - 1) as u32
            && self.z == (METACHUNKSIZE - 1) as u32
        {
            return None;
        }
        let pos = ChunkPos {
            x: self.meta_chunk.pos.x * METACHUNKSIZE as i32 + self.x as i32,
            y: self.y as i32,
            z: self.meta_chunk.pos.z * METACHUNKSIZE as i32 + self.z as i32,
        };
        let c = self
            .meta_chunk
            .get_chunk(&LocalChunkPos {
                x: self.x as i32,
                y: self.y as i32,
                z: self.z as i32,
            })
            .unwrap();

        if self.x == (METACHUNKSIZE) as u32 {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }
        if self.y == (METACHUNKSIZE) as u32 {
            self.y = 0;
            self.z += 1;
        }

        return Some((c, pos));
    }
}
