use crate::{
    rendering::{RenderController, RenderValues, Renderer},
    utils::{self, SubTextureInfo, Vertex},
    world::{Block, World},
};
use glium::{index::PrimitiveType, texture::Texture2d, IndexBuffer, VertexBuffer};
use nalgebra::{Matrix4, Vector3};
use std::f32::consts::PI;

struct Blocques {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    texture: Texture2d,
    background_colour: (f32, f32, f32, f32),
    fov: f32,
    near: f32,
    far: f32,

    camera_pos: Vector3<f32>,
    camera_rot: (f32, f32, f32),
}

impl RenderController for Blocques {
    fn update(&mut self, total_elapsed: f32, _elapsed: f32) {
        // self.model = Matrix4::from_euler_angles(
        //     0.0,
        //     // PI / 6.0 * (total_elapsed * 2.0 * PI).sin(),
        //     (total_elapsed / 2.0 * PI).floor() * 2.0 * PI / 5.0,
        //     0.0,
        // )
        // .append_translation(&Vector3::new(0.0, 0.0, -2.0));
        // self.camera_rot.1 = total_elapsed;

        let (rx, ry, rz) = self.camera_rot;
        self.view = Matrix4::from_euler_angles(rz, rx, ry)
            .append_translation(&self.camera_pos.scale(-1.0));
    }

    fn get_values(&self) -> RenderValues {
        RenderValues {
            vertex_buffer: &self.vertex_buffer,
            index_buffer: &self.index_buffer,
            model: &self.model,
            view: &self.view,
            texture: &self.texture,
            background_colour: self.background_colour,
            fov: self.fov,
            near: self.near,
            far: self.far,
        }
    }
}

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
    world.set_block(
        (5, 5, 5),
        if let Block::Empty = world.get_block((5, 5, 5)) {
            Block::Filled
        } else {
            Block::Empty
        },
    );
    world.generate_vertices_for_chunks(vec![(0, 0, 0)], &texture_info);
    println!("Vertices generated: {}", world.get_vertices_for_chunks(vec![(0, 0, 0)]).len());

    // let vertices = world.get_vertices_for_chunks(vec![(0, 0, 0)]);
    let vertices = vec![
        Vertex {
            position: [0.5, 0.5, 0.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.0],
            tex_coords: [0.0, 1.0],
        },
    ];
    let vertex_buffer = VertexBuffer::new(&renderer.display, &vertices).unwrap();
    let indices: Vec<u16> = vec![0, 1, 3, 1, 2, 3];
    let index_buffer =
        IndexBuffer::new(&renderer.display, PrimitiveType::TrianglesList, &indices).unwrap();

    let controller = Blocques {
        vertex_buffer,
        index_buffer,
        model: Matrix4::from_euler_angles(0.0, 4.0 * PI / 5.0, 0.0),
        // model: Matrix4::identity(),
        view: Matrix4::identity(),
        texture,
        background_colour: (0.005, 0.0, 0.01, 1.0),
        fov: PI / 3.0,
        near: 0.1,
        far: 1024.0,

        camera_pos: Vector3::new(0.0, 0.0, 2.0),
        camera_rot: (0.0, 0.0, 0.0),
    };
    renderer.start(Box::new(controller));
}
