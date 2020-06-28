use super::super::super::utils::{Vertex, SubTextureInfo};

// The face vertices should go clockwise:
// 4 $    * 1
// ^        |
// |        v
// 3 <----- 2

// [axis: X|Y|Z][dir: Neg|Pos]
// The axis will remain unchained. It'll be axis when Neg and axis + FACE when Pos.
pub enum Face {
    XNeg,
    XPos,
    YNeg,
    YPos,
    ZNeg,
    ZPos,
}

impl Face {
    // Given the lower coordinates (ie rounded down)
    pub fn vertices(&self, (x, y, z): (f32, f32, f32), texture_info: &SubTextureInfo) -> Vec<Vertex> {
        // Here lies code that I write once and will never be able to read again
        // Nonetheless, a reminder that -Z is in the forwards direction of the camera
        vec![
            Vertex {
                position: match self {
                    Face::XNeg => [x, y + FACE, z + FACE],
                    Face::XPos => [x + FACE, y + FACE, z],
                    Face::YNeg => [x, y, z],
                    Face::YNeg => [x + FACE, y + FACE, z],
                    Face::ZNeg => [x + FACE, y + FACE, z],
                    Face::ZPos => [x, y + FACE, z + FACE],
                },
                tex_coords: [texture_info.x + texture_info.size, texture_info.y + texture_info.size],
            },
            Vertex {
                position: match self {
                    Face::XNeg => [x, y, z + FACE],
                    Face::XPos => [x + FACE, y, z],
                    Face::YNeg => [x, y, z + FACE],
                    Face::YNeg => [x + FACE, y + FACE, z + FACE],
                    Face::ZNeg => [x + FACE, y, z],
                    Face::ZPos => [x, y, z + FACE],
                },
                tex_coords: [texture_info.x + texture_info.size, texture_info.y],
            },
            Vertex {
                position: match self {
                    Face::XNeg => [x, y, z],
                    Face::XPos => [x + FACE, y, z + FACE],
                    Face::YNeg => [x + FACE, y, z + FACE],
                    Face::YNeg => [x, y + FACE, z + FACE],
                    Face::ZNeg => [x, y, z],
                    Face::ZPos => [x + FACE, y, z + FACE],
                },
                tex_coords: [texture_info.x, texture_info.y],
            },
            Vertex {
                position: match self {
                    Face::XNeg => [x, y + FACE, z],
                    Face::XPos => [x + FACE, y + FACE, z + FACE],
                    Face::YNeg => [x + FACE, y, z],
                    Face::YNeg => [x, y + FACE, z],
                    Face::ZNeg => [x, y + FACE, z],
                    Face::ZPos => [x + FACE, y + FACE, z + FACE],
                },
                tex_coords: [texture_info.x, texture_info.y + texture_info.size],
            },
        ]
    }
}

pub const FACES: [Face; 6] = [
    Face::XNeg,
    Face::XPos,
    Face::YNeg,
    Face::YPos,
    Face::ZNeg,
    Face::ZPos,
];

// Size of face
const FACE: f32 = 1.0;
