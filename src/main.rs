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

    let event_loop = glutin::event_loop::EventLoop::new();
    let display = create_display(&event_loop);


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
    let mut context = imgui::Context::create();
    let mut glium_renderer = imgui_glium_renderer::Renderer::init(&mut context, &draw_info.display).unwrap();

    let mut platform = WinitPlatform::init(&mut context);
    {
        let gl_window = &draw_info.display.gl_window();
        let window = gl_window.window();
        platform.attach_window(context.io_mut(), &window, HiDpiMode::Rounded);
    }
    let hidpi_factor = platform.hidpi_factor();


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


            let mut ui = context.frame();
            let mut run = true;
            Window::new(im_str!("Hello world"))
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(format!("Hello world! {}", rerender_timer.elapsed().as_secs_f32()));
                    ui.text(im_str!("こんにちは世界！"));
                    ui.text(im_str!("This...is...imgui-rs!"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            platform.prepare_render(&ui, draw_info.display.gl_window().window());
            glium_renderer.render(&mut target, ui.render());


            target.finish().unwrap();
            //println!("vertices: {} rendering time: {} ms", c.count_verticecs(), rerender_timer.elapsed().as_secs_f32()*1000f32);
        }
    });
}