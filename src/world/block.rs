pub mod face;

use super::{
    chunk::{AdjacentChunkManager, BlockCoord},
    WorldCoord,
};
use crate::utils::{SubTextureInfo, Vertex};
use std::iter::Iterator;

#[derive(Clone, Copy)]
pub enum Block {
    Empty,
    Filled,
    NotGenerated,
}

impl Block {
    pub fn is_transparent(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Filled => false,
            Self::NotGenerated => false,
        }
    }

    // Isn't it nonideal to create a new SubTextureInfo every time?
    fn get_texture_info(&self) -> Option<SubTextureInfo> {
        match self {
            Self::Empty => None,
            Self::Filled => Some(SubTextureInfo {
                x: 0.0,
                y: 0.0,
                size: 1.0,
            }),
            Self::NotGenerated => None,
        }
    }

    pub fn get_vertices(
        &self,
        (x, y, z): WorldCoord,
        pos: BlockCoord,
        adj_chunk_manager: &AdjacentChunkManager,
    ) -> Vec<Vertex> {
        if self.is_transparent() {
            Vec::new()
        } else {
            let float_coords = (x as f32, y as f32, z as f32);
            face::FACES
                .iter()
                .filter_map(|face| {
                    if adj_chunk_manager.get_face(pos, *face).is_transparent() {
                        self.get_texture_info()
                            .map(|texture_info| face.vertices(float_coords, &texture_info))
                    } else {
                        None
                    }
                })
                .flatten()
                .collect()
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty
    }
}
