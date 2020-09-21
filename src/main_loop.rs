use glium::backend::glutin::glutin::event_loop::EventLoop;
use crate::renderer::glium::{DrawInfo, create_display, gen_program, gen_draw_params};
use crate::ui::UiRenderer;
use std::time::{SystemTime, Instant};
use crate::chunk_manager::ChunkManager;
use crate::player::Player;
use imgui_glium_renderer::imgui::{Window, Condition};
use glium::{glutin, Surface};
use log::info;
use imgui::{Ui, im_str};
use imgui_glium_renderer::imgui;
use crate::positions::ChunkPos;

pub struct MainLoop{
    //pub event_loop: EventLoop<()>,
    //pub draw_info: DrawInfo<'a>,
    //pub ui_renderer: UiRenderer
}

impl MainLoop{
    pub fn new() -> MainLoop{
        /*let event_loop = glutin::event_loop::EventLoop::new();
        let display = create_display(&event_loop);
        let program = gen_program(&display);
        let mut draw_info = DrawInfo{display: display, program: program, program_start: SystemTime::now(), draw_params: gen_draw_params()};
        let mut ui_renderer = UiRenderer::init(&draw_info);
        MainLoop{event_loop, draw_info, ui_renderer}*/
        return MainLoop{};
    }

    pub fn run(&mut self){
        let event_loop = glutin::event_loop::EventLoop::new();
        let display = create_display(&event_loop);
        let program = gen_program(&display);
        let mut draw_info = DrawInfo{display: display, program: program, program_start: SystemTime::now(), draw_params: gen_draw_params()};
        let mut ui_renderer = UiRenderer::init(&draw_info);
        //MainLoop{event_loop, draw_info, ui_renderer};

        let mut player = Player::new();
        info!("generating chunk main");
        let mut world = ChunkManager::new();
        for x in 0..5{
            for y in 0..5{
                for z in 0..5 {
                    world.load_chunk(ChunkPos {x,y,z });
                }
            }
        }

        let mut timer = Instant::now();
        let mut rerender_timer = Instant::now();
        const FRAMERATE: f32 = 30f32;
        info!("starting main loop");
        let mut t= 1f32;
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
                world.update(&dt, &draw_info);
                let mut target = draw_info.display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                world.render_chunks(&mut draw_info, &mut target, &player);


                let mut ui = ui_renderer.context.frame();
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

                ui_renderer.platform.prepare_render(&ui, draw_info.display.gl_window().window());
                ui_renderer.renderer.render(&mut target, ui.render());


                target.finish().unwrap();
            }
        });
    }
}