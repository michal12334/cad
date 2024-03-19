use math::matrix4::Matrix4;
use math::vector3::Vector3;

pub struct Transformer {
    pub position: (f64, f64, f64),
    pub rotation: (f64, f64, f64),
    pub scale: (f64, f64, f64),
}

impl Transformer {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
        }
    }
    
    pub fn get_model_matrix(&self) -> Matrix4 {
        let translation = Matrix4::translation(self.position.0 as f32, self.position.1 as f32, self.position.2 as f32);
        let rotation = Matrix4::rotation(self.rotation.0 as f32, self.rotation.1 as f32, self.rotation.2 as f32);
        let scale = Matrix4::scale(self.scale.0 as f32, self.scale.1 as f32, self.scale.2 as f32);
        translation * rotation * scale
    }
}
