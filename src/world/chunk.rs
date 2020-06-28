use super::block::Block;

pub type ChunkPos = isize;
pub type ChunkCoord = (ChunkPos, ChunkPos, ChunkPos);
pub type BlockPos = u8;
pub type BlockCoord = (BlockPos, BlockPos, BlockPos);

pub const CHUNK_SIZE: usize = 16;

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

    pub fn get_local_block(&self, (x, y, z): BlockCoord) -> Block {
        self.blocks[x as usize][y as usize][z as usize]
    }
}
