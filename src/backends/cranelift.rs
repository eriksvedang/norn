use crate::backend::Backend;

pub struct CraneliftBackend;

impl CraneliftBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Backend for CraneliftBackend {}
