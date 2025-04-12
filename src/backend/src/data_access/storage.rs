use std::collections::HashMap;

use crate::domain::bezier_c0::BezierC0;
use crate::domain::bezier_c2::BezierC2;
use crate::domain::bezier_int::BezierInt;
use crate::domain::cursor::Cursor;
use crate::domain::gregory::Gregory;
use crate::domain::intersection::Intersection;
use crate::domain::point::Point;
use crate::domain::selected_object::SelectedObject;
use crate::domain::surface_c0::SurfaceC0;
use crate::domain::surface_c2::SurfaceC2;
use crate::domain::torus::Torus;

pub struct Storage {
    pub toruses: HashMap<u64, Torus>,
    pub points: HashMap<u64, Point>,
    pub beziers_c0: HashMap<u64, BezierC0>,
    pub beziers_c2: HashMap<u64, BezierC2>,
    pub beziers_int: HashMap<u64, BezierInt>,
    pub surfaces_c0: HashMap<u64, SurfaceC0>,
    pub surfaces_c2: HashMap<u64, SurfaceC2>,
    pub gregories: HashMap<u64, Gregory>,
    pub intersections: HashMap<u64, Intersection>,
    pub selected_objects: Vec<SelectedObject>,
    pub cursor: Cursor,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            toruses: HashMap::new(),
            points: HashMap::new(),
            beziers_c0: HashMap::new(),
            beziers_c2: HashMap::new(),
            beziers_int: HashMap::new(),
            surfaces_c0: HashMap::new(),
            surfaces_c2: HashMap::new(),
            gregories: HashMap::new(),
            intersections: HashMap::new(),
            selected_objects: Vec::new(),
            cursor: Cursor::new(),
        }
    }
}
