use crate::algorithms::bfs_world::bfs_world_air;
use crate::block::{Block, BlockSides, BlockType};
use crate::constants::{CHUNKSIZE, METACHUNKSIZE};
use crate::io::file_reader::read_meta_chunk_from_file;
use crate::io::file_writer::write_to_file;
use crate::player::Player;
use crate::positions::{ChunkPos, GlobalBlockPos, LocalBlockPos, LocalChunkPos, MetaChunkPos};
use crate::renderer::chunk_render_data::ChunkRenderData;
use crate::renderer::vertex::Vertex;
use crate::structures::square::place_square;
use crate::structures::tree::place_tree;
use crate::utils::{to_sign_of, wrap};
use crate::world_gen::basic::{
    floodfill_water, generate_empty_chunk, generate_landmass, ChunkGenerator,
};
use crate::world_gen::chunk::Chunk;
use rand::distributions::{Distribution, Uniform};
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use wgpu::Device;

#[derive(Serialize, Deserialize)]
pub struct MetaChunk {
    pub chunks: Vec<Vec<Vec<Chunk>>>,
    pub pos: MetaChunkPos,
    pub seed: u32,
}

impl MetaChunk {
    pub fn load_or_gen(pos: MetaChunkPos, seed: u32, force_gen: bool) -> MetaChunk {
        if !force_gen {
            let loaded = MetaChunk::load_from_disk(&pos);
            if loaded.is_some() {
                return loaded.unwrap();
            }
        }
        let chunk_generator = ChunkGenerator::new();

        let mut chunks: Vec<Vec<Vec<Chunk>>> = Vec::with_capacity(METACHUNKSIZE);
        for x in 0..METACHUNKSIZE {
            chunks.push(Vec::new());
            for y in 0..METACHUNKSIZE {
                chunks[x].push(Vec::new());
                for z in 0..METACHUNKSIZE {
                    let local_pos = &ChunkPos {
                        x: x as i32 + pos.x * METACHUNKSIZE as i32,
                        y: y as i32,
                        z: z as i32 + pos.z * METACHUNKSIZE as i32,
                    };
                    chunks[x][y].push(chunk_generator.full_generation_pass(local_pos));
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
            Block::new(BlockType::Sand),
        );

        let structure_x = pos.x * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + 3;
        let structure_z = pos.z * METACHUNKSIZE as i32 * CHUNKSIZE as i32 + 60;
        let structure_y = chunk.first_above_land_y(structure_x, structure_z);
        println!("{}", structure_y);
        let global_center_pos = GlobalBlockPos {
            x: structure_x,
            y: structure_y,
            z: structure_z,
        };
        place_square(&global_center_pos, 10, &mut chunk);

        let mut rng = rand::thread_rng();
        let location_range = Uniform::from(0..(METACHUNKSIZE * CHUNKSIZE));
        for _ in 0..1000 {
            let structure_x = pos.x * METACHUNKSIZE as i32 * CHUNKSIZE as i32
                + location_range.sample(&mut rng) as i32;
            let structure_z = pos.z * METACHUNKSIZE as i32 * CHUNKSIZE as i32
                + location_range.sample(&mut rng) as i32;
            let structure_y = chunk.first_above_land_y(structure_x, structure_z);
            let tree_pos = GlobalBlockPos {
                x: structure_x,
                y: structure_y,
                z: structure_z,
            };
            if chunk
                .get_block(&tree_pos.get_diff(0, -1, 0))
                .unwrap()
                .block_type
                == BlockType::Grass
            {
                place_tree(&tree_pos, &mut chunk);
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
            chunk.set_block(&global_center_pos, Block::new(BlockType::Sand));
        }

        return chunk;
    }
    pub fn first_above_land_y(&self, x: i32, z: i32) -> i32 {
        let mut y = METACHUNKSIZE as i32 * CHUNKSIZE as i32 - 1;
        while let Some(b) = self.get_block(&GlobalBlockPos { x, y, z }) {
            if b.block_type == BlockType::Grass
                || b.block_type == BlockType::Water
                || b.block_type == BlockType::Dirt
                || b.block_type == BlockType::Stone
            {
                return y + 1;
            }
            y -= 1;
        }
        return y;
    }

    pub fn load_from_disk(pos: &MetaChunkPos) -> Option<MetaChunk> {
        let filename = format!("{}-{}.txt", pos.x, pos.z);
        return read_meta_chunk_from_file(filename.as_str());
    }

    pub fn save_to_disk(&self) {
        let filename = format!("{}-{}.txt", self.pos.x, self.pos.z);
        write_to_file(filename.as_str(), self)
    }

    pub fn set_block(&mut self, pos: &GlobalBlockPos, block: Block) {
        let chunk_pos = pos.get_local_chunk();
        let chunk = self.get_chunk_mut(&chunk_pos);
        match chunk {
            Some(c) => c.set_block(block, &pos.get_local_pos()),
            None => {}
        }
    }
    pub fn get_block(&self, pos: &GlobalBlockPos) -> Option<&Block> {
        if !(pos.x >= self.pos.x * METACHUNKSIZE as i32 * CHUNKSIZE as i32
            && pos.x < (self.pos.x + 1) * METACHUNKSIZE as i32 * CHUNKSIZE as i32
            && pos.z >= self.pos.z * METACHUNKSIZE as i32 * CHUNKSIZE as i32
            && pos.z < (self.pos.z + 1) * METACHUNKSIZE as i32 * CHUNKSIZE as i32)
        {
            return None;
        }
        let chunk_pos = pos.get_local_chunk();
        let chunk = self.get_chunk(&chunk_pos);
        match chunk {
            Some(c) => c.get_block(&pos.get_local_pos()),
            None => None,
        }
    }
    pub fn for_each_mut(&mut self, f: impl Fn(&mut Chunk, ChunkPos)) {
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
    pub fn for_each(&self, f: impl Fn(&Chunk, ChunkPos)) {
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
    pub fn get_chunk_mut(&mut self, pos: &LocalChunkPos) -> Option<&mut Chunk> {
        return Some(self.chunks[pos.x as usize][pos.y as usize][pos.z as usize].borrow_mut());
    }
    pub fn get_chunk(&self, pos: &LocalChunkPos) -> Option<&Chunk> {
        return Some(&self.chunks[pos.x as usize][pos.y as usize][pos.z as usize]);
    }
    pub fn get_chunk_pos(&self, pos: &LocalChunkPos) -> ChunkPos {
        let x = self.pos.x * METACHUNKSIZE as i32
            + wrap(to_sign_of(self.pos.x, pos.x), METACHUNKSIZE as i32);
        let y = pos.y;
        let z = self.pos.z * METACHUNKSIZE as i32
            + wrap(to_sign_of(self.pos.z, pos.z), METACHUNKSIZE as i32);
        ChunkPos { x, y, z }
    }
}
