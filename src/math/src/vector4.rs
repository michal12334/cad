use crate::vector3::Vector3;

pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    
    pub fn from_vector3(v: Vector3, w: f32) -> Self {
        Self { x: v.x, y: v.y, z: v.z, w }
    }
    
    pub fn xyz(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
    
    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.x / self.w, self.y / self.w, self.z / self.w)
    }
}
