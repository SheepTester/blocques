mod face;

use super::super::utils::{Vertex, SubTextureInfo};
use super::WorldCoord;
use std::iter::{self, Iterator};

#[derive(Clone, Copy)]
pub enum Block {
    Empty,
    Filled,
}

impl Block {
    pub fn is_transparent(&self) -> bool {
        match self {
            Empty => true,
            Filled => false,
        }
    }

    pub fn get_vertices(&self, (x, y, z): WorldCoord, texture_info: &SubTextureInfo) -> Box<dyn Iterator<Item = Vertex>> {
        if self.is_transparent() {
            Box::new(iter::empty::<Vertex>())
        } else {
            let float_coords = (x as f32, y as f32, z as f32);
            Box::new(face::FACES.iter().filter_map(|face| {
                // TODO: is_next_to_transp_block
                if false {
                    Some(face.vertices(float_coords, texture_info))
                } else {
                    None
                }
            }))
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty
    }
}
