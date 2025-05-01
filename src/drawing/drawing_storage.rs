use std::collections::HashMap;

use crate::drawing::domain::bezier_c0::BezierC0;
use crate::drawing::domain::bezier_c2::BezierC2;
use crate::drawing::domain::bezier_int::BezierInt;
use crate::drawing::domain::surface_c0::SurfaceC0;
use crate::drawing::domain::surface_c2::SurfaceC2;

use super::domain::gregory::Gregory;
use super::domain::intersection::Intersection;
use super::domain::torus::Torus;

pub struct DrawingStorage {
    pub toruses: HashMap<u64, Torus>,
    pub beziers_c0: HashMap<u64, BezierC0>,
    pub beziers_c2: HashMap<u64, BezierC2>,
    pub beziers_int: HashMap<u64, BezierInt>,
    pub surfaces_c0: HashMap<u64, SurfaceC0>,
    pub surfaces_c2: HashMap<u64, SurfaceC2>,
    pub gregories: HashMap<u64, Gregory>,
    pub intersections: HashMap<u64, Intersection>,
}

impl DrawingStorage {
    pub fn new() -> Self {
        Self {
            toruses: HashMap::new(),
            beziers_c0: HashMap::new(),
            beziers_c2: HashMap::new(),
            beziers_int: HashMap::new(),
            surfaces_c0: HashMap::new(),
            surfaces_c2: HashMap::new(),
            gregories: HashMap::new(),
            intersections: HashMap::new(),
        }
    }
}
