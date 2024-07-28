pub struct SurfaceC2PointsSelected {
    pub surface_id: u64,
}

impl SurfaceC2PointsSelected {
    pub fn new(surface_id: u64) -> Self {
        Self { surface_id }
    }
}
