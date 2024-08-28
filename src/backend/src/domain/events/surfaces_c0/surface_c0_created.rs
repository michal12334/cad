pub struct SurfaceC0Created {
    pub id: u64,
    pub size: (u32, u32),
    pub is_cylinder: bool,
}

impl SurfaceC0Created {
    pub fn new(id: u64, size: (u32, u32), is_cylinder: bool) -> Self {
        SurfaceC0Created {
            id,
            size,
            is_cylinder,
        }
    }
}
