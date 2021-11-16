use crate::blocks::block_type::BlockType;
use crate::blocks::blockside::BlockSides;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use vox_core::constants::COLORS;
use vox_core::positions::{GlobalBlockPos, ObjectPos};
use vox_render::renderer::vertex::{vertex, vertex_typed, Vertex};

pub type BlockId = u8;

#[inline]
pub const fn get_blocktype(block_id: BlockId) -> BlockType {
    match block_id {
        0 => BlockType::Grass,
        1 => BlockType::Water,
        2 => BlockType::Dirt,
        3 => BlockType::Stone,
        4 => BlockType::Sand,
        5 => BlockType::Air,
        6 => BlockType::Leaf,
        _ => BlockType::Unknown,
    }
}
#[inline]
pub const fn get_blockid(block: BlockType) -> BlockId {
    match block {
        BlockType::Grass => 0,
        BlockType::Water => 1,
        BlockType::Dirt => 2,
        BlockType::Stone => 3,
        BlockType::Sand => 4,
        BlockType::Air => 5,
        BlockType::Leaf => 6,
        BlockType::Unknown => 255,
    }
}
#[inline]
pub fn should_render_against(source_block_id: BlockId, neighbor_block_id: BlockId) -> bool {
    if source_block_id == neighbor_block_id {
        return false;
    }
    if source_block_id == get_blockid(BlockType::Leaf) {
        return true;
    }
    if COLORS[neighbor_block_id as usize][3] == 255.0 {
        return false;
    }
    return true;
}