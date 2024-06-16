use std::collections::HashMap;
use crate::drawing::domain::bezier_c0::BezierC0;
use crate::drawing::domain::bezier_c2::BezierC2;

pub struct DrawingStorage {
    pub beziers_c0: HashMap<u64, BezierC0>,
    pub beziers_c2: HashMap<u64, BezierC2>,
}

impl DrawingStorage {
    pub fn new() -> Self {
        Self {
            beziers_c0: HashMap::new(),
            beziers_c2: HashMap::new(),
        }
    }
}
