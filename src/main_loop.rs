use crate::input::input::Input;
use crate::personal_world::PersonalWorld;
use crate::renderer::wgpu::WgpuState;
use crate::ui::ui::UiRenderer;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub struct MainLoop {}

impl MainLoop {
    pub fn new() -> MainLoop {
        return MainLoop {};
    }

    pub fn run(self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_maximized(true)
            .build(&event_loop)
            .unwrap();
        let mut window_input = Input::new();
        let mut personal_world = PersonalWorld::new(&window);
        let mut world_tick_timer = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    window_input.update_cursor_moved(position);
                }
                WindowEvent::CursorEntered { .. } => {
                    window_input.update_cursor_entered();
                }
                WindowEvent::CursorLeft { .. } => {
                    window_input.update_cursor_left();
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    window_input.update_mouse_input(state, button);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    window_input.update_mouse_wheel(delta);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    window_input.update_keyboard_input(input, control_flow);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    personal_world.ui = UiRenderer::new(&window, &personal_world.renderer);
                    MainLoop::resize(*physical_size, &mut personal_world.renderer.wgpu);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    personal_world.ui = UiRenderer::new(&window, &personal_world.renderer);
                    MainLoop::resize(**new_inner_size, &mut personal_world.renderer.wgpu);
                }

                _ => {}
            },
            Event::RedrawRequested(_) => {
                let timer = Instant::now();

                personal_world.update_ui_input(&window_input);
                personal_world
                    .player
                    .handle_input(&window_input, &(0.01 as f32));
                personal_world.render(control_flow, &window, &event);
                window_input.update();
                personal_world
                    .ui
                    .debug_info
                    .insert_stat("render".to_string(), timer.elapsed().as_secs_f32());
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {
                if world_tick_timer.elapsed().as_secs_f32() * 20f32 > 1f32 {
                    personal_world.ui.debug_info.set_numbers(
                        "player x".to_string(),
                        personal_world.player.position.x as f64,
                    );
                    personal_world.ui.debug_info.set_numbers(
                        "player y".to_string(),
                        personal_world.player.position.y as f64,
                    );
                    personal_world.ui.debug_info.set_numbers(
                        "player z".to_string(),
                        personal_world.player.position.z as f64,
                    );
                    personal_world.ui.debug_info.set_numbers(
                        "amount of renderable chunks".to_string(),
                        personal_world.chunk_render_data.len() as f64,
                    );
                    personal_world.ui.debug_info.set_numbers(
                        "amount of chunks".to_string(),
                        personal_world.world.count_chunks() as f64,
                    );
                    let timer = Instant::now();
                    let number_generated = personal_world.check_vertices_to_generate();
                    if number_generated > 0 {
                        personal_world.ui.debug_info.insert_stat(
                            "per chunk vertex time".to_string(),
                            timer.elapsed().as_secs_f32() / number_generated as f32,
                        );
                    }

                    let timer = Instant::now();

                    personal_world.on_game_tick(0.1);
                    world_tick_timer = Instant::now();
                    personal_world
                        .ui
                        .debug_info
                        .insert_stat("world tick".to_string(), timer.elapsed().as_secs_f32());
                }
            }
        });
    }
    pub(crate) fn resize(new_size: winit::dpi::PhysicalSize<u32>, wgpu: &mut WgpuState) {
        wgpu.size = new_size;
        wgpu.sc_desc.width = new_size.width;
        wgpu.sc_desc.height = new_size.height;
        wgpu.swap_chain = wgpu.device.create_swap_chain(&wgpu.surface, &wgpu.sc_desc);
    }
}
