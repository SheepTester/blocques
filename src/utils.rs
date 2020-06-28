use glium::{implement_vertex, texture::RawImage2d};
use image::ImageFormat;
use std::io::Cursor;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct SubTextureInfo {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

pub fn load_image<'a>(bytes: &[u8]) -> RawImage2d<'a, u8> {
    let image = image::load(Cursor::new(bytes), ImageFormat::Png)
        .unwrap()
        .to_rgba();
    let image_dimensions = image.dimensions();
    RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}
