use crate::chunk::Chunk;
use crate::chunk::{CHUNK_DEPTH, CHUNK_WIDTH};
use crate::world::WorldCoordinate;

use math::vector::Vector2;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(PartialEq, Eq, Hash, Default, Copy, Clone, Debug)]
pub struct ChunkGridCoordinate {
    pub x: i64,
    pub z: i64,
}

impl ChunkGridCoordinate {
    pub fn new(x: i64, z: i64) -> Self {
        Self { x, z }
    }

    pub fn from_world_coordinate(WorldCoordinate { x, z, .. }: WorldCoordinate) -> Self {
        Self {
            x: (x / 16.0).floor() as i64,
            z: (z / 16.0).floor() as i64,
        }
    }

    pub fn abs(&self) -> Vector2 {
        Vector2 {
            x: CHUNK_WIDTH as f32 * self.x as f32,
            y: CHUNK_DEPTH as f32 * self.z as f32,
        }
    }

    pub fn manhattan_distance(&self, other: ChunkGridCoordinate) -> Option<usize> {
        ((self.x - other.x).abs() + (self.z - other.z).abs())
            .try_into()
            .ok()
    }

    pub fn north(&self) -> ChunkGridCoordinate {
        ChunkGridCoordinate::new(self.x, self.z + 1)
    }

    pub fn south(&self) -> ChunkGridCoordinate {
        ChunkGridCoordinate::new(self.x, self.z - 1)
    }

    pub fn east(&self) -> ChunkGridCoordinate {
        ChunkGridCoordinate::new(self.x - 1, self.z)
    }

    pub fn west(&self) -> ChunkGridCoordinate {
        ChunkGridCoordinate::new(self.x + 1, self.z)
    }

    pub fn are_neighbours(left: &ChunkGridCoordinate, right: &ChunkGridCoordinate) -> bool {
        ((left.x - right.x).abs() == 1 && left.z - right.z == 0)
            || (left.x - right.x == 0 && (left.z - right.z).abs() == 1)
    }
}

pub type ChunkGrid = HashMap<ChunkGridCoordinate, Chunk>;
