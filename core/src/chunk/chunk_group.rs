use crate::block::Block;
use crate::chunk::{Chunk, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};

pub struct ChunkGroup {
    pub current: Chunk,
    pub north: Chunk,
    pub south: Chunk,
    pub east: Chunk,
    pub west: Chunk,
}

impl ChunkGroup {
    pub fn new(current: Chunk, north: Chunk, south: Chunk, east: Chunk, west: Chunk) -> Self {
        Self {
            current,
            north,
            south,
            east,
            west,
        }
    }

    pub fn get_block(&self, x: i8, y: i16, z: i8) -> Option<Block> {
        if y < 0 || y > CHUNK_HEIGHT as i16 {
            return None;
        }

        let y = y as usize;

        if x < 0 {
            return Some(
                self.east
                    .block((x + CHUNK_WIDTH as i8) as usize, y, z as usize),
            );
        }

        let x = x as usize;

        if z < 0 {
            return Some(self.south.block(x, y, (z + CHUNK_DEPTH as i8) as usize));
        }

        let z = z as usize;

        if x >= CHUNK_WIDTH {
            return Some(self.west.block(x - CHUNK_WIDTH, y, z));
        }

        if z >= CHUNK_DEPTH {
            return Some(self.north.block(x, y, z - CHUNK_DEPTH));
        }

        return Some(self.current.block(x, y, z));
    }
}
