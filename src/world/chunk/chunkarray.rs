use super::{BlockCoord, BlockPos};

pub const CHUNK_SIZE: usize = 16;

pub type ChunkArray<T> = [[[T; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub fn make_chunk_array<T>() -> ChunkArray<T>
where
    T: Default,
{
    ChunkArray::default()
}

pub fn map_chunk_array<A, B, F>(input: &ChunkArray<A>, map: F) -> ChunkArray<B>
where
    B: Default,
    F: Fn(BlockCoord, &A) -> B,
{
    let mut output = make_chunk_array::<B>();
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                output[x][y][z] = map(
                    (x as BlockPos, y as BlockPos, z as BlockPos),
                    &input[x][y][z],
                );
            }
        }
    }
    output
}

pub fn iter_flat<T>(array: &ChunkArray<T>) -> impl Iterator<Item = &T> + '_ {
    array
        .iter()
        .flat_map(|slice| slice.iter().flat_map(|column| column.iter()))
}

fn _iter_flat_map<T>(array: &ChunkArray<T>) -> impl Iterator<Item = (BlockCoord, &T)> + '_ {
    array.iter().enumerate().flat_map(move |(x, slice)| {
        slice.iter().enumerate().flat_map(move |(y, column)| {
            column
                .iter()
                .enumerate()
                .map(move |(z, block)| ((x as BlockPos, y as BlockPos, z as BlockPos), block))
        })
    })
}
