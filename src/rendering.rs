use glium::{
    Display,
    Program,
    Frame,
    glutin::{
        event_loop::{EventLoop, ControlFlow},
        window::WindowBuilder,
        ContextBuilder,
        event::{WindowEvent, Event},
    },
    Surface,
    VertexBuffer,
    IndexBuffer,
    texture::Texture2d,
    index::PrimitiveType,
    DrawParameters,
    draw_parameters::{DepthTest, BackfaceCullingMode},
    Depth,
    uniform,
};
use std::{time::{Instant, Duration}, f32::consts::PI};
use nalgebra::Matrix4;
use super::utils::Vertex;

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
                .into_owned().as_str(),
            String::from_utf8_lossy(include_bytes!("./rendering/shader.frag"))
                .into_owned().as_str(),
            None,
        ).unwrap();
        Renderer {
            event_loop: event_loop,
            display: display,
            program: program,
        }
    }

    pub fn start(self, draw: Box<dyn Fn(f32, f32) -> (&VertexBuffer<Vertex>, &IndexBuffer<u16>, &[[f32; 4]; 4], &Texture2d)>) {
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
            let next_frame_time = now + Duration::from_nanos(16_666_667);
            let total_elapsed = now.duration_since(start).as_secs_f32();
            let elapsed = now.duration_since(last_time).as_secs_f32();
            last_time = now;

            let mut target = display.draw();
            let (width, height) = target.get_dimensions();
            let perspective = Matrix4::new_perspective(
                width as f32 / height as f32,
                PI / 3.0,
                0.1,
                1024.0,
            );
            let perspective_ref = perspective.as_ref();
            target.clear_color_and_depth((0.0, 0.5, 1.0, 1.0), 1.0);
            let (vertex_buffer, index_buffer, model_ref, texture) = draw(
                total_elapsed,
                elapsed,
            );
            target.draw(
                vertex_buffer,
                index_buffer,
                &program,
                &uniform! {
                    matrix: *model_ref,
                    perspective: *perspective_ref,
                    tex: texture,
                },
                &params,
            ).unwrap();
            target.finish().unwrap();

            *control_flow = ControlFlow::WaitUntil(next_frame_time);
            if let Event::WindowEvent { event, .. } = ev {
                if event == WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                }
            }
        });
    }
}
