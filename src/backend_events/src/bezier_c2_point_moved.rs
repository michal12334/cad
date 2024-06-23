pub struct BezierC2PointMoved {
    pub bezier_id: u64,
}

impl BezierC2PointMoved {
    pub fn new(bezier_id: u64) -> Self {
        Self { bezier_id,  }
    }
}
