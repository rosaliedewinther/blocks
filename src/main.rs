#[macro_use]
extern crate glium;
//extern crate glium_text;

use log::info;

use glium::{glutin, IndexBuffer, Surface};
use glium::index::PrimitiveType;
use std::time::{SystemTime, Instant};
use crate::chunk_manager::{ChunkManager};
use crate::player::Player;
use crate::positions::ChunkPos;
use crate::renderer::{DrawInfo, create_display, gen_program, gen_draw_params, Vertex};

mod block;
mod chunk;
mod chunk_manager;
mod player;
mod utils;
mod input;
mod positions;
mod renderer;
mod constants;

fn gen_index(draw_info: &DrawInfo) -> IndexBuffer<u16> {
    return glium::IndexBuffer::new(&draw_info.display, PrimitiveType::TrianglesList,
                                   &[0u16, 1, 2]).unwrap();
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger();

    info!("starting up");
    implement_vertex!(Vertex, position, color);
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = create_display(&event_loop);

    //let system = glium_text::TextSystem::new(&display);
    //let font = glium_text::FontTexture::new(&display, std::fs::File::open(&std::path::Path::new("assets/OpenSans-Regular.ttf")).unwrap(), 24).unwrap();
    let program = gen_program(&display);
    let mut draw_info = DrawInfo{display: display, program: program, program_start: SystemTime::now(), draw_params: gen_draw_params()};
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

    //vertex_buffer.write()
    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let mut timer = Instant::now();
    let mut rerender_timer = Instant::now();
    const FRAMERATE: f32 = 30f32;
    info!("starting main loop");
    let mut t= 1f32;
    // the main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::KeyboardInput {device_id, input, is_synthetic} =>{
                    if input.virtual_keycode.is_some() && input.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::Escape  {
                        glutin::event_loop::ControlFlow::Exit
                    } else {
                        glutin::event_loop::ControlFlow::Poll
                    }
                }

                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => {
                    glutin::event_loop::ControlFlow::Poll
                },
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
        if 1f32/rerender_timer.elapsed().as_secs_f32() < FRAMERATE{
            rerender_timer = Instant::now();
            let dt = timer.elapsed().as_secs_f32();
            timer = Instant::now();
            player.handle_input(&dt);
            player.update(&dt);
            c.update(&dt, &draw_info);
            let mut target = draw_info.display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
            c.render_chunks(&mut draw_info, &mut target, &player);


            target.finish().unwrap();
            //println!("vertices: {} rendering time: {} ms", c.count_verticecs(), rerender_timer.elapsed().as_secs_f32()*1000f32);
        }
    });
}