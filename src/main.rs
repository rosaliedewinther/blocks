#[macro_use]
extern crate glium;

use log::info;

use crate::logger::setup_logger;
use crate::main_loop::MainLoop;
use crate::renderer::glium::DrawInfo;
use glium::index::PrimitiveType;
use glium::IndexBuffer;

mod block;
mod chunk;
mod chunk_manager;
mod constants;
mod input;
mod logger;
mod main_loop;
mod player;
mod positions;
mod renderer;
mod ui;
mod utils;
mod world;

fn gen_index(draw_info: &DrawInfo) -> IndexBuffer<u16> {
    return glium::IndexBuffer::new(
        &draw_info.display,
        PrimitiveType::TrianglesList,
        &[0u16, 1, 2],
    )
    .unwrap();
}

fn main() {
    setup_logger().unwrap();

    info!("starting up");

    let mut main_loop = MainLoop::new();
    main_loop.run();
}
