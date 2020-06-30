mod adjacent_manager;
mod chunkarray;

use super::block::Block;
use super::{WorldCoord, WorldPos};
use crate::utils::{SubTextureInfo, Vertex};
pub use adjacent_manager::AdjacentChunkManager;
pub use chunkarray::{ChunkArray, CHUNK_SIZE};

pub type ChunkPos = isize;
pub type ChunkCoord = (ChunkPos, ChunkPos, ChunkPos);

pub type BlockPos = u8;
pub type BlockCoord = (BlockPos, BlockPos, BlockPos);

pub struct Chunk {
    blocks: ChunkArray<Block>,
    pub vertices: ChunkArray<Vec<Vertex>>,
    location: ChunkCoord,
}

impl Chunk {
    pub fn new(location: ChunkCoord) -> Self {
        Chunk {
            blocks: ChunkArray::new(),
            vertices: ChunkArray::new(),
            location,
        }
    }

    fn to_world_coords(&self, (bx, by, bz): BlockCoord) -> WorldCoord {
        let chunk_size = CHUNK_SIZE as WorldPos;
        let (cx, cy, cz) = self.location;
        (
            (cx as WorldPos) * chunk_size + (bx as WorldPos),
            (cy as WorldPos) * chunk_size + (by as WorldPos),
            (cz as WorldPos) * chunk_size + (bz as WorldPos),
        )
    }

    pub fn get_local_block(&self, pos: BlockCoord) -> Block {
        self.blocks.get(pos).clone()
    }

    pub fn set_local_block(&mut self, pos: BlockCoord, block: Block) {
        self.blocks.set(pos, block);
        // TODO: Update vertices?
    }

    pub fn generate_all_vertices<'a>(
        &'a self,
        texture_info: &'a SubTextureInfo,
        adj_chunk_manager: AdjacentChunkManager<'a>,
    ) -> ChunkArray<Vec<Vertex>> {
        self.blocks.map(|pos, block| {
            block.get_vertices(
                self.to_world_coords(pos),
                pos,
                texture_info,
                &adj_chunk_manager,
            )
        })
    }

    pub fn update_generated_vertices(&mut self, generated: ChunkArray<Vec<Vertex>>) {
        self.vertices = generated;
    }
}
