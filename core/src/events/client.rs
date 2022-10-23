use serde::{Deserialize, Serialize};

use crate::world::WorldCoordinate;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientEvent {
    PlayerConnect,
    PlayerDisconnect,
    PlayerMove { position: WorldCoordinate },
}
