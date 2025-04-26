use derive_new::new;

use crate::domain::intersection::TextureDraw;

#[derive(Debug, Clone, new)]
pub struct IntersectionTexturesDrawSet {
    pub id: u64,
    pub uv_draw: TextureDraw,
    pub st_draw: TextureDraw,
}
