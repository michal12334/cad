use nalgebra::{Matrix4, Vector3};

pub struct Transformer {
    pub position: (f64, f64, f64),
    pub rotation: (f64, f64, f64),
    pub scale: (f64, f64, f64),
}

impl Transformer {
    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        let translation = Matrix4::new_translation(&Vector3::new(self.position.0 as f32, self.position.1 as f32, self.position.2 as f32));
        let rotation = Matrix4::new_rotation(Vector3::new(self.rotation.0 as f32, self.rotation.1 as f32, self.rotation.2 as f32));
        let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(self.scale.0 as f32, self.scale.1 as f32, self.scale.2 as f32));
        let result = translation * rotation * scale;
        return  result.as_ref().clone();
    }
}
