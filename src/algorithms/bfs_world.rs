use crate::block::{Block, BlockType};
use crate::positions::GlobalBlockPos;
use crate::world_gen::meta_chunk::MetaChunk;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

pub struct Blocksides {
    pub top: bool,
    pub top_left: bool,
    pub top_right: bool,
    pub top_front: bool,
    pub top_back: bool,
    pub bottom: bool,
    pub bottom_left: bool,
    pub bottom_right: bool,
    pub bottom_front: bool,
    pub bottom_back: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}
impl Blocksides {
    pub fn new() -> Blocksides {
        Blocksides {
            top: false,
            top_left: false,
            top_right: false,
            top_front: false,
            top_back: false,
            bottom: false,
            bottom_left: false,
            bottom_right: false,
            bottom_front: false,
            bottom_back: false,
            left: false,
            right: false,
            front: false,
            back: false,
        }
    }
}

pub fn bfs_world_air(pos: &GlobalBlockPos, depth: u32, world: &mut MetaChunk, block: Block) {
    let time = Instant::now();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let sides = get_surrounding_blocks(world, pos, |b: &Block| b.block_type == BlockType::Air);
    push_sides(&mut queue, &visited, &sides, pos, 0);
    visited.insert(*pos);
    while let Some((temp_pos, d)) = queue.pop_front() {
        if d == depth {
            world.set_block(&temp_pos, block);
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
    if sides.top_left {
        let p = pos.get_diff(-1, 1, 0);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.top_right {
        let p = pos.get_diff(1, 1, 0);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.top_front {
        let p = pos.get_diff(0, 1, 1);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.top_back {
        let p = pos.get_diff(0, 1, -1);
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
    if sides.bottom_left {
        let p = pos.get_diff(-1, -1, 0);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.bottom_right {
        let p = pos.get_diff(1, -1, 0);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.bottom_front {
        let p = pos.get_diff(0, -1, 1);
        if !visited.contains(&p) {
            queue.push_back((p, depth));
        }
    }
    if sides.bottom_back {
        let p = pos.get_diff(0, -1, 1);
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
    update_side(&world, &pos.get_diff(-1, 1, 0), &f, &mut sides.top_left);
    update_side(&world, &pos.get_diff(1, 1, 0), &f, &mut sides.top_right);
    update_side(&world, &pos.get_diff(0, 1, 1), &f, &mut sides.top_front);
    update_side(&world, &pos.get_diff(0, 1, -1), &f, &mut sides.top_back);
    update_side(&world, &pos.get_diff(0, -1, 0), &f, &mut sides.bottom);
    update_side(&world, &pos.get_diff(-1, -1, 0), &f, &mut sides.bottom_left);
    update_side(&world, &pos.get_diff(1, -1, 0), &f, &mut sides.bottom_right);
    update_side(&world, &pos.get_diff(0, -1, 1), &f, &mut sides.bottom_front);
    update_side(&world, &pos.get_diff(0, -1, -1), &f, &mut sides.bottom_back);
    update_side(&world, &pos.get_diff(0, 0, 1), &f, &mut sides.front);
    update_side(&world, &pos.get_diff(0, 0, -1), f, &mut sides.back);
    return sides;
}

fn update_side(
    world: &MetaChunk,
    pos: &GlobalBlockPos,
    f: impl Fn(&Block) -> bool,
    side: &mut bool,
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
