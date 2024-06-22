pub struct BezierC2DrawBSplinePolygonSet {
    pub bezier_id: u64,
    pub draw_b_spline_polygon: bool,
}

impl BezierC2DrawBSplinePolygonSet {
    pub fn new(bezier_id: u64, draw_b_spline_polygon: bool) -> Self {
        Self { bezier_id, draw_b_spline_polygon, }
    }
}
