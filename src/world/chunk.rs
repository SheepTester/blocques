use super::block::{Block, face::Face};
use crate::utils::{SubTextureInfo, Vertex};
use super::{World, WorldPos, WorldCoord};
use std::iter::Iterator;
use num_traits::PrimInt;

pub type ChunkPos = isize;
pub type ChunkCoord = (ChunkPos, ChunkPos, ChunkPos);

pub type BlockPos = u8;
pub type BlockCoord = (BlockPos, BlockPos, BlockPos);

pub fn apply_face<I>((x, y, z): (I, I, I), face: Face) -> (I, I, I) where I: PrimInt {
    match face {
        Face::XNeg => (x - I::one(), y, z),
        Face::XPos => (x + I::one(), y, z),
        Face::YNeg => (x, y - I::one(), z),
        Face::YPos => (x, y + I::one(), z),
        Face::ZNeg => (x, y, z - I::one()),
        Face::ZPos => (x, y, z + I::one()),
    }
}

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

    fn iter_blocks(&self) -> impl Iterator<Item = (BlockCoord, &Block)> + '_ {
        self.blocks.iter()
            .enumerate()
            .flat_map(move |(x, slice)| slice.iter()
                .enumerate()
                .flat_map(move |(y, column)| column.iter()
                    .enumerate()
                    .map(move |(z, block)| ((x as BlockPos, y as BlockPos, z as BlockPos), block))))
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

    pub fn get_vertices<'a>(&'a self, texture_info: &'a SubTextureInfo, adj_chunk_manager: AdjacentChunkManager<'a>) -> impl Iterator<Item = Vertex> + 'a {
        self.iter_blocks()
            .flat_map(move |(pos, block)| block.get_vertices(self.to_world_coords(pos), pos, texture_info, &adj_chunk_manager))
    }
}

pub struct AdjacentChunkManager<'a> {
    chunk: &'a Chunk,
    xneg_chunk: Option<&'a Chunk>,
    xpos_chunk: Option<&'a Chunk>,
    yneg_chunk: Option<&'a Chunk>,
    ypos_chunk: Option<&'a Chunk>,
    zneg_chunk: Option<&'a Chunk>,
    zpos_chunk: Option<&'a Chunk>,
}

impl<'a> AdjacentChunkManager<'a> {
    pub fn from_world(world: &'a World, chunk: &'a Chunk) -> Self {
        AdjacentChunkManager {
            chunk: chunk,
            xneg_chunk: world.get_chunk(&apply_face(chunk.location, Face::XNeg)),
            xpos_chunk: world.get_chunk(&apply_face(chunk.location, Face::XPos)),
            yneg_chunk: world.get_chunk(&apply_face(chunk.location, Face::YNeg)),
            ypos_chunk: world.get_chunk(&apply_face(chunk.location, Face::YPos)),
            zneg_chunk: world.get_chunk(&apply_face(chunk.location, Face::ZNeg)),
            zpos_chunk: world.get_chunk(&apply_face(chunk.location, Face::ZPos)),
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
            _ => (Some(self.chunk), apply_face(block_pos, face)),
        };
        match maybe_chunk {
            Some(chunk) => chunk.get_local_block(pos),
            None => Block::default(),
        }
    }
}
