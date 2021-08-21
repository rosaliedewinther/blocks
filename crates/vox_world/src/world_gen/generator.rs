use crate::blocks::block::BlockId;

pub trait WorldGenerator {
    fn new() -> Self;
    fn generate_area(
        &self,
        x_start: i32,
        y_start: i32,
        z_start: i32,
        size: usize,
    ) -> Box<[BlockId]>;
    fn add_generation_layer(
        &self,
        generation_function: fn(x_start: i32, y_start: i32, z_start: i32, size: usize),
    );
}
