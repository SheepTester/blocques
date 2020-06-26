mod rendering;
mod utils;

use utils::Vertex;
use glium::{
    texture::Texture2d,
    VertexBuffer,
    IndexBuffer,
    index::PrimitiveType,
    DrawParameters,
    draw_parameters::{DepthTest, BackfaceCullingMode},
    Depth,
    uniform,
};
use nalgebra::{Matrix4, Vector3};
use std::f32::consts::PI;

pub fn main() {
    let (event_loop, display, program) = rendering::init();

    let texture = Texture2d::new(&display, image).unwrap();

    let vertices = vec![
        Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 1.0] },
    ];
    let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
    let indices: Vec<u16> = vec![
        0, 1, 3,
        1, 2, 3,
    ];
    let index_buffer = IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &indices
    ).unwrap();

    rendering::start(event_loop, move |total_elapsed, elapsed| {
        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        let model = Matrix4::from_euler_angles(
            PI / 6.0 * (total_elapsed * 2.0 * PI).sin(),
            total_elapsed * 2.0 * PI / 5.0,
            0.0,
        ).append_translation(&Vector3::new(0.0, 0.0, -2.0));
        let model_ref = model.as_ref();
        let perspective = Matrix4::new_perspective(
            width as f32 / height as f32,
            PI / 3.0,
            0.1,
            1024.0,
        );
        let perspective_ref = perspective.as_ref();
        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        target.clear_color_and_depth((0.0, 0.5, 1.0, 1.0), 1.0);
        target.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniform! {
                matrix: *model_ref,
                perspective: *perspective_ref,
                tex: &texture,
            },
            &params,
        ).unwrap();
        target.finish().unwrap();
    });
}
