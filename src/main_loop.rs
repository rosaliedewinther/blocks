use crate::chunk_manager::ChunkManager;
use crate::constants::{CHUNK_GEN_RANGE, CHUNK_UNLOAD_RADIUS, VERTICALCHUNKS};
use crate::player::Player;
use crate::positions::ChunkPos;
use crate::renderer::glium::{create_display, gen_draw_params, gen_program, DrawInfo};
use crate::ui::UiRenderer;
use crate::world::World;
use glium::backend::glutin::glutin::event_loop::ControlFlow;
use glium::glutin::event::Event;
use glium::{glutin, Surface};
use log::info;
use rayon::prelude::IntoParallelIterator;
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
        let world_seed = 15u32;
        let mut world = World {
            chunk_manager: ChunkManager::new(world_seed),
            seed: ((1f64
                / SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64())
                * 10000f64) as u32,
        };

        let timer = Instant::now();
        let mut rerender_timer = Instant::now();
        const FRAMERATE: f32 = 60f32;
        let mut update_timer = Instant::now();
        let mut update_times = LinkedList::new();
        for _ in 0..30 {
            update_times.push_back(0f32);
        }
        let mut draw_times = LinkedList::new();
        for _ in 0..30 {
            draw_times.push_back(0f32);
        }
        info!("starting main loop");
        event_loop.run(move |event, _, control_flow| {
            MainLoop::event_handler(event, control_flow);

            if update_timer.elapsed().as_millis() > 100 {
                let dt = timer.elapsed().as_secs_f32();
                update_timer = Instant::now();
                MainLoop::on_game_tick(&dt, &mut player, &mut world);
                world
                    .chunk_manager
                    .gen_vertex_buffers(&mut draw_info, &player);
                update_times.pop_front();
                update_times.push_back(update_timer.elapsed().as_secs_f32());
            } else if 1f32 / rerender_timer.elapsed().as_secs_f32() < FRAMERATE {
                let dt = rerender_timer.elapsed().as_secs_f32();
                rerender_timer = Instant::now();
                player.handle_input(&dt);

                MainLoop::on_render(
                    &dt,
                    &update_times,
                    &draw_times,
                    &player,
                    &world,
                    &mut draw_info,
                    &mut ui_renderer,
                );
                draw_times.pop_front();
                draw_times.push_back(rerender_timer.elapsed().as_secs_f32());
            }
        });
    }
    pub fn on_game_tick(dt: &f32, player: &mut Player, world: &mut World) {
        player.update(&dt);
        if player.generated_chunks_for != player.position.get_chunk() {
            MainLoop::on_player_moved_chunks(player, world);
        }
        world.chunk_manager.update(&dt);
    }
    pub fn on_player_moved_chunks(player: &mut Player, world: &mut World) {
        let current_chunk = player.position.get_chunk();
        for x in
            current_chunk.x - CHUNK_GEN_RANGE as i32..current_chunk.x + CHUNK_GEN_RANGE as i32 + 1
        {
            for y in 0..VERTICALCHUNKS as i32 {
                for z in current_chunk.z - CHUNK_GEN_RANGE as i32
                    ..current_chunk.z + CHUNK_GEN_RANGE as i32 + 1
                {
                    if ChunkManager::chunk_should_be_loaded(&player, &ChunkPos { x, y, z })
                        && !world
                            .chunk_manager
                            .world_data
                            .chunk_exists_or_generating(&ChunkPos { x, y, z })
                    {
                        world.chunk_manager.load_chunk(ChunkPos { x, y, z });
                    }
                }
            }
        }
        world
            .chunk_manager
            .world_data
            .chunks
            .retain(|pos, c| ChunkManager::chunk_should_be_loaded(&player, pos));
        world
            .chunk_manager
            .vertex_buffers
            .retain(|pos, c| ChunkManager::chunk_should_be_loaded(&player, pos));
        player.generated_chunks_for = player.position.get_chunk();
    }
    pub fn on_render(
        dt: &f32,
        update_buffer: &LinkedList<f32>,
        draw_buffer: &LinkedList<f32>,
        player: &Player,
        world: &World,
        draw_info: &mut DrawInfo,
        ui_renderer: &mut UiRenderer,
    ) {
        let mut average_update = 0f32;
        let mut longest_update = 0f32;
        for i in update_buffer.iter() {
            if i.clone() > longest_update {
                longest_update = i.clone();
            }
            average_update += i.clone();
        }
        average_update = average_update / update_buffer.len() as f32;

        let mut average_draw = 0f32;
        let mut longest_draw = 0f32;
        for i in draw_buffer.iter() {
            if i.clone() > longest_draw {
                longest_draw = i.clone();
            }
            average_draw += i.clone();
        }
        average_draw = average_draw / draw_buffer.len() as f32;

        let mut target = draw_info.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 0.0), 1.0);
        world
            .chunk_manager
            .render_chunks(draw_info, &mut target, &player);

        let text = vec![
            format!("long up: {}", longest_update.to_string()),
            format!("ave up: {}", average_update.to_string()),
            format!("long dr: {}", longest_draw.to_string()),
            format!("ave dr: {}", average_draw.to_string()),
            format!(
                "total vertex buffers: {}",
                world.chunk_manager.count_vertex_buffers()
            ),
            format!("total chunks: {}", world.chunk_manager.count_chunks()),
            format!(
                "total vertex buffers drawn: {}",
                world.chunk_manager.count_vertex_buffers_in_range(&player)
            ),
            format!("total vertices: {}", world.chunk_manager.count_vertices()),
        ];
        ui_renderer.draw(&draw_info, &text, &mut target);

        target.finish().unwrap();
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
