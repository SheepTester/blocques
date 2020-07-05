use super::utils::Vertex;
use failure::Error;
use glium::{
    draw_parameters::{BackfaceCullingMode, DepthTest},
    glutin::{
        event::{Event, KeyboardInput, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    index::IndicesSource,
    texture::Texture2d,
    uniform,
    uniforms::Sampler,
    Depth, Display, DrawParameters, Program, Surface, VertexBuffer,
};
use nalgebra::{Isometry3, Perspective3, Similarity3};
use std::time::Instant;

pub struct RenderValues<'a> {
    pub vertex_buffer: &'a VertexBuffer<Vertex>,
    pub indices: IndicesSource<'a>,
    pub model: &'a Similarity3<f32>, // Transformation of object itself
    pub view: &'a Isometry3<f32>,    // Transformation due to camera
    pub sampler: Sampler<'a, Texture2d>,
    pub background_colour: (f32, f32, f32, f32),
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

pub struct FrameInfo<'a> {
    pub total_elapsed: f32,
    pub elapsed: f32,
    pub display: &'a Display,
}

pub trait RenderController {
    fn on_key_event(&mut self, _key_event: KeyboardInput) {}
    fn on_frame(&mut self, _info: FrameInfo) {}
    fn get_values(&self) -> RenderValues;
}

pub struct Renderer {
    pub event_loop: EventLoop<()>,
    pub display: Display,
    pub program: Program,
}

impl Renderer {
    pub fn new() -> Result<Self, Error> {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new().with_title("B L O C Q U E S");
        let cb = ContextBuilder::new().with_depth_buffer(24);
        let display = Display::new(wb, cb, &event_loop)?;
        let program = Program::from_source(
            &display,
            String::from_utf8_lossy(include_bytes!("./rendering/shader.vert"))
                .into_owned()
                .as_str(),
            String::from_utf8_lossy(include_bytes!("./rendering/shader.frag"))
                .into_owned()
                .as_str(),
            None,
        )?;
        Ok(Renderer {
            event_loop,
            display,
            program,
        })
    }

    pub fn start<C>(self, mut controller: C)
    where
        C: RenderController + 'static,
    {
        let display = self.display;
        let event_loop = self.event_loop;
        let program = self.program;

        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            // This means that counterclockwise vertices WON'T be drawn!
            backface_culling: BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        let start = Instant::now();
        let mut last_time = start;

        event_loop.run(move |ev, _, control_flow| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        controller.on_key_event(input);
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

            controller.on_frame(FrameInfo {
                total_elapsed,
                elapsed,
                display: &display,
            });
            let RenderValues {
                vertex_buffer,
                indices,
                model,
                view,
                sampler,
                background_colour,
                fov,
                near,
                far,
            } = controller.get_values();

            let (width, height) = target.get_dimensions();
            let projection = Perspective3::new(width as f32 / height as f32, fov, near, far);

            let transform = projection.into_inner() * (view * model).to_homogeneous();
            let transform_ref = transform.as_ref();

            target.clear_color_and_depth(background_colour, 1.0);
            target
                .draw(
                    vertex_buffer,
                    indices,
                    &program,
                    &uniform! {
                        transform: *transform_ref,
                        tex: sampler,
                    },
                    &params,
                )
                .unwrap();
            target.finish().unwrap();
        });
    }
}
