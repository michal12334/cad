use std::collections::HashMap;
use crate::domain::mesh::Mesh;
use crate::domain::torus::Torus;
use crate::domain::transformer::Transformer;

pub struct Storage {
    pub toruses: HashMap<u64, Torus>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            toruses: HashMap::new(),
        }
    }
}
