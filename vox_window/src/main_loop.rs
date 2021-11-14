use crate::input::input::Input;
use std::time::Instant;
use winit::dpi::Size::Physical;
use winit::dpi::{PhysicalSize, Size};
use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::monitor::MonitorHandle;
use winit::window::{Fullscreen, Window, WindowBuilder};
use winit::event::VirtualKeyCode::Escape;

#[derive(PartialEq)]
pub enum RenderResult {
    Continue,
    Exit,
}
#[derive(PartialEq)]
pub enum UpdateResult {
    Continue,
    Exit,
}
#[derive(PartialEq)]
pub enum InitResult {
    Continue,
    Exit,
}

pub trait Game {
    fn on_tick(&mut self, input: &Input, dt: f64) -> UpdateResult;
    fn on_frame(&mut self, input: &Input, dt: f64) -> RenderResult;
    fn on_init(&mut self) -> InitResult;
}

pub fn main_loop_run<T>(mut game: T, window_width: Option<u32>, window_height: Option<u32>, vsync: bool, ms_per_tick: u32)
where
    T: 'static + Game,
{
    game.on_init();
    let (event_loop, window) = build_window(window_width, window_height);
    let mut wgpu_state = crate::renderer::wgpu_state::WgpuState::new(&window, vsync);
    let mut ui_renderer = crate::renderer::ui_renderer::UiRenderer::new(&window, &wgpu_state.device);

    let mut window_input = Input::new();
    let mut on_tick_timer = Instant::now();

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
                wgpu_state.resize(*physical_size)
            }
            WindowEvent::ScaleFactorChanged {  new_inner_size, .. } => {
                wgpu_state.resize(**new_inner_size)
            }

            _ => {}
        },
        Event::RedrawRequested(_) => {
            crate::renderer::renderer::do_render_pass(&wgpu_state, &mut ui_renderer);
            if window_input.key_pressed(Escape){
                *control_flow = ControlFlow::Exit
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {
            if on_tick_timer.elapsed().as_secs_f32() * (ms_per_tick as f32) > 1f32 {
                game.on_tick(&window_input, on_tick_timer.elapsed().as_secs_f64());
                on_tick_timer = Instant::now();
            }
        }
    });
}

fn build_window(window_width: Option<u32>, window_height: Option<u32>) -> (EventLoop<()>, Window){
    let event_loop = EventLoop::new();
    let mut window_builder = WindowBuilder::new();
    if window_width.is_some() && window_height.is_some() {
        window_builder = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(
                window_width.unwrap(),
                window_height.unwrap(),
            ))
            .with_resizable(false);
    } else {
        window_builder = WindowBuilder::new().with_maximized(true);
    }
    let window = window_builder.build(&event_loop).unwrap();

    (event_loop, window)
}