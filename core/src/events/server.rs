use crate::world::WorldCoordinate;

use math::random::Seed;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerEvent {
    PlayerConnected { id: u128 },
    PlayerDisconnected { id: u128 },
    PlayerMoved { id: u128, position: WorldCoordinate },
    ServerInfo { seed: Seed, player_ids: Vec<u128> },
}
