use super::block::Block;
use super::super::utils::{SubTextureInfo, Vertex};
use super::{WorldPos, WorldCoord};
use std::iter::Iterator;

pub type ChunkPos = isize;
pub type ChunkCoord = (ChunkPos, ChunkPos, ChunkPos);
pub type BlockPos = u8;
pub type BlockCoord = (BlockPos, BlockPos, BlockPos);

pub const CHUNK_SIZE: usize = 16;

fn to_world_coords((cx, cy, cz): ChunkCoord, (bx, by, bz): BlockCoord) -> WorldCoord {
    let chunk_size = CHUNK_SIZE as WorldPos;
    (
        (cx as WorldPos) * chunk_size + (bx as WorldPos),
        (cy as WorldPos) * chunk_size + (by as WorldPos),
        (cz as WorldPos) * chunk_size + (bz as WorldPos)
    )
}

pub struct Chunk {
    blocks: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    location: ChunkCoord,
}

impl Chunk {
    pub fn new(location: ChunkCoord) -> Self {
        Chunk {
            blocks: [[[Block::default(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            location: location,
        }
    }

    fn iter_blocks(&self) -> impl Iterator<Item = (BlockCoord, &Block)> + '_ {
        self.blocks.iter()
            .enumerate()
            .flat_map(move |(x, slice)| slice.iter()
                .enumerate()
                .flat_map(move |(y, column)| column.iter()
                    .enumerate()
                    .map(move |(z, block)| ((x as BlockPos, y as BlockPos, z as BlockPos), block))))
    }

    pub fn get_local_block(&self, (x, y, z): BlockCoord) -> Block {
        self.blocks[x as usize][y as usize][z as usize]
    }

    pub fn get_vertices<'a>(&'a self, texture_info: &'a SubTextureInfo) -> impl Iterator<Item = Vertex> + 'a {
        self.iter_blocks()
            .flat_map(move |(pos, block)| block.get_vertices(to_world_coords(self.location, pos), texture_info))
    }
}
