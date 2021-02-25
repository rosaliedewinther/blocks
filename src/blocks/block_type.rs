use serde::{Deserialize, Serialize};
#[derive(PartialEq, Clone, Copy, Deserialize, Serialize, Debug)]
pub enum BlockType {
    Unknown,
    Grass,
    Water,
    Dirt,
    Stone,
    Sand,
    Air,
    Leaf,
}
