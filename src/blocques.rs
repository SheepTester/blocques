use crate::{
    rendering::{RenderController, RenderValues, Renderer},
    utils::{self, SubTextureInfo, Vertex},
    world::{Block, ChunkCoord, World},
};
use glium::{
    glutin::event::{ElementState, KeyboardInput, VirtualKeyCode as KeyCode},
    index::{IndicesSource, PrimitiveType},
    texture::Texture2d,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    Display, IndexBuffer, VertexBuffer,
};
use nalgebra::{Isometry3, Similarity3, Translation3, UnitQuaternion, Vector3};
use std::{collections::HashMap, error::Error, f32::consts::PI};

struct Blocques {
    world: World,
    texture_info: SubTextureInfo,

    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: Option<IndexBuffer<u16>>,
    model: Similarity3<f32>,
    view: Isometry3<f32>,
    texture: Texture2d,
    background_colour: (f32, f32, f32, f32),
    fov: f32,
    near: f32,
    far: f32,

    camera_pos: Vector3<f32>,
    camera_rot: (f32, f32, f32),

    keys: HashMap<KeyCode, bool>,
}

impl Blocques {
    fn new(texture: Texture2d, display: &Display) -> Result<Self, Box<dyn Error>> {
        let vertices = vec![];
        Ok(Blocques {
            world: World::new(),
            texture_info: SubTextureInfo {
                x: 0.0,
                y: 0.0,
                size: 1.0,
            },

            vertex_buffer: VertexBuffer::new(display, &vertices)?,
            index_buffer: None,
            model: Similarity3::identity(),
            view: Isometry3::identity(),
            texture,
            background_colour: (0.005, 0.0, 0.01, 1.0),
            fov: PI / 3.0,
            near: 0.1,
            far: 1024.0,

            camera_pos: Vector3::new(0.0, 0.0, 0.0),
            camera_rot: (0.0, 0.0, 0.0),

            keys: HashMap::new(),
        })
    }

    fn update_vertices_for_chunks(
        &mut self,
        chunk_coords: Vec<ChunkCoord>,
        display: &Display,
    ) -> Result<(), Box<dyn Error>> {
        let vertices = self.world.get_vertices_for_chunks(chunk_coords);
        self.vertex_buffer = VertexBuffer::new(display, &vertices)?;

        let squares = vertices.len() / 4;
        let mut indices = Vec::with_capacity(squares * 6);
        for square in 0..squares {
            let i = square as u16 * 4;
            indices.extend(vec![i, i + 1, i + 3, i + 1, i + 2, i + 3]);
        }
        self.index_buffer = Some(IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &indices,
        )?);
        Ok(())
    }

    fn is_key_down(&self, key: &KeyCode) -> bool {
        self.keys.get(key).unwrap_or(&false).to_owned()
    }
}

impl RenderController for Blocques {
    fn on_key_event(&mut self, key_event: KeyboardInput) {
        if let Some(key) = key_event.virtual_keycode {
            if let ElementState::Pressed = key_event.state {
                match key {
                    KeyCode::R => {
                        println!(
                            "Position {:?}; rotation {:?}",
                            self.camera_pos, self.camera_rot
                        );
                    }
                    _ => {}
                }
            }
            self.keys.insert(
                key,
                match key_event.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                },
            );
        }
    }

    fn on_frame(&mut self, _total_elapsed: f32, elapsed: f32) {
        let rotation_change = elapsed * PI / 2.0;
        if self.is_key_down(&KeyCode::Left) {
            self.camera_rot.1 += rotation_change;
        }
        if self.is_key_down(&KeyCode::Right) {
            self.camera_rot.1 -= rotation_change;
        }

        if self.is_key_down(&KeyCode::Up) {
            self.camera_rot.0 += rotation_change;
            if self.camera_rot.0 > PI / 2.0 {
                self.camera_rot.0 = PI / 2.0;
            }
        }
        if self.is_key_down(&KeyCode::Down) {
            self.camera_rot.0 -= rotation_change;
            if self.camera_rot.0 < -PI / 2.0 {
                self.camera_rot.0 = -PI / 2.0;
            }
        }

        let (rx, ry, rz) = self.camera_rot;

        let mut movement: Vector3<f32> = nalgebra::zero();
        if self.is_key_down(&KeyCode::A) {
            movement.x -= 1.0;
        }
        if self.is_key_down(&KeyCode::D) {
            movement.x += 1.0;
        }
        if self.is_key_down(&KeyCode::W) {
            movement.z -= 1.0;
        }
        if self.is_key_down(&KeyCode::S) {
            movement.z += 1.0;
        }
        if self.is_key_down(&KeyCode::LShift) {
            movement.y -= 1.0;
        }
        if self.is_key_down(&KeyCode::Space) {
            movement.y += 1.0;
        }
        if movement.magnitude() > 0.0 {
            self.camera_pos += UnitQuaternion::from_axis_angle(&Vector3::y_axis(), ry)
                * movement.normalize().scale(elapsed * 2.0);
        }

        self.view = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -rz)
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -rx)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -ry)
            * Translation3::from(self.camera_pos.scale(-1.0));
    }

    fn get_values(&self) -> RenderValues {
        RenderValues {
            vertex_buffer: &self.vertex_buffer,
            indices: match &self.index_buffer {
                Some(buffer) => IndicesSource::from(buffer),
                None => IndicesSource::NoIndices {
                    primitives: PrimitiveType::TriangleStrip,
                },
            },
            model: &self.model,
            view: &self.view,
            sampler: self
                .texture
                .sampled()
                .magnify_filter(MagnifySamplerFilter::Nearest)
                .minify_filter(MinifySamplerFilter::Nearest),
            background_colour: self.background_colour,
            fov: self.fov,
            near: self.near,
            far: self.far,
        }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let renderer = Renderer::new();

    let image = utils::load_image(include_bytes!("./assets/blocques2.png"));
    let texture = Texture2d::new(&renderer.display, image)?;

    let mut controller = Blocques::new(texture, &renderer.display)?;
    controller.camera_rot.1 = -3.0 * PI / 4.0;
    controller.camera_rot.0 = PI / 4.0;
    controller.world.generate_chunk((0, 0, 0));
    controller.world.set_block(
        (2, 2, 2),
        if let Block::Empty = controller.world.get_block((2, 2, 2)) {
            Block::Filled
        } else {
            Block::Empty
        },
    );
    controller
        .world
        .generate_vertices_for_chunks(vec![(0, 0, 0)], &controller.texture_info);
    controller.update_vertices_for_chunks(vec![(0, 0, 0)], &renderer.display)?;
    renderer.start(Box::new(controller));
    Ok(())
}
