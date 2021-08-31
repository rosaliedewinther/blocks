use crate::blocks::block_type::BlockType;

pub type BlockId = u8;

#[inline]
pub fn get_blocktype(block_id: BlockId) -> BlockType {
    match block_id {
        0 => BlockType::Air,
        1 => BlockType::Water,
        2 => BlockType::Dirt,
        3 => BlockType::Stone,
        4 => BlockType::Sand,
        5 => BlockType::Leaf,
        6 => BlockType::Grass,
        _ => BlockType::Unknown,
    }
}
#[inline]
pub fn get_blockid(block: BlockType) -> BlockId {
    match block {
        BlockType::Air => 0,
        BlockType::Water => 1,
        BlockType::Dirt => 2,
        BlockType::Stone => 3,
        BlockType::Sand => 4,
        BlockType::Leaf => 5,
        BlockType::Grass => 6,
        BlockType::Unknown => 255,
    }
}
