use crate::domain::mesh::Mesh;
use crate::domain::transformer::Transformer;

pub struct Torus {
    pub id: u64,
    pub name: String,
    
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
    
    pub transformer: Transformer,
    pub mesh: Mesh,
}

impl Torus {
    pub fn new(id: u64, major_radius: f64, minor_radius: f64, major_segments: u32, minor_segments: u32) -> Self {
        let transformer = Transformer::new();
        let mesh = Mesh::from_torus(major_radius, minor_radius, major_segments, minor_segments);
        
        Self {
            id,
            name: format!("Torus {}", id),
            major_radius,
            minor_radius,
            major_segments,
            minor_segments,
            transformer,
            mesh,
        }
    }
    
    pub fn update(&mut self, major_radius: f64, minor_radius: f64, major_segments: u32, minor_segments: u32) {
        self.major_radius = major_radius;
        self.minor_radius = minor_radius;
        self.major_segments = major_segments;
        self.minor_segments = minor_segments;
        self.mesh = Mesh::from_torus(major_radius, minor_radius, major_segments, minor_segments);
    }
    
    pub fn transform(&mut self, position: (f64, f64, f64), rotation: (f64, f64, f64), scale: (f64, f64, f64)) {
        self.transformer.position = position;
        self.transformer.rotation = rotation;
        self.transformer.scale = scale;
    }
    
    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
