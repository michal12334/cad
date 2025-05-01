use bitflags::bitflags;
use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct IntersectionTexturesDrawSet {
    pub id: u64,
    pub uv_draw: TextureDraw,
    pub st_draw: TextureDraw,
    pub id1: IntersectionObjectIdDTO,
    pub id2: IntersectionObjectIdDTO,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureDraw: u32 {
        const True = 0b00000001;
        const False = 0b00000010;

        const Both = Self::True.bits() | Self::False.bits();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IntersectionObjectIdDTO {
    Torus(u64),
    SurfaceC0(u64),
    SurfaceC2(u64),
}
