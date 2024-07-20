pub struct SurfaceC0 {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub points: Vec<SurfaceC0Point>,
}

pub struct SurfaceC0Point {
    pub id: u64,
}

impl SurfaceC0 {
    pub fn new(id: u64, points: Vec<SurfaceC0Point>) -> Self {
        Self {
            id,
            name: format!("SurfaceC0 {}", id),
            draw_polygon: false,
            points,
        }
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_draw_polygon(&mut self, draw_polygon: bool) {
        self.draw_polygon = draw_polygon;
    }
}
