use std::collections::HashMap;
use crate::domain::bezier_c0::BezierC0;

use crate::domain::cursor::Cursor;
use crate::domain::point::Point;
use crate::domain::selected_object::SelectedObject;
use crate::domain::torus::Torus;

pub struct Storage {
    pub toruses: HashMap<u64, Torus>,
    pub points: HashMap<u64, Point>,
    pub beziers_c0: HashMap<u64, BezierC0>,
    pub selected_objects: Vec<SelectedObject>,
    pub cursor: Cursor,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            toruses: HashMap::new(),
            points: HashMap::new(),
            beziers_c0: HashMap::new(),
            selected_objects: Vec::new(),
            cursor: Cursor::new(),
        }
    }
}
