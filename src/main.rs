#[macro_use]
extern crate glium;

use log::info;

use glium::IndexBuffer;
use glium::index::PrimitiveType;
use crate::renderer::glium::DrawInfo;
use crate::logger::setup_logger;
use crate::main_loop::MainLoop;

mod block;
mod chunk;
mod chunk_manager;
mod player;
mod utils;
mod input;
mod positions;
mod renderer;
mod constants;
mod ui;
mod logger;
mod main_loop;

fn gen_index(draw_info: &DrawInfo) -> IndexBuffer<u16> {
    return glium::IndexBuffer::new(&draw_info.display, PrimitiveType::TrianglesList,
                                   &[0u16, 1, 2]).unwrap();
}

fn main() {
    setup_logger();

    info!("starting up");

    let mut main_loop = MainLoop::new();
    main_loop.run();
}