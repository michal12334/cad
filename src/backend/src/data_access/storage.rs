use std::collections::HashMap;
use crate::domain::mesh::Mesh;
use crate::domain::selected_object::SelectedObject;
use crate::domain::torus::Torus;
use crate::domain::transformer::Transformer;

pub struct Storage {
    pub toruses: HashMap<u64, Torus>,
    pub selected_objects: Vec<SelectedObject>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            toruses: HashMap::new(),
            selected_objects: Vec::new(),
        }
    }
}
