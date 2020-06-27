use super::block::Block;

pub type ChunkPos = i32;
pub type ChunkCoord = (ChunkPos, ChunkPos, ChunkPos);

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    blocks: [[Block; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            blocks: [[Block::default(); CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}
