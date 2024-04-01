use crate::domain::cursor::Cursor;
use crate::domain::point::Point;
use crate::domain::selected_object::SelectedObject;
use crate::domain::torus::Torus;
use std::collections::HashMap;

pub struct Storage {
    pub toruses: HashMap<u64, Torus>,
    pub points: HashMap<u64, Point>,
    pub selected_objects: Vec<SelectedObject>,
    pub cursor: Cursor,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            toruses: HashMap::new(),
            points: HashMap::new(),
            selected_objects: Vec::new(),
            cursor: Cursor::new(),
        }
    }
}
