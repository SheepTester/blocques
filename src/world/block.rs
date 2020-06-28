mod face;

use super::super::utils::{Vertex, SubTextureInfo, FACES};
use super::WorldCoord;
use std::iter;

#[derive(Clone, Copy)]
pub enum Block {
    Empty,
    Filled,
}

impl Block {
    pub fn get_vertices(&self, (x, y, z): WorldCoord, texture_info: SubTextureInfo) -> Iterator<Vertex> {
        let float_coords = (x as f32, y as f32, z as f32);
        match self {
            Empty => iter::empty(),
            Filled => {
                FACES.iter().filter_map(|face| {
                    if is_next_to_transp_block {
                        Some(face::vertices(face, float_coords, texture_info))
                    } else {
                        None
                    }
                })
            },
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty
    }
}
