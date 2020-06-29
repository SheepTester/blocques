mod adjacent_manager;
mod chunkarray;

use super::block::{face::Face, Block};
use super::{World, WorldCoord, WorldPos};
use crate::utils::{SubTextureInfo, Vertex};
pub use adjacent_manager::AdjacentChunkManager;
pub use chunkarray::{iter_flat, make_chunk_array, map_chunk_array, ChunkArray, CHUNK_SIZE};
use std::iter::Iterator;

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
            blocks: make_chunk_array(),
            vertices: make_chunk_array(),
            location,
        }
    }

    fn iter_blocks(&self) -> impl Iterator<Item = (BlockCoord, &Block)> + '_ {
        self.blocks.iter().enumerate().flat_map(move |(x, slice)| {
            slice.iter().enumerate().flat_map(move |(y, column)| {
                column
                    .iter()
                    .enumerate()
                    .map(move |(z, block)| ((x as BlockPos, y as BlockPos, z as BlockPos), block))
            })
        })
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

    pub fn get_local_block(&self, (x, y, z): BlockCoord) -> Block {
        self.blocks[x as usize][y as usize][z as usize]
    }

    pub fn set_local_block(&mut self, (x, y, z): BlockCoord, block: Block) {
        self.blocks[x as usize][y as usize][z as usize] = block;
        // TODO: Update vertices?
    }

    pub fn generate_all_vertices<'a>(
        &'a mut self,
        texture_info: &'a SubTextureInfo,
        adj_chunk_manager: AdjacentChunkManager<'a>,
    ) {
        self.vertices = map_chunk_array(&self.blocks, |pos, block| {
            block.get_vertices(
                self.to_world_coords(pos),
                pos,
                texture_info,
                &adj_chunk_manager,
            )
        })
    }
}
