use crate::block::Block;
use crate::chunk::ChunkGridCoordinate;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT;

type Blocks = Vec<Block>;

/// computes flat array index from a 3d coordinate
#[inline]
fn at(x: usize, y: usize, z: usize) -> usize {
    (x * CHUNK_WIDTH) + (y * CHUNK_HEIGHT) + z
}

#[derive(Clone)]
pub struct Chunk {
    blocks: Blocks,
    pub coords: ChunkGridCoordinate,
}

impl Chunk {
    pub fn new(coords: ChunkGridCoordinate) -> Self {
        Self {
            blocks: vec![Block { id: 0 }; CHUNK_SIZE],
            coords,
        }
    }

    /// returns the block at relative position x, y ,z
    pub fn block(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[at(x, y, z)]
    }

    /// replaces the block at relative position x, y, z
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[at(x, y, z)] = block
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn at_returns_within_volume() {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                for y in 0..CHUNK_HEIGHT {
                    let i = at(x, y, z);
                    assert!((0..CHUNK_SIZE).contains(&i));
                }
            }
        }
    }

    #[test]
    fn at_returns_unique_values() {
        let mut s = std::collections::HashSet::new();
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                for y in 0..CHUNK_HEIGHT {
                    let i = at(x, y, z);
                    assert!(!s.contains(&i));
                    s.insert(i);
                }
            }
        }
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_at(b: &mut Bencher) {
        b.iter(|| {
            let mut chunk = Chunk::new(ChunkGridCoordinate { x: 0, z: 0 });
            for x in 0..CHUNK_WIDTH {
                for z in 0..CHUNK_DEPTH {
                    for y in 0..CHUNK_HEIGHT {
                        chunk.set_block(x, y, z, Block { id: 1 });
                    }
                }
            }
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_flat_vec(b: &mut Bencher) {
        b.iter(|| vec![Block { id: 0 }; CHUNK_SIZE]);
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_nested_vec(b: &mut Bencher) {
        b.iter(|| vec![vec![vec![Block { id: 0 }; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH]);
    }
}
