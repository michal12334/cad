use bitflags::bitflags;
use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct IntersectionTexturesDrawSet {
    pub id: u64,
    pub uv_draw: TextureDraw,
    pub st_draw: TextureDraw,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureDraw: u32 {
        const True = 0b00000001;
        const False = 0b00000010;

        const Both = Self::True.bits() | Self::False.bits();
    }
}
