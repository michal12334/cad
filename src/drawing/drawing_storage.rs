use crate::drawing::domain::bezier_c0::BezierC0;

pub struct DrawingStorage {
    pub beziers_c0: Vec<BezierC0>
}

impl DrawingStorage {
    pub fn new() -> Self {
        Self {
            beziers_c0: Vec::new()
        }
    }
}
