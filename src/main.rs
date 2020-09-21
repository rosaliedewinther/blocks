#[macro_use]
extern crate glium;

use log::info;

use imgui_glium_renderer::imgui;
use imgui::{Ui, im_str};
use glium::{glutin, IndexBuffer, Surface};
use glium::index::PrimitiveType;
use std::time::{SystemTime, Instant};
use crate::chunk_manager::{ChunkManager};
use crate::player::Player;
use crate::positions::ChunkPos;
use crate::renderer::glium::{DrawInfo, create_display, gen_draw_params, gen_program};
use crate::constants::{WIDTH, HEIGHT};
use std::borrow::BorrowMut;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use imgui_glium_renderer::imgui::{Condition, Window};
use crate::ui::UiRenderer;
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


    let mut player = Player::new();
    info!("generating chunk main");
    let mut c = ChunkManager::new();
    for x in 0..5{
        for y in 0..5{
            for z in 0..5 {
                c.load_chunk(ChunkPos {x,y,z });
            }
        }
    }
    info!("generating chunk main done");
    let mut main_loop = MainLoop::new();
    //vertex_buffer.write()
    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.

    main_loop.run();




}