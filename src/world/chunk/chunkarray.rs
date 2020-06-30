use super::{BlockCoord, BlockPos};

pub const CHUNK_SIZE: usize = 16;

#[derive(Default)]
pub struct ChunkArray<T>([[[T; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

impl<T: Default> ChunkArray<T> {
    pub fn new() -> Self {
        ChunkArray::default()
    }

    pub fn _map<O, F>(&self, map: F) -> ChunkArray<O>
    where
        O: Default,
        F: Fn(BlockCoord, &T) -> O,
    {
        let mut output: ChunkArray<O> = ChunkArray::new();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    output.0[x][y][z] = map(
                        (x as BlockPos, y as BlockPos, z as BlockPos),
                        &self.0[x][y][z],
                    );
                }
            }
        }
        output
    }

    pub fn _iter_flat(&self) -> impl Iterator<Item = &T> + '_ {
        self.0
            .iter()
            .flat_map(|slice| slice.iter().flat_map(|column| column.iter()))
    }

    pub fn iter_flat_coords(&self) -> impl Iterator<Item = (BlockCoord, &T)> + '_ {
        self.0.iter().enumerate().flat_map(move |(x, slice)| {
            slice.iter().enumerate().flat_map(move |(y, column)| {
                column
                    .iter()
                    .enumerate()
                    .map(move |(z, block)| ((x as BlockPos, y as BlockPos, z as BlockPos), block))
            })
        })
    }

    pub fn get(&self, (x, y, z): BlockCoord) -> &T {
        &self.0[x as usize][y as usize][z as usize]
    }

    pub fn set(&mut self, (x, y, z): BlockCoord, new: T) {
        self.0[x as usize][y as usize][z as usize] = new;
    }
}
