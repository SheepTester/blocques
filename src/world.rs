mod block;
mod chunk;

use chunk::{Chunk, ChunkCoord};
use std::collections::HashMap;

pub struct World {
    chunks: HashMap<ChunkCoord, Chunk>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: HashMap::new(),
        }
    }
}
