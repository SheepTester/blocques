use super::utils::Vertex;
use glium::{
    draw_parameters::{BackfaceCullingMode, DepthTest},
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    texture::Texture2d,
    uniform, Depth, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};
use nalgebra::Matrix4;
use std::time::Instant;

pub struct RenderValues<'a> {
    pub vertex_buffer: &'a VertexBuffer<Vertex>,
    pub index_buffer: &'a IndexBuffer<u16>,
    pub model: &'a Matrix4<f32>,
    pub texture: &'a Texture2d,
    pub background_colour: (f32, f32, f32, f32),
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

pub trait RenderController {
    fn draw(&mut self, total_elapsed: f32, elapsed: f32);
    fn get_values(&self) -> RenderValues;
}

pub struct Renderer {
    pub event_loop: EventLoop<()>,
    pub display: Display,
    pub program: Program,
}

impl Renderer {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new().with_title("B L O C Q U E S");
        let cb = ContextBuilder::new().with_depth_buffer(24);
        let display = Display::new(wb, cb, &event_loop).unwrap();
        let program = Program::from_source(
            &display,
            String::from_utf8_lossy(include_bytes!("./rendering/shader.vert"))
                .into_owned()
                .as_str(),
            String::from_utf8_lossy(include_bytes!("./rendering/shader.frag"))
                .into_owned()
                .as_str(),
            None,
        )
        .unwrap();
        Renderer {
            event_loop,
            display,
            program,
        }
    }

    pub fn start(self, mut controller: Box<dyn RenderController>) {
        let display = self.display;
        let event_loop = self.event_loop;
        let program = self.program;

        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let start = Instant::now();
        let mut last_time = start;

        event_loop.run(move |ev, _, control_flow| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        println!("Keyboard input: {:?}", input);
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => {}
                },
                _ => {}
            };

            let now = Instant::now();
            let total_elapsed = now.duration_since(start).as_secs_f32();
            let elapsed = now.duration_since(last_time).as_secs_f32();
            last_time = now;

            let mut target = display.draw();

            controller.draw(total_elapsed, elapsed);
            let RenderValues {
                vertex_buffer,
                index_buffer,
                model,
                texture,
                background_colour,
                fov,
                near,
                far,
            } = controller.get_values();

            let (width, height) = target.get_dimensions();
            let perspective =
                Matrix4::new_perspective(width as f32 / height as f32, fov, near, far);
            let perspective_ref = perspective.as_ref();

            let model_ref = model.as_ref();

            target.clear_color_and_depth(background_colour, 1.0);
            target
                .draw(
                    vertex_buffer,
                    index_buffer,
                    &program,
                    &uniform! {
                        matrix: *model_ref,
                        perspective: *perspective_ref,
                        tex: texture,
                    },
                    &params,
                )
                .unwrap();
            target.finish().unwrap();
        });
    }
}
