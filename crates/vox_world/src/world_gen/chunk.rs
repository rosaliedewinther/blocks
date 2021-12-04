use crate::blocks::block::{get_blockid, get_blocktype, BlockId};
use crate::blocks::block_type::BlockType;
use crate::world_gen::basic::ChunkGenerator;
use arrayvec::ArrayVec;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use serde_big_array::big_array;
use vox_core::constants::CHUNKSIZE;
use vox_core::positions::{ChunkPos, LocalBlockPos};
use vox_core::utils::coord_to_array_indice;

#[derive(Copy, Clone, Debug, PartialEq)]
enum BlockIdOrPointer {
    Id(BlockId),
    Ptr(u32),
}

pub struct OldChunk {
    pub data: Vec<BlockId>,
}

#[derive(Debug)]
pub struct Chunk<const Depth: usize, const BlockSize: usize, const BlockSizeCubed: usize> {
    data_structure: [Vec<[BlockIdOrPointer; BlockSizeCubed]>; Depth], //array of layers where every layer has a vector with sparse data, and all sparse data has a ptr to its owner in the layer above
}

impl<const Depth: usize, const BlockSize: usize, const BlockSizeCubed: usize>
    Chunk<Depth, BlockSize, BlockSizeCubed>
{
    fn what_index(pos: &LocalBlockPos, depth: usize) -> usize {
        let width = BlockSize.pow((Depth - depth) as u32) as u32;
        let index = coord_to_array_indice(
            (pos.x as u32 % width) / (width / BlockSize as u32),
            (pos.y as u32 % width) / (width / BlockSize as u32),
            (pos.z as u32 % width) / (width / BlockSize as u32),
            BlockSize as u32,
        );
        return index;
    }
    pub fn generate(chunk_generator: &ChunkGenerator, pos: &ChunkPos) -> Self {
        debug_assert!(BlockSize == 4 || BlockSize == 2 || BlockSize == 8);
        debug_assert!(BlockSize.pow(3) == BlockSizeCubed);
        let mut c = Self {
            data_structure: array_init::array_init(|i| {
                return if i == 0 {
                    vec![[BlockIdOrPointer::Id(get_blockid(BlockType::Air)); BlockSizeCubed]; 1]
                } else {
                    Vec::new()
                };
            }),
        };

        return c;
    }
    pub fn get_structure_size(&self) -> usize {
        let mut size = 0;
        size += std::mem::size_of::<[Vec<[BlockIdOrPointer; BlockSizeCubed]>; Depth]>();
        println!("sizeof base arrary: {}", size);
        for i in 0..Depth {
            if self.data_structure[i].len() > 0 {
                size += self.data_structure[i].len()
                    * std::mem::size_of::<([BlockIdOrPointer; BlockSizeCubed], u32)>();
                println!(
                    "sizeof layer {}: {}",
                    i,
                    self.data_structure[i].len()
                        * std::mem::size_of::<([BlockIdOrPointer; BlockSizeCubed], u32)>()
                );
            }
        }
        size
    }
    pub fn print_structured(&self) {
        for i in 0..self.data_structure.len() {
            println!(
                "layer {} contains {} bricks:",
                i,
                self.data_structure[i].len()
            );
            for j in 0..self.data_structure[i].len() {
                print!("  brick {}: ", j,);
                for k in 0..self.data_structure[i][j].len() {
                    print!("\t{:?}", self.data_structure[i][j][k]);
                }
                println!();
            }
        }
    }

    pub fn update(&mut self, _dt: f32) -> bool {
        return false;
    }

    pub fn set_block(&mut self, block: BlockId, pos: &LocalBlockPos) {
        debug_assert!(
            pos.x >= 0
                || pos.x <= (CHUNKSIZE - 1) as i32
                || pos.y >= 0
                || pos.y <= (CHUNKSIZE - 1) as i32
                || pos.z >= 0
                || pos.z <= (CHUNKSIZE - 1) as i32
        );

        let mut ptrs = [0usize; Depth];
        for i in 0..Depth as usize {
            let index = Chunk::<Depth, BlockSize, BlockSizeCubed>::what_index(pos, i);
            let r: BlockIdOrPointer = self.data_structure[i][ptrs[i]][index];
            if let BlockIdOrPointer::Id(id) = r {
                if block == id {
                    //if entire chunk is already block, all is good
                    return;
                } else {
                    if i == (Depth - 1) {
                        // if we are at the deepest possible layer, just change the block
                        self.data_structure[i][ptrs[i]][index] = BlockIdOrPointer::Id(block);
                        self.fix_tree_upwards(i, ptrs, pos);
                        return;
                    } else {
                        // we need to add a new layer of leaf nodes
                        let new_ptr = self.data_structure[i + 1].len();
                        self.data_structure[i + 1].push([r; BlockSizeCubed]);
                        self.data_structure[i][ptrs[i]][index] =
                            BlockIdOrPointer::Ptr(new_ptr as u32);
                        ptrs[i + 1] = new_ptr;
                    }
                }
            } else if let BlockIdOrPointer::Ptr(pointer) = r {
                ptrs[i + 1] = pointer as usize;
            }
        }
        panic!("error in chunk datastructure");
    }
    fn fix_tree_upwards(&mut self, depth: usize, ptrs: [usize; Depth], pos: &LocalBlockPos) {
        for j in (1..depth + 1).rev() {
            //iterate back up to find complete chunks
            self.print_structured();
            if self.data_structure[j][ptrs[depth]]
                .iter()
                .all(|x| *x == self.data_structure[j][ptrs[depth]][0])
            {
                //if all chunks in layer j are the same
                if let BlockIdOrPointer::Id(b) = self.data_structure[j][ptrs[depth]][0] {
                    //get the homogenious block
                    let parent_ptr = ptrs[j - 1];
                    self.data_structure[j].remove(ptrs[j]); //remove the entire chunk from layer
                    let index = Chunk::<Depth, BlockSize, BlockSizeCubed>::what_index(pos, j - 1); //get index for removed chunk for layer j-1
                    self.data_structure[j - 1][parent_ptr][index] = BlockIdOrPointer::Id(b); //set this ptr to be a block instead

                    for brick_i in 0..self.data_structure[j - 1].len() {
                        for possible_pointer in 0..self.data_structure[j - 1][brick_i].len() {
                            if let BlockIdOrPointer::Ptr(x) =
                                self.data_structure[j - 1][brick_i][possible_pointer]
                            {
                                if x > ptrs[depth] as u32 {
                                    self.data_structure[j - 1][brick_i][possible_pointer] =
                                        BlockIdOrPointer::Ptr(x - 1);
                                }
                            }
                        }
                    }
                } else {
                    panic!("error in chunk datastructure");
                }
            } else {
                return;
            }
        }
    }

    pub fn get_block(&self, pos: &LocalBlockPos) -> BlockId {
        debug_assert!(
            pos.x >= 0
                || pos.x <= (CHUNKSIZE - 1) as i32
                || pos.y >= 0
                || pos.y <= (CHUNKSIZE - 1) as i32
                || pos.z >= 0
                || pos.z <= (CHUNKSIZE - 1) as i32
        );

        let mut ptr: usize = 0;
        for i in 0..Depth as i32 {
            match self.data_structure[i as usize][ptr]
                [Chunk::<Depth, BlockSize, BlockSizeCubed>::what_index(pos, i as usize)]
            {
                BlockIdOrPointer::Id(id) => return id,
                BlockIdOrPointer::Ptr(p) => {
                    ptr = p as usize;
                }
            }
        }
        panic!("error in chunk datastructure");
    }

    /*pub fn set_block(&mut self, block: BlockId, pos: &LocalBlockPos) {
        debug_assert!(pos.x < 0
            || pos.x > (CHUNKSIZE - 1) as i32
            || pos.y < 0
            || pos.y > (CHUNKSIZE - 1) as i32
            || pos.z < 0
            || pos.z > (CHUNKSIZE - 1) as i32);
        match self.single_block{
            255 => {
                self.set_block_l1(block, pos);
                if self.top_layer.iter().all(|e| e == BlockIdOrPointer::Id && e == self.top_layer[0]){
                    match self.top_layer[0]{
                        BlockIdOrPointer::Id(id) => self.single_block = id,
                        BlockIdOrPointer::Ptr(_) => panic!("something went horribly wrong when setting a block")
                    }
                }
            },
            b => {
                if b == block{
                    return
                } else {
                    self.top_layer = [BlockIdOrPointer::Id(*b);64];
                    self.single_block = get_blockid(BlockType::Unknown);
                    self.set_block_l1(block, pos);

                }
            }
        }
    }
    pub fn set_block_l1(&mut self, block: BlockId, pos: &LocalBlockPos){
        let index = Chunk::what_index_top_layer(pos);
        match self.top_layer[index] {
            BlockIdOrPointer::Id(id) => {
                if id == block{
                    return
                } else {
                    let ptr = self.first_medium_layer.len() as u32;
                    self.top_layer[index] = BlockIdOrPointer::Ptr(ptr);
                    self.first_medium_layer.push([BlockIdOrPointer::Id(*b);4096]);
                    self.set_block_l2(block, pos, ptr);
                    if self.top_layer.iter().all(|e| e == BlockIdOrPointer::Id && e == self.top_layer[0]){
                        match self.top_layer[0]{
                            BlockIdOrPointer::Id(id) => self.top_layer[index] = BlockIdOrPointer::Id(id),
                            BlockIdOrPointer::Ptr(_) => panic!("something went horribly wrong when setting a block")
                        }
                    }
                }
            }
            BlockIdOrPointer::Ptr(ptr) => {
                if ptr < self.first_medium_layer.len() as u32 {
                    self.set_block_l2(block: BlockId, pos: &LocalBlockPos, ptr);
                }
            }
        }
    }
    pub fn set_block_l2(&mut self, block: BlockId, pos: &LocalBlockPos, ptr: u32){
        todo!("completely write");
    }

    pub fn get_block(&self, pos: &LocalBlockPos) -> BlockId {
        debug_assert!(pos.x >= 0
            || pos.x <= (CHUNKSIZE - 1) as i32
            || pos.y >= 0
            || pos.y <= (CHUNKSIZE - 1) as i32
            || pos.z >= 0
            || pos.z <= (CHUNKSIZE - 1) as i32);
        return match self.single_block{
            255 => self.get_block_l1(pos),
            b => b
        }
    }
    pub fn get_block_l1(&self, pos: &LocalBlockPos) -> BlockId{
        let index = Chunk::what_index_top_layer(pos);
        return match self.top_layer[index] {
            BlockIdOrPointer::Id(id) => id,
            BlockIdOrPointer::Ptr(ptr) => self.get_block_l2(pos, ptr)
        }
    }
    pub fn get_block_l2(&self, pos: &LocalBlockPos, ptr: u32) -> BlockId{
        let index = Chunk::what_index_first_medium_layer(pos);
        return match self.first_medium_layer[ptr][index] {
            BlockIdOrPointer::Id(id) => id,
            BlockIdOrPointer::Ptr(ptr) => self.get_block_l3(pos, ptr)
        }
    }
    pub fn get_block_l3(&self, pos: &LocalBlockPos, ptr: u32) -> BlockId{
        let index = Chunk::what_index_second_medium_layer(pos);
        return match self.second_medium_layer[ptr][index] {
            BlockIdOrPointer::Id(id) => id,
            BlockIdOrPointer::Ptr(ptr) => self.get_block_l4(pos, ptr)
        }
    }
    pub fn get_block_l4(&self, pos: &LocalBlockPos, ptr: u32) -> BlockId{
        let index = Chunk::what_index_bottom_layer(pos);
        return self.bottom_layer[ptr][index]
    }*/
}
