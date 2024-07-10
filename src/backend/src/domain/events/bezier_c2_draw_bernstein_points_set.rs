pub struct BezierC2DrawBernsteinPointsSet {
    pub bezier_id: u64,
    pub draw_bernstein_points: bool,
}

impl BezierC2DrawBernsteinPointsSet {
    pub fn new(bezier_id: u64, draw_bernstein_points: bool) -> Self {
        Self {
            bezier_id,
            draw_bernstein_points,
        }
    }
}
