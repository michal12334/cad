pub struct BezierC0PointMoved {
    pub bezier_id: u64,
}

impl BezierC0PointMoved {
    pub fn new(bezier_id: u64) -> Self {
        Self { bezier_id }
    }
}
