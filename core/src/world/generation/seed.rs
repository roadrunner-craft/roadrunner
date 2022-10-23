use crate::chunk::ChunkGridCoordinate;

use math::random::Seed;
use std::num::Wrapping;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WorldSeed(pub Seed);

impl WorldSeed {
    pub fn new() -> Self {
        Self(Seed::new())
    }

    pub fn to_chunk_seed(&self, coords: ChunkGridCoordinate) -> ChunkSeed {
        ChunkSeed::new(&self.0, coords)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChunkSeed(pub Seed);

impl ChunkSeed {
    pub fn new(seed: &Seed, coords: ChunkGridCoordinate) -> Self {
        // https://en.wikipedia.org/wiki/Pairing_function#Cantor_pairing_function
        let k1 = Wrapping(coords.x as u64);
        let k2 = Wrapping(coords.z as u64);

        let value = Wrapping(((k1 + k2) * (k1 + k2 + Wrapping(1))).0 / 2) + k2;
        let seed = Wrapping(seed.0 as u64);

        Self(Seed(((seed + value).0 & 0xffffffff) as u32))
    }
}
