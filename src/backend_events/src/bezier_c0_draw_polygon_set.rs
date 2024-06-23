pub struct BezierC0DrawPolygonSet {
    pub bezier_id: u64,
    pub draw_polygon: bool,
}

impl BezierC0DrawPolygonSet {
    pub fn new(bezier_id: u64, draw_polygon: bool) -> Self {
        Self { bezier_id, draw_polygon, }
    }
}
