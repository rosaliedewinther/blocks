use crate::block::{Block, BlockType};
use crate::positions::GlobalBlockPos;
use crate::world::World;
use crate::world_gen::meta_chunk::MetaChunk;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

pub struct Blocksides {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}
impl Blocksides {
    pub fn new() -> Blocksides {
        Blocksides {
            top: false,
            bottom: false,
            left: false,
            right: false,
            front: false,
            back: false,
        }
    }
}

pub fn bfs_world_air(
    pos: &GlobalBlockPos,
    depth: u32,
    world: &mut MetaChunk,
    f: impl Fn(&Block) -> Block,
) {
    let time = Instant::now();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let sides = get_surrounding_blocks(world, pos, |b: &Block| b.block_type == BlockType::Air);
    push_sides(&mut queue, &visited, &sides, pos, 0);
    visited.insert(*pos);
    while let Some((temp_pos, d)) = queue.pop_front() {
        if d == depth {
            if let Some(b) = world.get_block(&temp_pos) {
                world.set_block(&temp_pos, f(b));
            }
            continue;
        }
        let sides =
            get_surrounding_blocks(world, &temp_pos, |b: &Block| b.block_type == BlockType::Air);
        push_sides(&mut queue, &visited, &sides, &temp_pos, d + 1);
        visited.insert(temp_pos);
    }
    println!("bfs took: {}", time.elapsed().as_secs_f64());
}

fn push_sides(
    queue: &mut VecDeque<(GlobalBlockPos, u32)>,
    visited: &HashSet<GlobalBlockPos>,
    sides: &Blocksides,
    pos: &GlobalBlockPos,
    depth: u32,
) {
    if sides.right {
        let p = pos.get_diff(1, 0, 0);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.left {
        let p = pos.get_diff(-1, 0, 0);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.top {
        let p = pos.get_diff(0, 1, 0);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.bottom {
        let p = pos.get_diff(0, -1, 0);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.front {
        let p = pos.get_diff(0, 0, 1);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.back {
        let p = pos.get_diff(0, 0, -1);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
}

fn get_surrounding_blocks(
    world: &MetaChunk,
    pos: &GlobalBlockPos,
    f: impl Fn(&Block) -> bool,
) -> Blocksides {
    let mut sides = Blocksides::new();
    update_side(&world, &pos.get_diff(1, 0, 0), &f, &mut sides.right);
    update_side(&world, &pos.get_diff(-1, 0, 0), &f, &mut sides.left);
    update_side(&world, &pos.get_diff(0, 1, 0), &f, &mut sides.top);
    update_side(&world, &pos.get_diff(0, -1, 0), &f, &mut sides.bottom);
    update_side(&world, &pos.get_diff(0, 0, 1), &f, &mut sides.front);
    update_side(&world, &pos.get_diff(0, 0, -1), f, &mut sides.back);
    return sides;
}

fn update_side(
    world: &MetaChunk,
    pos: &GlobalBlockPos,
    f: impl Fn(&Block) -> bool,
    mut side: &mut bool,
) {
    let b = world.get_block(&pos);
    match b {
        Some(block) => {
            if f(block) {
                *side = true;
            }
        }
        _ => {}
    }
}
