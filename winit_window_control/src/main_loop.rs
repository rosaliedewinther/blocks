use crate::input::input::Input;
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

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

pub fn main_loop_run<T>(mut game: T)
where
    T: 'static + Game,
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)
        .unwrap();
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
                //personal_world.ui = UiRenderer::new(&window, &personal_world.renderer);
                //resize(*physical_size, &mut personal_world.renderer.wgpu);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                game.on_resize(**new_inner_size);
                //personal_world.ui = UiRenderer::new(&window, &personal_world.renderer);
                //resize(**new_inner_size, &mut personal_world.renderer.wgpu);
            }

            _ => {}
        },
        Event::RedrawRequested(_) => {
            match game.on_render(
                &mut window_input,
                on_render_timer.elapsed().as_secs_f64(),
                &window,
            ) {
                RenderResult::Continue => {}
                RenderResult::Exit => *control_flow = ControlFlow::Exit,
            };
            on_render_timer = Instant::now();
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
