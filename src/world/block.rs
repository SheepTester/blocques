pub mod face;

use super::super::utils::{Vertex, SubTextureInfo};
use super::WorldCoord;
use std::iter::Iterator;

#[derive(Clone, Copy)]
pub enum Block {
    Empty,
    Filled,
}

impl Block {
    pub fn is_transparent(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Filled => false,
        }
    }

    pub fn get_vertices(&self, (x, y, z): WorldCoord, texture_info: &SubTextureInfo) -> Vec<Vertex> {
        if self.is_transparent() {
            Vec::new()
        } else {
            let float_coords = (x as f32, y as f32, z as f32);
            face::FACES.iter().filter_map(
                |face| {
                    // TODO: is_next_to_transp_block
                    if false {
                        Some(face.vertices(float_coords, texture_info))
                    } else {
                        None
                    }
                }
            ).flatten().collect()
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty
    }
}
