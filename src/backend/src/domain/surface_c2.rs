pub struct SurfaceC2 {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub tess_level: u8,
    pub points: Vec<SurfaceC2Point>,
}

#[derive(Clone)]
pub struct SurfaceC2Point {
    pub id: u64,
}

impl SurfaceC2 {
    pub fn new(id: u64, points: Vec<SurfaceC2Point>) -> Self {
        Self {
            id,
            name: format!("SurfaceC2 {}", id),
            draw_polygon: false,
            tess_level: 8,
            points,
        }
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_draw_polygon(&mut self, draw_polygon: bool) {
        self.draw_polygon = draw_polygon;
    }
    
    pub fn set_tess_level(&mut self, tess_level: u8) {
        self.tess_level = tess_level;
    }
}
