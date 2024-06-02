use std::collections::HashMap;
use crate::drawing::domain::bezier_c0::BezierC0;

pub struct DrawingStorage {
    pub beziers_c0: HashMap<u64, BezierC0>
}

impl DrawingStorage {
    pub fn new() -> Self {
        Self {
            beziers_c0: HashMap::new()
        }
    }
}
