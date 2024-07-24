pub struct SurfaceC0PointsSelected {
    pub surface_id: u64,
}

impl SurfaceC0PointsSelected {
    pub fn new(surface_id: u64) -> Self {
        Self { surface_id }
    }
}
