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

pub const FACES: [Face; 6] = [
    XNeg,
    XPos,
    YNeg,
    YPos,
    ZNeg,
    ZPos,
];

// Size of face
const FACE: f32 = 1.0;

// Given the lower coordinates (ie rounded down)
pub fn vertices(face: Face, (x, y, z): (f32, f32, f32), texture_info: SubTextureInfo) -> Vec<Vertex> {
    // Here lies code that I write once and will never be able to read again
    // Nonetheless, a reminder that -Z is in the forwards direction of the camera
    vec![
        Vertex {
            position: match face {
                XNeg => [x, y + FACE, z + FACE],
                XPos => [x + FACE, y + FACE, z],
                YNeg => [x, y, z],
                YNeg => [x + FACE, y + FACE, z],
                ZNeg => [x + FACE, y + FACE, z],
                ZPos => [x, y + FACE, z + FACE],
            },
            tex_coords: [texture_info.x + texture_info.size, texture_info.y + texture_info.size],
        },
        Vertex {
            position: match face {
                XNeg => [x, y, z + FACE],
                XPos => [x + FACE, y, z],
                YNeg => [x, y, z + FACE],
                YNeg => [x + FACE, y + FACE, z + FACE],
                ZNeg => [x + FACE, y, z],
                ZPos => [x, y, z + FACE],
            },
            tex_coords: [texture_info.x + texture_info.size, texture_info.y],
        },
        Vertex {
            position: match face {
                XNeg => [x, y, z],
                XPos => [x + FACE, y, z + FACE],
                YNeg => [x + FACE, y, z + FACE],
                YNeg => [x, y + FACE, z + FACE],
                ZNeg => [x, y, z],
                ZPos => [x + FACE, y, z + FACE],
            },
            tex_coords: [texture_info.x, texture_info.y],
        },
        Vertex {
            position: match face {
                XNeg => [x, y + FACE, z],
                XPos => [x + FACE, y + FACE, z + FACE],
                YNeg => [x + FACE, y, z],
                YNeg => [x, y + FACE, z],
                ZNeg => [x, y + FACE, z],
                ZPos => [x + FACE, y + FACE, z + FACE],
            },
            tex_coords: [texture_info.x, texture_info.y + texture_info.size],
        },
    ]
}
