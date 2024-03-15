use std::collections::HashMap;
use crate::domain::mesh::Mesh;
use crate::domain::torus::Torus;
use crate::domain::transformer::Transformer;

pub struct Storage {
    pub toruses: HashMap<u64, Torus>,
    pub transformers: HashMap<u64, Transformer>,
    pub meshes: HashMap<u64, Mesh>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            toruses: HashMap::new(),
            transformers: HashMap::new(),
            meshes: HashMap::new(),
        }
    }
}
