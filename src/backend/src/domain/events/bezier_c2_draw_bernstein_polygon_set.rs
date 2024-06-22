pub struct BezierC2DrawBernsteinPolygonSet {
    pub bezier_id: u64,
    pub draw_bernstein_polygon: bool,
}

impl BezierC2DrawBernsteinPolygonSet {
    pub fn new(bezier_id: u64, draw_bernstein_polygon: bool) -> Self {
        Self { bezier_id, draw_bernstein_polygon, }
    }
}
