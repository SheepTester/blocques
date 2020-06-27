#[derive(Clone, Copy)]
pub enum Block {
    Empty,
    Filled,
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty
    }
}
