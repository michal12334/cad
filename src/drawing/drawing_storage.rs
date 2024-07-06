use std::collections::HashMap;
use crate::drawing::domain::bezier_c0::BezierC0;
use crate::drawing::domain::bezier_c2::BezierC2;
use crate::drawing::domain::bezier_int::BezierInt;

pub struct DrawingStorage {
    pub beziers_c0: HashMap<u64, BezierC0>,
    pub beziers_c2: HashMap<u64, BezierC2>,
    pub beziers_int: HashMap<u64, BezierInt>,
}

impl DrawingStorage {
    pub fn new() -> Self {
        Self {
            beziers_c0: HashMap::new(),
            beziers_c2: HashMap::new(),
            beziers_int: HashMap::new(),
        }
    }
}
