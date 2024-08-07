pub struct BezierC0 {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
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
            draw_polygon: false,
            points,
        }
    }

    pub fn new_with_name(id: u64, name: String, points: Vec<BezierC0Point>) -> Self {
        Self {
            id,
            name,
            draw_polygon: false,
            points,
        }
    }

    pub fn add_point(&mut self, point: BezierC0Point) {
        self.points.push(point);
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn delete_points(&mut self, points: &[u64]) {
        self.points.retain(|point| !points.contains(&point.id));
    }

    pub fn set_draw_polygon(&mut self, draw_polygon: bool) {
        self.draw_polygon = draw_polygon;
    }
}
