use crate::{
    rendering::{RenderValues, Renderer},
    utils::{self, SubTextureInfo},
    world::{World, Block},
};
use glium::{index::PrimitiveType, texture::Texture2d, IndexBuffer, VertexBuffer};
use nalgebra::{Matrix4, Vector3};
use std::f32::consts::PI;

pub fn main() {
    let renderer = Renderer::new();

    let image = utils::load_image(include_bytes!("./assets/blocques.png"));
    let texture = Texture2d::new(&renderer.display, image).unwrap();
    let texture_info = SubTextureInfo {
        x: 0.0,
        y: 0.0,
        size: 1.0,
    };

    let mut world = World::new();
    world.generate_chunk((0, 0, 0));
    world.set_block((5, 5, 5), if let Block::Empty = world.get_block((5, 5, 5)) {
        Block::Filled
    } else {
        Block::Empty
    });
    world.generate_vertices_for_chunks(vec![(0, 0, 0)], &texture_info);

    let vertices = world.get_vertices_for_chunks(vec![(0, 0, 0)]);
    let vertex_buffer = VertexBuffer::new(&renderer.display, &vertices).unwrap();
    let indices: Vec<u16> = vec![0, 1, 3, 1, 2, 3];
    let index_buffer =
        IndexBuffer::new(&renderer.display, PrimitiveType::TrianglesList, &indices).unwrap();

    renderer.start(
        Box::new(move |total_elapsed, _elapsed| {
            let model = Matrix4::from_euler_angles(
                PI / 6.0 * (total_elapsed * 2.0 * PI).sin(),
                total_elapsed * 2.0 * PI / 5.0,
                0.0,
            )
            .append_translation(&Vector3::new(0.0, 0.0, -2.0));
            RenderValues {
                model: Some(model),
                ..RenderValues::default()
            }
        }),
        RenderValues {
            vertex_buffer: Some(vertex_buffer),
            index_buffer: Some(index_buffer),
            texture: Some(texture),
            background_colour: Some((0.005, 0.0, 0.01, 1.0)),
            fov: Some(PI / 3.0),
            near: Some(0.1),
            far: Some(1024.0),
            ..RenderValues::default()
        },
    );
}
