pub struct SurfaceC0Created {
    pub id: u64,
    pub size: (u32, u32),
}

impl SurfaceC0Created {
    pub fn new(id: u64, size: (u32, u32)) -> Self {
        SurfaceC0Created { id, size }
    }
}
