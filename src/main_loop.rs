use crate::chunk_manager::ChunkManager;
use crate::constants::VERTICALCHUNKS;
use crate::player::Player;
use crate::positions::ChunkPos;
use crate::renderer::glium::{create_display, gen_draw_params, gen_program, DrawInfo};
use crate::ui::{UiData, UiRenderer};
use crate::world::World;
use glium::backend::glutin::glutin::event_loop::ControlFlow;
use glium::glutin::event::Event;
use glium::{glutin, Surface};
use log::info;
use std::collections::LinkedList;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct MainLoop {
    //pub event_loop: EventLoop<()>,
//pub draw_info: DrawInfo<'a>,
//pub ui_renderer: UiRenderer
}

impl MainLoop {
    pub fn new() -> MainLoop {
        /*let event_loop = glutin::event_loop::EventLoop::new();
        let display = create_display(&event_loop);
        let program = gen_program(&display);
        let mut draw_info = DrawInfo{display: display, program: program, program_start: SystemTime::now(), draw_params: gen_draw_params()};
        let mut ui_renderer = UiRenderer::init(&draw_info);
        MainLoop{event_loop, draw_info, ui_renderer}*/
        return MainLoop {};
    }

    pub fn run(&mut self) {
        let event_loop = glutin::event_loop::EventLoop::new();
        let display = create_display(&event_loop);
        let program = gen_program(&display);
        let mut draw_info = DrawInfo {
            display: display,
            program: program,
            program_start: SystemTime::now(),
            draw_params: gen_draw_params(),
        };
        let mut ui_renderer = UiRenderer::init(&draw_info);
        //MainLoop{event_loop, draw_info, ui_renderer};

        let mut player = Player::new();
        info!("generating chunk main");
        let mut world = World {
            chunk_manager: ChunkManager::new(),
            seed: ((1f64
                / SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64())
                * 10000f64) as u32,
        };
        /*for x in 0..20 {
            for y in 0..VERTICALCHUNKS as i32 {
                for z in 0..20 {
                    world.chunk_manager.load_chunk(ChunkPos { x, y, z });
                }
            }
        }*/

        let mut timer = Instant::now();
        let mut rerender_timer = Instant::now();
        const FRAMERATE: f32 = 60f32;
        let mut ui_data = UiData { clicked: false };
        let mut update_timer = Instant::now();
        let mut frame_rate_queue = LinkedList::new();
        for _ in 0..10 {
            frame_rate_queue.push_back(0f32);
        }
        info!("starting main loop");
        event_loop.run(move |event, _, control_flow| {
            MainLoop::event_handler(event, control_flow);

            if update_timer.elapsed().as_millis() > 100 {
                let dt = timer.elapsed().as_secs_f32();
                update_timer = Instant::now();
                player.update(&dt);
                let current_chunk = player.position.get_chunk();
                for x in current_chunk.x - 5..current_chunk.x + 6 {
                    for y in 0..VERTICALCHUNKS as i32 {
                        for z in current_chunk.z - 5..current_chunk.z + 6 {
                            if !world.chunk_manager.chunk_exists(&ChunkPos { x, y, z }) {
                                world.chunk_manager.load_chunk(ChunkPos { x, y, z });
                            }
                        }
                    }
                }
                world.chunk_manager.update(&dt, &draw_info, &world.seed);
            }

            if 1f32 / rerender_timer.elapsed().as_secs_f32() < FRAMERATE {
                let frame_rate = 1f32 / rerender_timer.elapsed().as_secs_f32();
                frame_rate_queue.pop_front();
                frame_rate_queue.push_back(frame_rate);
                let mut average_fps = 0f32;
                let mut lowest_fps = 88888888f32;
                for i in &frame_rate_queue {
                    if i < &lowest_fps {
                        lowest_fps = i.clone();
                    }
                    average_fps += i;
                }
                average_fps = average_fps / frame_rate_queue.len() as f32;
                rerender_timer = Instant::now();
                let dt = timer.elapsed().as_secs_f32();
                timer = Instant::now();
                player.handle_input(&dt);
                let mut target = draw_info.display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                world
                    .chunk_manager
                    .render_chunks(&mut draw_info, &mut target, &player, 100);

                let text = vec![
                    "yeet".to_string(),
                    format!("now: {}", frame_rate.to_string()),
                    format!("low: {}", lowest_fps.to_string()),
                    format!("ave: {}", average_fps.to_string()),
                ];
                ui_renderer.draw(&draw_info, &text, &mut target, &mut ui_data);

                target.finish().unwrap();
            }
        });
    }

    pub fn event_handler(event: Event<()>, control_flow: &mut ControlFlow) {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    if input.virtual_keycode.is_some()
                        && input.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::Escape
                    {
                        glutin::event_loop::ControlFlow::Exit
                    } else {
                        glutin::event_loop::ControlFlow::Poll
                    }
                }

                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => glutin::event_loop::ControlFlow::Poll,
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    }
}
