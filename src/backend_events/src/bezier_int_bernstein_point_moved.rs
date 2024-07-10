pub struct BezierIntBernsteinPointMoved {
    pub bezier_id: u64,
}

impl BezierIntBernsteinPointMoved {
    pub fn new(bezier_id: u64) -> Self {
        Self { bezier_id, }
    }
}
