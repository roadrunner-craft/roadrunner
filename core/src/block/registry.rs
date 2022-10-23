use crate::block::BlockProperties;

use std::collections::HashMap;

#[derive(Clone)]
pub struct BlockRegistry {
    data: HashMap<u8, BlockProperties>,
}

impl BlockRegistry {
    pub fn new(data: HashMap<u8, BlockProperties>) -> Self {
        Self { data }
    }

    pub fn properties(&self, id: u8) -> Option<&BlockProperties> {
        self.data.get(&id)
    }

    pub fn is_opaque(&self, id: u8) -> bool {
        if let Some(p) = self.properties(id) {
            p.opaque
        } else {
            false
        }
    }
}
