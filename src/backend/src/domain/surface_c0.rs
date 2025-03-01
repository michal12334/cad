pub struct SurfaceC0 {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub tess_level: u8,
    pub points: Vec<SurfaceC0Point>,
    pub size: (u32, u32),
    pub is_cylinder: bool,
}

#[derive(Clone)]
pub struct SurfaceC0Point {
    pub id: u64,
}

impl SurfaceC0 {
    pub fn new(id: u64, points: Vec<SurfaceC0Point>, size: (u32, u32), is_cylinder: bool) -> Self {
        Self {
            id,
            name: format!("SurfaceC0 {}", id),
            draw_polygon: false,
            tess_level: 4,
            points,
            size,
            is_cylinder,
        }
    }

    pub fn new_with_name(
        id: u64,
        name: String,
        points: Vec<SurfaceC0Point>,
        size: (u32, u32),
        is_cylinder: bool,
    ) -> Self {
        Self {
            id,
            name,
            draw_polygon: false,
            tess_level: 4,
            points,
            size,
            is_cylinder,
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

    pub fn replace_point(&mut self, old_point: u64, new_point: u64) {
        for i in 0..self.points.len() {
            if self.points[i].id == old_point {
                self.points[i] = SurfaceC0Point { id: new_point };
            }
        }
    }
}
