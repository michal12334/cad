pub struct BezierC0PointMoved {
    pub bezier_id: u64,
    pub point_id: u64,
    pub position: (f64, f64, f64),
}

impl BezierC0PointMoved {
    pub fn new(bezier_id: u64, point_id: u64, position: (f64, f64, f64)) -> Self {
        Self { bezier_id, point_id, position }
    }
}
