use super::block::Block;
use super::utils::{SubTextureInfo, Vertex};
use super::{WorldPos, WorldCoord};

pub type ChunkPos = isize;
pub type ChunkCoord = (ChunkPos, ChunkPos, ChunkPos);
pub type BlockPos = u8;
pub type BlockCoord = (BlockPos, BlockPos, BlockPos);

pub const CHUNK_SIZE: usize = 16;

fn to_world_coords((cx, cy, cz): ChunkCoord, (bx, by, bz): BlockCoord) -> WorldCoord {
    let chunk_size = CHUNK_SIZE as BlockPos;
    (
        (cx as BlockPos) * chunk_size + (bx as BlockPos),
        (cy as BlockPos) * chunk_size + (by as BlockPos),
        (cz as BlockPos) * chunk_size + (bz as BlockPos)
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

    fn iter_blocks(&self) -> Iterator<Block> {
        self.blocks.iter()
            .enumerate()
            .map(|(x, slice)| slice.iter()
                .map(|(y, column)| column.iter()
                    .map(|(z, block)| ((x as BlockPos, y as BlockPos, z as BlockPos) block)))
                .flatten())
            .flatten()
    }

    pub fn get_local_block(&self, (x, y, z): BlockCoord) -> Block {
        self.blocks[x as usize][y as usize][z as usize]
    }

    pub fn get_vertices(&self, texture_info: SubTextureInfo) -> Iterator<Vertex> {
        self.iter_blocks()
            .map(|(pos, block)| block.get_vertices(to_world_coords(self.location, pos), texture_info))
            .flatten()
    }
}
