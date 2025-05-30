use bit_vec::BitVec;
use derive_new::new;
use math::vector3::Vector3;

#[derive(Debug, Clone, new)]
pub struct IntersectionCreated {
    pub id: u64,
    pub name: String,
    pub uv_texture: Vec<BitVec>,
    pub st_texture: Vec<BitVec>,
    pub points: Vec<Vector3>,
    pub wrap: bool,
}
