mod block;
mod chunk;

use crate::utils::{SubTextureInfo, Vertex};
use block::Block;
use chunk::{AdjacentChunkManager, BlockPos, Chunk, ChunkCoord, CHUNK_SIZE};
use std::{collections::HashMap, iter};

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

    pub fn add_chunk_vertices<'a>(
        &'a self,
        target: &mut Vec<Vertex>,
        chunk_coord: ChunkCoord,
        texture_info: &'a SubTextureInfo,
    ) {
        if let Some(chunk) = self.get_chunk(chunk_coord) {
            let adjacent_chunks = AdjacentChunkManager::from_world(self, chunk);
            target.extend(chunk.get_vertices(texture_info, adjacent_chunks));
        }
    }
}
