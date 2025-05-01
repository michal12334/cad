use derive_new::new;

use crate::domain::intersection::{IntersectionObjectId, TextureDraw};

#[derive(Debug, Clone, new)]
pub struct IntersectionTexturesDrawSet {
    pub id: u64,
    pub uv_draw: TextureDraw,
    pub st_draw: TextureDraw,
    pub id1: IntersectionObjectId,
    pub id2: IntersectionObjectId,
}
