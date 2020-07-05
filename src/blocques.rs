use crate::{
    rendering::{FrameInfo, RenderController, RenderValues, Renderer},
    utils::{self, Vertex},
    world::{Block, ChunkCoord, ChunkPos, World},
};
use glium::{
    glutin::event::{ElementState, KeyboardInput, VirtualKeyCode as KeyCode},
    index::{IndicesSource, PrimitiveType},
    texture::Texture2d,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    Display, IndexBuffer, VertexBuffer,
};
use nalgebra::{Isometry3, Similarity3, Translation3, UnitQuaternion, Vector3};
use std::{collections::HashMap, f32::consts::PI};
// https://stackoverflow.com/a/48431339
use failure::Error;

struct BlocquesOptions {
    vert_loaded_radius: u8,
    horiz_loaded_radius: u8,
}

impl Default for BlocquesOptions {
    fn default() -> Self {
        Self {
            vert_loaded_radius: 1,
            horiz_loaded_radius: 1,
        }
    }
}

struct Blocques {
    world: World,
    vert_loaded_radius: ChunkPos,
    horiz_loaded_radius: ChunkPos,

    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: Option<IndexBuffer<u32>>,
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
    fn new(texture: Texture2d, display: &Display, options: BlocquesOptions) -> Result<Self, Error> {
        let vertices = vec![];
        Ok(Blocques {
            world: World::new(),
            vert_loaded_radius: options.vert_loaded_radius as ChunkPos,
            horiz_loaded_radius: options.horiz_loaded_radius as ChunkPos,

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
        chunk_coords: &Vec<ChunkCoord>,
        display: &Display,
    ) -> Result<(), Error> {
        let vertices = self.world.get_vertices_for_chunks(chunk_coords);
        self.vertex_buffer = VertexBuffer::new(display, &vertices)?;

        let squares = vertices.len() / 4;
        let mut indices = Vec::with_capacity(squares * 6);
        for square in 0..squares {
            let i = square as u32 * 4;
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

    fn on_frame(&mut self, info: FrameInfo) {
        let FrameInfo {
            elapsed, display, ..
        } = info;
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
                * movement.normalize().scale(elapsed * 4.0);
        }

        self.view = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -rz)
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -rx)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -ry)
            * Translation3::from(self.camera_pos.scale(-1.0));

        let vert_loaded_radius = self.vert_loaded_radius;
        let horiz_loaded_radius = self.horiz_loaded_radius;
        let loaded_chunks: Vec<ChunkCoord> = (-horiz_loaded_radius..=horiz_loaded_radius)
            .flat_map(|dx| {
                (-horiz_loaded_radius..=horiz_loaded_radius).flat_map(move |dz| {
                    (-vert_loaded_radius..=vert_loaded_radius).map(move |dy| (dx, dy, dz))
                })
            })
            .collect();
        for chunk in &loaded_chunks {
            self.world.ensure_ready_chunk(*chunk);
        }
        if self.world.changed {
            // Ignores error
            if let Ok(()) = self.update_vertices_for_chunks(&loaded_chunks, display) {
                self.world.changed = false;
            }
        }
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

pub fn main() -> Result<(), Error> {
    let renderer = Renderer::new();

    let image = utils::load_image(include_bytes!("./assets/blocques2.png"));
    let texture = Texture2d::new(&renderer.display, image)?;

    let mut controller = Blocques::new(
        texture,
        &renderer.display,
        BlocquesOptions {
            vert_loaded_radius: 1,
            horiz_loaded_radius: 3,
        },
    )?;
    controller.camera_pos = Vector3::new(8.0, 14.0, 8.0);
    controller.world.ensure_ready_chunk((0, 0, 0));
    controller.world.set_block(
        (2, 2, 2),
        if let Block::Empty = controller.world.get_block((2, 2, 2)) {
            Block::Filled
        } else {
            Block::Empty
        },
    );
    renderer.start(controller);
    Ok(())
}
