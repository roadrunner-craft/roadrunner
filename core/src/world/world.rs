use crate::chunk::{Chunk, ChunkGrid, ChunkGridCoordinate, ChunkGroup};
use crate::utils::ThreadPool;
use crate::world::generation::generate_chunk;
use crate::world::generation::WorldSeed;
use crate::world::WorldCoordinate;

use math::random::Seed;
use std::collections::HashSet;
use std::sync::mpsc::{channel, Receiver, Sender};

#[cfg(debug_assertions)]
pub const LOAD_DISTANCE: u8 = 4;
#[cfg(not(debug_assertions))]
pub const LOAD_DISTANCE: u8 = 12;

type ChunkLoadingChannel = (Sender<Chunk>, Receiver<Chunk>);

pub struct World {
    pub chunks: ChunkGrid,
    world_seed: WorldSeed,

    // threading
    chunk_loading_chan: ChunkLoadingChannel,
    threadpool: ThreadPool,
    loading_chunks: HashSet<ChunkGridCoordinate>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: ChunkGrid::default(),
            world_seed: WorldSeed::new(),
            chunk_loading_chan: channel(),
            loading_chunks: HashSet::new(),
            threadpool: ThreadPool::new(1),
        }
    }

    pub fn from_seed(seed: Seed) -> Self {
        World {
            chunks: ChunkGrid::default(),
            world_seed: WorldSeed(seed),
            chunk_loading_chan: channel(),
            loading_chunks: HashSet::new(),
            threadpool: ThreadPool::new(1),
        }
    }

    pub fn seed(&self) -> WorldSeed {
        self.world_seed
    }

    pub fn load_chunk(&mut self, coords: ChunkGridCoordinate) {
        if !self.loading_chunks.contains(&coords) && !self.chunks.contains_key(&coords) {
            let seed = self.world_seed;

            // start a generating thread for the chunk
            let (sender, _) = &self.chunk_loading_chan;
            let tx = sender.clone();
            self.threadpool
                .run(move || tx.send(generate_chunk(coords, seed)).unwrap());
            self.loading_chunks.insert(coords);
        }
    }

    // TODO: add an update methode to remove this garbage code
    pub fn load_around(&mut self, positions: Vec<WorldCoordinate>) {
        // get back chunks from generating thread
        let mut received_chunks = 0;
        let (_, receiver) = &self.chunk_loading_chan;
        while let Ok(chunk) = receiver.try_recv() {
            self.loading_chunks.remove(&chunk.coords);
            self.chunks.insert(chunk.coords, chunk);
            received_chunks += 1;
        }

        // (un?)load chunks as the players move
        let mut chunks_to_load = HashSet::new();
        let mut chunks_to_keep = HashSet::new();
        for position in positions {
            let target_chunk = ChunkGridCoordinate::from_world_coordinate(position);

            let mut counter: u16 = 0;
            for i in 0..=(LOAD_DISTANCE + 1) as i16 {
                for x in -i..=i {
                    for z in -i..=i {
                        let coords = ChunkGridCoordinate::new(
                            target_chunk.x + x as i64,
                            target_chunk.z + z as i64,
                        );
                        if !self.chunks.contains_key(&coords) {
                            if counter < (received_chunks + 1) * 2 {
                                chunks_to_load.insert(coords);
                                counter += 1;
                            }
                        } else {
                            chunks_to_keep.insert(coords);
                        }
                    }
                }
            }

            self.chunks
                .retain(|coords, _| chunks_to_keep.contains(coords));
        }

        for coord in chunks_to_load {
            self.load_chunk(coord);
        }
    }

    pub fn get_chunk_group(&self, coords: ChunkGridCoordinate) -> Option<ChunkGroup> {
        let current = self.chunks.get(&coords)?.clone();
        let north = self.chunks.get(&coords.north())?.clone();
        let south = self.chunks.get(&coords.south())?.clone();
        let east = self.chunks.get(&coords.east())?.clone();
        let west = self.chunks.get(&coords.west())?.clone();

        Some(ChunkGroup::new(current, north, south, east, west))
    }
}
