use super::utils::{self, Vertex};
use super::rendering::Renderer;
use glium::{
    texture::Texture2d,
    VertexBuffer,
    IndexBuffer,
    index::PrimitiveType,
    DrawParameters,
    draw_parameters::{DepthTest, BackfaceCullingMode},
    Depth,
    uniform,
    Surface,
    Frame,
};
use nalgebra::{Matrix4, Vector3};
use std::f32::consts::PI;

pub fn main() {
    let renderer = Renderer::new();

    let image = utils::load_image(include_bytes!("./assets/blocques.png"));
    let texture = Texture2d::new(&renderer.display, image).unwrap();

    let vertices = vec![
        Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 1.0] },
    ];
    let vertex_buffer = VertexBuffer::new(&renderer.display, &vertices).unwrap();
    let indices: Vec<u16> = vec![
        0, 1, 3,
        1, 2, 3,
    ];
    let index_buffer = IndexBuffer::new(
        &renderer.display,
        PrimitiveType::TrianglesList,
        &indices
    ).unwrap();

    renderer.start(Box::new(move |target, perspective_ref, total_elapsed, _elapsed| {
        let model = Matrix4::from_euler_angles(
            PI / 6.0 * (total_elapsed * 2.0 * PI).sin(),
            total_elapsed * 2.0 * PI / 5.0,
            0.0,
        ).append_translation(&Vector3::new(0.0, 0.0, -2.0));
        let model_ref = model.as_ref();
        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        target.draw(
            &vertex_buffer,
            &index_buffer,
            &renderer.program,
            &uniform! {
                matrix: *model_ref,
                perspective: *perspective_ref,
                tex: &texture,
            },
            &params,
        ).unwrap();
    }));
}
