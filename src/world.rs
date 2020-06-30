mod block;
mod chunk;

use crate::utils::{SubTextureInfo, Vertex};
pub use block::Block;
use chunk::{AdjacentChunkManager, BlockPos, Chunk, ChunkCoord, CHUNK_SIZE};
use std::collections::HashMap;

type WorldPos = isize;
type WorldCoord = (WorldPos, WorldPos, WorldPos);

pub struct World {
    chunks: HashMap<ChunkCoord, Chunk>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: HashMap::new(),
        }
    }

    fn get_chunk(&self, coord: ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(&coord)
    }

    fn get_chunk_mut(&mut self, coord: ChunkCoord) -> Option<&mut Chunk> {
        self.chunks.get_mut(&coord)
    }

    pub fn generate_chunk(&mut self, coord: ChunkCoord) {
        let chunk = Chunk::new(coord);
        self.chunks.insert(coord, chunk);
    }

    fn edit_chunk(&mut self, coord: ChunkCoord) -> &mut Chunk {
        if let None = self.chunks.get(&coord) {
            self.generate_chunk(coord);
        }
        self.chunks.get_mut(&coord).unwrap()
    }

    pub fn get_block(&self, (x, y, z): WorldCoord) -> Block {
        let chunk_size = CHUNK_SIZE as WorldPos;
        match self.get_chunk((x / chunk_size, y / chunk_size, z / chunk_size)) {
            Some(chunk) => chunk.get_local_block((
                (x % chunk_size) as BlockPos,
                (y % chunk_size) as BlockPos,
                (z % chunk_size) as BlockPos,
            )),
            None => Block::default(),
        }
    }

    pub fn generate_vertices_for_chunks<'a>(
        &'a mut self,
        chunk_coords: Vec<ChunkCoord>,
        texture_info: &'a SubTextureInfo,
    ) {
        for chunk_coord in chunk_coords {
            let generated = if let Some(chunk) = self.get_chunk(chunk_coord) {
                let adjacent_chunks = AdjacentChunkManager::from_world(self, chunk_coord);
                Some(chunk.generate_all_vertices(texture_info, adjacent_chunks))
            } else {
                None
            };
            // Updating separately in order to not mix a mutable reference with an immutable
            // reference
            if let (Some(generated), Some(chunk)) = (generated, self.get_chunk_mut(chunk_coord)) {
                chunk.update_generated_vertices(generated);
            }
        }
    }

    pub fn get_vertices_for_chunks(&self, chunk_coords: Vec<ChunkCoord>) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        for chunk_coord in chunk_coords {
            if let Some(chunk) = self.get_chunk(chunk_coord) {
                for face_vertices in chunk.vertices.iter_flat() {
                    vertices.extend(face_vertices);
                }
            }
        }
        vertices
    }

    pub fn set_block(&mut self, (x, y, z): WorldCoord, block: Block) {
        let chunk_size = CHUNK_SIZE as WorldPos;
        let chunk_pos = (x / chunk_size, y / chunk_size, z / chunk_size);
        let chunk = self.edit_chunk(chunk_pos);
        chunk.set_local_block(
            (
                (x % chunk_size) as BlockPos,
                (y % chunk_size) as BlockPos,
                (z % chunk_size) as BlockPos,
            ),
            block,
        );
    }
}
