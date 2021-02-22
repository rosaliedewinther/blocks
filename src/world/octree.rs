use crate::block::BlockType;
use crate::positions::ObjectPos;
use noise::Fbm;

#[derive(Debug)]
pub struct Octree {
    height: u32,
    volume: Option<BlockType>,
    children: Box<[Option<Octree>; 8]>,
}

impl Octree {
    pub fn new() -> Octree {
        let mut children = [None, None, None, None, None, None, None, None];

        return Octree {
            height: 0,
            volume: None,
            children: Box::new(children),
        };
    }
    pub fn set_chunk(&mut self, chunk: OctreeChunk, octree: Octree) {
        match chunk {
            OctreeChunk::LeftBottomFront => self.children[0] = Some(octree),
            OctreeChunk::LeftBottomBack => self.children[1] = Some(octree),
            OctreeChunk::LeftTopFront => self.children[2] = Some(octree),
            OctreeChunk::LeftTopBack => self.children[3] = Some(octree),
            OctreeChunk::RightBottomFront => self.children[4] = Some(octree),
            OctreeChunk::RightBottomBack => self.children[5] = Some(octree),
            OctreeChunk::RightTopFront => self.children[6] = Some(octree),
            OctreeChunk::RightTopBack => self.children[7] = Some(octree),
        }
    }
    pub fn try_populate(&mut self, size: u32) {
        for child in self.children.iter_mut() {
            match child {
                None => {}
                Some(_) => {}
            }
        }
    }
}
#[derive(Debug)]
pub struct OctreeManager {
    octree: Option<Octree>,
    center: ObjectPos,
    height: u32,
}
pub enum OctreeChunk {
    LeftBottomFront,
    LeftBottomBack,
    LeftTopFront,
    LeftTopBack,
    RightBottomFront,
    RightBottomBack,
    RightTopFront,
    RightTopBack,
}

impl OctreeManager {
    pub fn new() -> OctreeManager {
        let mut octree = Octree::new();

        return OctreeManager {
            octree: Some(octree),
            center: ObjectPos {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            height: 0,
        };
    }
    pub fn increase(&mut self, chunk: OctreeChunk) {
        let mut new_octree = Octree::new();
        let old_octree = std::mem::replace(&mut self.octree, None).unwrap();
        new_octree.set_chunk(chunk, old_octree);
        self.octree = Some(new_octree);
        self.height += 1;
    }

    pub fn generate(&mut self) {
        let noise = Fbm::new();
        for i in 0..3 {
            self.increase(OctreeChunk::LeftBottomFront);
        }
    }
}
