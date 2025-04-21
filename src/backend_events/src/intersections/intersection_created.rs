use bit_vec::BitVec;
use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct IntersectionCreated {
    pub id: u64,
    pub name: String,
    pub uv_texture: Vec<BitVec>,
    pub st_texture: Vec<BitVec>,
}
