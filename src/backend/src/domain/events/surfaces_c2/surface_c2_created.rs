pub struct SurfaceC2Created {
    pub id: u64,
    pub size: (u32, u32),
    pub is_cylinder: bool,
}

impl SurfaceC2Created {
    pub fn new(id: u64, size: (u32, u32), is_cylinder: bool) -> Self {
        SurfaceC2Created {
            id,
            size,
            is_cylinder,
        }
    }
}
