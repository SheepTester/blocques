use super::{
    super::{
        block::{face::Face, Block},
        World,
    },
    chunkarray::CHUNK_SIZE,
    BlockCoord, BlockPos, Chunk, ChunkCoord,
};
use num_traits::PrimInt;

fn apply_face<I>((x, y, z): (I, I, I), face: Face) -> (I, I, I)
where
    I: PrimInt,
{
    match face {
        Face::XNeg => (x - I::one(), y, z),
        Face::XPos => (x + I::one(), y, z),
        Face::YNeg => (x, y - I::one(), z),
        Face::YPos => (x, y + I::one(), z),
        Face::ZNeg => (x, y, z - I::one()),
        Face::ZPos => (x, y, z + I::one()),
    }
}

pub struct AdjacentChunkManager<'a> {
    chunk: Option<&'a Chunk>,
    xneg_chunk: Option<&'a Chunk>,
    xpos_chunk: Option<&'a Chunk>,
    yneg_chunk: Option<&'a Chunk>,
    ypos_chunk: Option<&'a Chunk>,
    zneg_chunk: Option<&'a Chunk>,
    zpos_chunk: Option<&'a Chunk>,
}

impl<'a> AdjacentChunkManager<'a> {
    pub fn from_world(world: &'a World, location: ChunkCoord) -> Self {
        AdjacentChunkManager {
            chunk: world.get_chunk(location),
            xneg_chunk: world.get_chunk(apply_face(location, Face::XNeg)),
            xpos_chunk: world.get_chunk(apply_face(location, Face::XPos)),
            yneg_chunk: world.get_chunk(apply_face(location, Face::YNeg)),
            ypos_chunk: world.get_chunk(apply_face(location, Face::YPos)),
            zneg_chunk: world.get_chunk(apply_face(location, Face::ZNeg)),
            zpos_chunk: world.get_chunk(apply_face(location, Face::ZPos)),
        }
    }

    pub fn get_face(&self, block_pos: BlockCoord, face: Face) -> Block {
        let chunk_size = CHUNK_SIZE as BlockPos;
        let (x, y, z) = block_pos;
        let (maybe_chunk, pos) = match face {
            Face::XNeg if x == 0 => (self.xneg_chunk, (chunk_size - 1, y, z)),
            Face::XPos if x == chunk_size - 1 => (self.xpos_chunk, (0, y, z)),
            Face::YNeg if y == 0 => (self.yneg_chunk, (x, chunk_size - 1, z)),
            Face::YPos if y == chunk_size - 1 => (self.ypos_chunk, (x, 0, z)),
            Face::ZNeg if z == 0 => (self.zneg_chunk, (x, y, chunk_size - 1)),
            Face::ZPos if z == chunk_size - 1 => (self.zpos_chunk, (x, y, 0)),
            _ => (self.chunk, apply_face(block_pos, face)),
        };
        match maybe_chunk {
            Some(chunk) => chunk.get_local_block(pos),
            None => Block::NotGenerated,
        }
    }
}
