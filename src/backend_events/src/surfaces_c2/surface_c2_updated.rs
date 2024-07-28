pub struct SurfaceC2Updated {
    pub id: u64,
    pub draw_polygon: bool,
    pub tess_level: u8,
}

impl SurfaceC2Updated {
    pub fn new(id: u64, draw_polygon: bool, tess_level: u8) -> Self {
        Self {
            id,
            draw_polygon,
            tess_level,
        }
    }
}
