use crate::blocks::block::{get_blocktype, BlockId};
use crate::blocks::block_type::BlockType;
use crate::blocks::blockside::BlockSides;
use vox_core::constants::COLORS;
use vox_core::positions::{GlobalBlockPos, ObjectPos};
use vox_render::renderer::vertex::{vertex_typed, Vertex};
use vox_render::renderer::normals::Normals;
use arrayvec::ArrayVec;

pub fn get_mesh(
    block_id: BlockId,
    pos: &GlobalBlockPos,
    sides: &BlockSides,
) -> (ArrayVec<Vertex, 24>, ArrayVec<u32, 36>) {
    let mut vertices = ArrayVec::<Vertex, 24>::new();
    let mut indices = ArrayVec::<u32, 36>::new();
    if get_blocktype(block_id) == BlockType::Air {
        return (ArrayVec::new(), ArrayVec::new());
    }
    let posf = pos.get_block_pos();
    if sides.right {
        mesh_right(block_id, &posf,&mut vertices, &mut indices);
    }
    if sides.left {
        mesh_left(block_id, &posf, &mut vertices, &mut indices);
    }
    if sides.top {
        mesh_top(block_id, &posf, &mut vertices, &mut indices);
    }
    if sides.bot {
        mesh_bottom(block_id, &posf, &mut vertices, &mut indices);
    }
    if sides.back {
        mesh_back(block_id, &posf, &mut vertices, &mut indices);
    }
    if sides.front {
        mesh_front(block_id, &posf, &mut vertices, &mut indices);
    }
    return (vertices, indices);
}
#[inline]
fn mesh_front(block_id: BlockId, pos: &ObjectPos, vec: &mut ArrayVec<Vertex, 24>, indices: &mut ArrayVec<u32, 36>) {
    indices.push((vec.len() + 0) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 2) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 3) as u32);
    indices.push((vec.len() + 2) as u32);
    vec.push(vertex_typed(
        [pos.x, pos.y, pos.z],
        block_id as u32,
        Normals::Front as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y, pos.z],
        block_id as u32,
        Normals::Front as u32
    ));
    vec.push(vertex_typed(
        [pos.x, pos.y + 1f32, pos.z],
        block_id as u32,
        Normals::Front as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y + 1f32, pos.z],
        block_id as u32,
        Normals::Front as u32
    ));
}
#[inline]
fn mesh_back(block_id: BlockId, pos: &ObjectPos, vec: &mut ArrayVec<Vertex, 24>, indices: &mut ArrayVec<u32, 36>) {
    indices.push((vec.len() + 0) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 2) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 3) as u32);
    indices.push((vec.len() + 2) as u32);
    vec.push(vertex_typed(
        [pos.x, pos.y, pos.z + 1f32],
        block_id as u32,
        Normals::Back as u32
    ));
    vec.push(vertex_typed(
        [pos.x, pos.y + 1f32, pos.z + 1f32],
        block_id as u32,
        Normals::Back as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y, pos.z + 1f32],
        block_id as u32,
        Normals::Back as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
        block_id as u32,
        Normals::Back as u32
    ));
}
#[inline]
fn mesh_left(block_id: BlockId, pos: &ObjectPos, vec: &mut ArrayVec<Vertex, 24>, indices: &mut ArrayVec<u32, 36>) {
    indices.push((vec.len() + 0) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 2) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 3) as u32);
    indices.push((vec.len() + 2) as u32);
    vec.push(vertex_typed(
        [pos.x, pos.y, pos.z],
        block_id as u32,
        Normals::Left as u32
    ));
    vec.push(vertex_typed(
        [pos.x, pos.y + 1f32, pos.z],
        block_id as u32,
        Normals::Left as u32
    ));
    vec.push(vertex_typed(
        [pos.x, pos.y, pos.z + 1f32],
        block_id as u32,
        Normals::Left as u32
    ));
    vec.push(vertex_typed(
        [pos.x, pos.y + 1f32, pos.z + 1f32],
        block_id as u32,
        Normals::Left as u32
    ));
}
#[inline]
fn mesh_right(block_id: BlockId, pos: &ObjectPos, vec: &mut ArrayVec<Vertex, 24>, indices: &mut ArrayVec<u32, 36>) {
    indices.push((vec.len() + 0) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 2) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 3) as u32);
    indices.push((vec.len() + 2) as u32);
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y, pos.z],
        block_id as u32,
        Normals::Right as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y, pos.z + 1f32],
        block_id as u32,
        Normals::Right as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y + 1f32, pos.z],
        block_id as u32,
        Normals::Right as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
        block_id as u32,
        Normals::Right as u32
    ));
}
#[inline]
fn mesh_top(block_id: BlockId, pos: &ObjectPos, vec: &mut ArrayVec<Vertex, 24>, indices: &mut ArrayVec<u32, 36>) {
    indices.push((vec.len() + 0) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 2) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 3) as u32);
    indices.push((vec.len() + 2) as u32);
    vec.push(vertex_typed(
        [pos.x, pos.y + 1f32, pos.z],
        block_id as u32,
        Normals::Up as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y + 1f32, pos.z],
        block_id as u32,
        Normals::Up as u32
    ));
    vec.push(vertex_typed(
        [pos.x, pos.y + 1f32, pos.z + 1f32],
        block_id as u32,
        Normals::Up as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y + 1f32, pos.z + 1f32],
        block_id as u32,
        Normals::Up as u32
    ));
}
#[inline]
fn mesh_bottom(block_id: BlockId, pos: &ObjectPos, vec: &mut ArrayVec<Vertex, 24>, indices: &mut ArrayVec<u32, 36>) {
    indices.push((vec.len() + 0) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 2) as u32);
    indices.push((vec.len() + 1) as u32);
    indices.push((vec.len() + 3) as u32);
    indices.push((vec.len() + 2) as u32);
    vec.push(vertex_typed(
        [pos.x, pos.y, pos.z],
        block_id as u32,
        Normals::Down as u32
    ));
    vec.push(vertex_typed(
        [pos.x, pos.y, pos.z + 1f32],
        block_id as u32,
        Normals::Down as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y, pos.z],
        block_id as u32,
        Normals::Down as u32
    ));
    vec.push(vertex_typed(
        [pos.x + 1f32, pos.y, pos.z + 1f32],
        block_id as u32,
        Normals::Down as u32
    ));
}
