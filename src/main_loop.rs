use crate::chunk_manager::ChunkManager;
use crate::constants::METACHUNK_GEN_RANGE;
use crate::player::Player;
use crate::positions::MetaChunkPos;
//use crate::ui::UiRenderer;
use log::info;
use std::collections::{BinaryHeap, LinkedList};
use std::time::{Instant, SystemTime};

pub struct MainLoop {}

impl MainLoop {
    pub fn new() -> MainLoop {
        return MainLoop {};
    }

    pub fn run(&mut self) {
        /*let event_loop = EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();
        let display = create_display(&event_loop);
        let program = gen_program(&display);
        let mut draw_info = DrawInfo {
            display: display,
            program: program,
            program_start: SystemTime::now(),
            draw_params: gen_draw_params(),
        };
        //let mut ui_renderer = UiRenderer::init(&draw_info);

        info!("generating chunk main");
        let mut chunk_manager = ChunkManager::new(10);
        let mut player = Player::new();
        let mut busy_frame_time = 0f64;
        let mut busy_update_time = 0f64;

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
            if draw_info.program_start.elapsed().unwrap().as_secs_f64() > 60f64 {
                println!("busy frame time: {}", busy_frame_time);
                println!("busy update time: {}", busy_update_time);
                MainLoop::kill_game_loop(control_flow);
                return;
            }
            MainLoop::event_handler(event, control_flow);

            if update_timer.elapsed().as_millis() > 100 {
                let dt = timer.elapsed().as_secs_f32();
                update_timer = Instant::now();
                MainLoop::on_game_tick(&dt, &mut player, &mut chunk_manager);
                chunk_manager.gen_vertex_buffers(&mut draw_info, &player);
                update_times.pop_front();
                update_times.push_back(update_timer.elapsed().as_secs_f32());
                busy_update_time += update_timer.elapsed().as_secs_f64();
            } else if 1f32 / rerender_timer.elapsed().as_secs_f32() < FRAMERATE {
                let dt = rerender_timer.elapsed().as_secs_f32();
                rerender_timer = Instant::now();
                player.handle_input(&dt);

                MainLoop::on_render(
                    &dt,
                    &update_times,
                    &draw_times,
                    &player,
                    &chunk_manager,
                    &mut draw_info,
                    &mut ui_renderer,
                );
                draw_times.pop_front();
                draw_times.push_back(rerender_timer.elapsed().as_secs_f32());
                busy_frame_time += rerender_timer.elapsed().as_secs_f64();
            }
        });*/
    }
    pub fn on_game_tick(dt: &f32, player: &mut Player, world: &mut ChunkManager) {
        player.update(&dt);
        world.load_generated_chunks();
        if player.generated_chunks_for != player.position.get_chunk() {
            MainLoop::on_player_moved_chunks(player, world);
        }
    }

    /*pub fn on_render(
        _dt: &f32,
        update_buffer: &LinkedList<f32>,
        draw_buffer: &LinkedList<f32>,
        player: &Player,
        world: &ChunkManager,
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
        world.render_chunks(draw_info, &mut target, &player);

        let text = vec![
            format!("long up: {}", longest_update.to_string()),
            format!("ave up: {}", average_update.to_string()),
            format!("long dr: {}", longest_draw.to_string()),
            format!("ave dr: {}", average_draw.to_string()),
            format!("total vertex buffers: {}", world.count_vertex_buffers()),
            format!("total chunks: {}", world.count_chunks()),
            format!(
                "total vertex buffers drawn: {}",
                world.count_vertex_buffers_in_range(&player)
            ),
            format!("total vertices: {}", world.count_vertices()),
            format!(
                "x: {} y: {} z: {}",
                player.position.x as i32, player.position.y as i32, player.position.z as i32
            ),
        ];
        let draw_result = ui_renderer.draw(&draw_info, &text, &mut target);
        match draw_result {
            Ok(_) => (),
            Err(e) => println!("error when drawing ui: {}", e),
        }

        target.finish().unwrap();
    }*/
    pub fn on_player_moved_chunks(player: &mut Player, world: &mut ChunkManager) {
        let current_chunk = player.position.get_meta_chunk();
        let mut to_load = BinaryHeap::new();
        for x in current_chunk.x - METACHUNK_GEN_RANGE as i32 - 1
            ..current_chunk.x + METACHUNK_GEN_RANGE as i32 + 1
        {
            for z in current_chunk.z - METACHUNK_GEN_RANGE as i32 - 1
                ..current_chunk.z + METACHUNK_GEN_RANGE as i32 + 1
            {
                if ChunkManager::meta_chunk_should_be_loaded(&player, &MetaChunkPos { x, z })
                    && !world
                        .world_data
                        .chunk_exists_or_generating(&MetaChunkPos { x, z })
                {
                    let chunk_pos = MetaChunkPos { x, z };
                    to_load.push((
                        (chunk_pos.get_distance_to_object(&player.position) * 10f32) as i64 * -1,
                        chunk_pos,
                    ));
                }
            }
        }
        while !to_load.is_empty() {
            world.load_chunk(to_load.pop().unwrap().1);
        }

        world
            .world_data
            .chunks
            .retain(|pos, _| ChunkManager::meta_chunk_should_be_loaded(&player, pos));
        /*world.vertex_buffers.retain(|pos, _| {
            ChunkManager::meta_chunk_should_be_loaded(&player, &pos.get_meta_chunk_pos())
        });*/
        player.generated_chunks_for = player.position.get_chunk();
    }
    /*pub fn event_handler(event: Event<()>, control_flow: &mut ControlFlow) {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,

                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => glutin::event_loop::ControlFlow::Poll,
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    }
    pub fn kill_game_loop(control_flow: &mut ControlFlow) {
        *control_flow = glutin::event_loop::ControlFlow::Exit;
    }*/
}
