use derive_new::new;
use math::vector3::Vector3;

pub struct Gregory {
    pub id: u64,
    pub name: String,
    pub patches: Vec<GregoryPatch>,
}

#[derive(Debug, Clone, new)]
pub struct GregoryPatch {
    pub top: [Vector3; 4],
    pub top_sides: [Vector3; 2],
    pub bottom_sides: [Vector3; 2],
    pub bottom: [Vector3; 4],
    pub u_inner: [Vector3; 4],
    pub v_inner: [Vector3; 4],
}

impl Gregory {
    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
