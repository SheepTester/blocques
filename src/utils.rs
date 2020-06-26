use glium::{implement_vertex, texture::RawImage2d};
use image::ImageFormat;
use std::io::Cursor;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub fn load_image<'a>(bytes: &[u8]) -> RawImage2d<'a, u8> {
    let image = image::load(
        Cursor::new(bytes),
        ImageFormat::Png,
    ).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}
