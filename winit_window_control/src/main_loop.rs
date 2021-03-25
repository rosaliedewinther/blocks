use crate::input::input::Input;
use std::time::Instant;
use winit::dpi::Size::Physical;
use winit::dpi::{PhysicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::monitor::MonitorHandle;
use winit::window::{Fullscreen, Window, WindowBuilder};

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
    fn on_tick(&mut self, dt: f64) -> UpdateResult;
    fn on_render(&mut self, input: &mut Input, dt: f64, window: &Window) -> RenderResult;
    fn on_init(&mut self, window: &Window) -> InitResult;
    fn on_resize(&mut self, physical_size: PhysicalSize<u32>);
}

pub fn main_loop_run<T>(mut game: T, window_width: Option<i32>, window_height: Option<i32>)
where
    T: 'static + Game,
{
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
    game.on_init(&window);
    let mut window_input = Input::new();
    let mut on_tick_timer = Instant::now();
    let mut on_render_timer = Instant::now();

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
                game.on_resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                game.on_resize(**new_inner_size);
            }

            _ => {}
        },
        Event::RedrawRequested(_) => {
            let dt = on_render_timer.elapsed().as_secs_f64();
            on_render_timer = Instant::now();
            match game.on_render(&mut window_input, dt, &window) {
                RenderResult::Continue => {}
                RenderResult::Exit => *control_flow = ControlFlow::Exit,
            };
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {
            if on_tick_timer.elapsed().as_secs_f32() * 20f32 > 1f32 {
                game.on_tick(on_tick_timer.elapsed().as_secs_f64());
                on_tick_timer = Instant::now();
            }
        }
    });
}
