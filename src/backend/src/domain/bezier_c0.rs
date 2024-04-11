pub struct BezierC0 {
    pub id: u64,
    pub name: String,
    pub points: Vec<BezierC0Point>,
}

pub struct BezierC0Point {
    pub id: u64,
}

impl BezierC0 {
    pub fn new(id: u64, points: Vec<BezierC0Point>) -> Self {
        Self {
            id,
            name: format!("BezierC0 {}", id),
            points,
        }
    }

    pub fn add_point(&mut self, point: BezierC0Point) {
        self.points.push(point);
    }
}
