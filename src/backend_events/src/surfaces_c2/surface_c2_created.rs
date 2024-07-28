pub struct SurfaceC2Created {
    pub id: u64,
    pub size: (u32, u32),
}

impl SurfaceC2Created {
    pub fn new(id: u64, size: (u32, u32)) -> Self {
        SurfaceC2Created { id, size }
    }
}
