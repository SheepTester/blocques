mod block;
mod chunk;

use block::Block;
use chunk::{Chunk, ChunkCoord, CHUNK_SIZE, BlockPos, AdjacentChunkManager};
use std::collections::HashMap;
use crate::utils::{SubTextureInfo, Vertex};

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

    fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(&coord)
    }

    pub fn get_block(&self, (x, y, z): WorldCoord) -> Block {
        let chunk_size = CHUNK_SIZE as WorldPos;
        match self.get_chunk(&(x / chunk_size, y / chunk_size, z / chunk_size)) {
            Some(chunk) => chunk.get_local_block(((x % chunk_size) as BlockPos, (y % chunk_size) as BlockPos, (z % chunk_size) as BlockPos)),
            None => Block::default(),
        }
    }

    pub fn generate_vertices<'a>(&'a self, chunk: &'a Chunk, texture_info: &'a SubTextureInfo) -> impl Iterator<Item = Vertex> + 'a {
        let adjacent_chunks = AdjacentChunkManager::from_world(self, chunk);
        chunk.get_vertices(texture_info, adjacent_chunks)
    }
}
