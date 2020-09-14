#[macro_use]
extern crate glium;
use log::info;

use glium::{glutin, Display, VertexBuffer, IndexBuffer, Program, Surface, Frame, DrawParameters, Blend};
use glium::index::PrimitiveType;
use glutin::event_loop::EventLoop;
use std::time::{SystemTime, Instant};
use crate::chunk_manager::{ChunkManager, CHUNKSIZE};
use crate::player::Player;
use std::ops;
use crate::block::Block;

mod block;
mod chunk;
mod chunk_manager;
mod player;
mod utils;
mod input;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

type Color = [f32; 4];


pub struct DrawInfo<'a>{
    display: Display,
    program: Program,
    program_start: SystemTime,
    draw_params: DrawParameters<'a>,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct GlobalBlockPos{
    pub x: i32,
    pub y: i32,
    pub z: i32
}
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct LocalBlockPos{
    pub x: i32,
    pub y: i32,
    pub z: i32
}
#[derive(Debug, Copy, Clone)]
pub struct ObjectPos{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct ChunkPos{
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl GlobalBlockPos{
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> GlobalBlockPos{
        GlobalBlockPos {x: self.x+x_diff, y: self.y+y_diff, z: self.z+z_diff}
    }
    pub fn new()-> GlobalBlockPos{
        GlobalBlockPos{x:0,y:0,z:0}
    }
    pub fn get_local_pos(&self) -> LocalBlockPos{
        LocalBlockPos {x:self.x% CHUNKSIZE as i32, y:self.y% CHUNKSIZE as i32, z:self.z% CHUNKSIZE as i32 }
    }
    pub fn get_chunk_pos(&self) -> ChunkPos{
        ChunkPos {x:self.x/ CHUNKSIZE as i32, y:self.y/ CHUNKSIZE as i32, z:self.z/ CHUNKSIZE as i32 }
    }
    pub fn get_block_centre(&self) -> ObjectPos{
        ObjectPos {x:self.x as f32-0.5, y:self.y as f32-0.5, z:self.z as f32-0.5}
    }
    pub fn new_from_chunk_local(chunk_pos: &ChunkPos, local_pos: &LocalBlockPos)->GlobalBlockPos{
        GlobalBlockPos{x:chunk_pos.x* CHUNKSIZE as i32 +local_pos.x, y:chunk_pos.y* CHUNKSIZE as i32 +local_pos.y, z:chunk_pos.z* CHUNKSIZE as i32 +local_pos.z}
    }
}
impl ChunkPos{
    pub fn get_diff(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> ChunkPos{
        ChunkPos {x: self.x+x_diff, y: self.y+y_diff, z: self.z+z_diff}
    }
}
impl LocalBlockPos{

}

impl ops::Sub<i32> for GlobalBlockPos {
    type Output = GlobalBlockPos;

    fn sub(self, val: i32) -> GlobalBlockPos {
        GlobalBlockPos {x: self.x-val, y: self.y-val, z: self.z-val}
    }
}

fn draw_vertices(draw_info: &mut DrawInfo, target: &mut Frame, vertex_buffer: &VertexBuffer<Vertex>, player: &Player){
    let utime: f32 = draw_info.program_start.elapsed().unwrap().as_secs_f32();
    let perspective = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    };
    let view = player.get_view_matrix();

    let uniforms = uniform! {
            matrix: [
                [0.1, 0.0, 0.0, 0.0],
                [0.0, 0.1, 0.0, 0.0],
                [0.0, 0.0, 0.1, 0.0],
                [0.0, 0.0, 1.0, 1.0f32]
            ],
            view: view,
            time: utime,
            perspective: perspective
        };
    // drawing a frame
    match target.draw(vertex_buffer, glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &draw_info.program, &uniforms, &draw_info.draw_params){
        Ok(_) => (),
        Err(err) => println!("{}", err.to_string())
    }
}

fn create_display(event_loop: &EventLoop<()>) -> Display {
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(23);
    return glium::Display::new(wb, cb, &event_loop).unwrap();
}

fn gen_index(draw_info: &DrawInfo) -> IndexBuffer<u16> {
    return glium::IndexBuffer::new(&draw_info.display, PrimitiveType::TrianglesList,
                                   &[0u16, 1, 2]).unwrap();
}

fn gen_program(display: &Display) -> Program {
    let program = program!(display,
        140 => {
            vertex: "
                #version 140
                mat4 rotationX( in float angle ) {
                return mat4(	1.0,		0,			0,			0,
                                0, 	cos(angle),	-sin(angle),		0,
                                0, 	sin(angle),	 cos(angle),		0,
                                0, 			0,			  0, 		1);
                }
                mat4 rotationY( in float angle ) {
                    return mat4(	cos(angle),		0,		sin(angle),	0,
                                            0,		1.0,			 0,	0,
                                    -sin(angle),	0,		cos(angle),	0,
                                            0, 		0,				0,	1);
                }
                mat4 rotationZ( in float angle ) {
                    return mat4(	cos(angle),		-sin(angle),	0,	0,
                                    sin(angle),		cos(angle),		0,	0,
                                            0,				0,		1,	0,
                                            0,				0,		0,	1);
                }
                #define PI 3.1415926535897932384626433832795
                uniform mat4 perspective;
                uniform mat4 matrix;
                uniform mat4 view;
                uniform float time;
                in vec3 position;
                in vec4 color;
                out vec4 vColor;
                void main() {
                    gl_Position = vec4(position, 1.0);
                    gl_Position = perspective * view * gl_Position;
                    vColor = color;
                }
            ",
            fragment: "
                #version 140
                uniform float time;
                in vec4 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vColor;
                }
            "
        },
    ).unwrap();
    return program;
}

fn gen_draw_params() -> DrawParameters<'static>{
    glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        blend: Blend::alpha_blending(),
        .. Default::default()
    }
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
    const FRAMERATE: f32 = 60f32;
    info!("starting main loop");
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