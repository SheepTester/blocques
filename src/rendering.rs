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
use std::time::{Duration, Instant};

pub struct RenderValues {
    pub vertex_buffer: Option<VertexBuffer<Vertex>>,
    pub index_buffer: Option<IndexBuffer<u16>>,
    pub model: Option<Matrix4<f32>>,
    pub texture: Option<Texture2d>,
    pub background_colour: Option<(f32, f32, f32, f32)>,
    pub fov: Option<f32>,
    pub near: Option<f32>,
    pub far: Option<f32>,
}

impl Default for RenderValues {
    fn default() -> Self {
        Self {
            vertex_buffer: None,
            index_buffer: None,
            model: None,
            texture: None,
            background_colour: None,
            fov: None,
            near: None,
            far: None,
        }
    }
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
            event_loop: event_loop,
            display: display,
            program: program,
        }
    }

    pub fn start(self, draw: Box<dyn Fn(f32, f32) -> RenderValues>, static_values: RenderValues) {
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
            let now = Instant::now();
            let total_elapsed = now.duration_since(start).as_secs_f32();
            let elapsed = now.duration_since(last_time).as_secs_f32();
            last_time = now;

            let mut target = display.draw();

            let dynamic_values = draw(total_elapsed, elapsed);
            // Panic if value not given in static and dynamic values
            let vertex_buffer = static_values
                .vertex_buffer
                .as_ref()
                .or_else(|| dynamic_values.vertex_buffer.as_ref())
                .unwrap();
            let index_buffer = static_values
                .index_buffer
                .as_ref()
                .or_else(|| dynamic_values.index_buffer.as_ref())
                .unwrap();
            let model = static_values
                .model
                .as_ref()
                .or_else(|| dynamic_values.model.as_ref())
                .unwrap();
            let texture = static_values
                .texture
                .as_ref()
                .or_else(|| dynamic_values.texture.as_ref())
                .unwrap();
            let background_colour = static_values
                .background_colour
                .or(dynamic_values.background_colour)
                .unwrap();
            let fov = static_values
                .fov
                .or(dynamic_values.fov)
                .unwrap();
            let near = static_values
                .near
                .or(dynamic_values.near)
                .unwrap();
            let far = static_values
                .far
                .or(dynamic_values.far)
                .unwrap();

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

            if let Event::WindowEvent { event, .. } = ev {
                if event == WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                }
            }
        });
    }
}
