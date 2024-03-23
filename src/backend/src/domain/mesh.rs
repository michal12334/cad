use crate::domain::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn from_torus(major_radius: f64, minor_radius: f64, major_segments: u32, minor_segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..major_segments {
            for j in 0..minor_segments {
                let u = i as f64 / major_segments as f64 * 2.0 * std::f64::consts::PI;
                let v = j as f64 / minor_segments as f64 * 2.0 * std::f64::consts::PI;

                let x = (major_radius + minor_radius * v.cos()) * u.cos();
                let y = (major_radius + minor_radius * v.cos()) * u.sin();
                let z = minor_radius * v.sin();

                vertices.push(Vertex {
                    position: [x as f32, y as f32, z as f32],
                });

                indices.push(j + i * minor_segments);
                indices.push(((j + 1) % minor_segments) + i * minor_segments);
                indices.push(j + i * minor_segments);
                indices.push(j + ((i + 1) % major_segments) * minor_segments);
            }
        }

        Mesh { vertices, indices }
    }
    
    pub fn from_cursor() -> Self {
        let vertices = vec![
            Vertex { position: [-1.0, 0.0, 0.0] },
            Vertex { position: [1.0, 0.0, 0.0] },
            Vertex { position: [0.0, -1.0, 0.0] },
            Vertex { position: [0.0, 1.0, 0.0] },
            Vertex { position: [0.0, 0.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ];
        
        let indices = vec![
            0, 1,
            2, 3,
            4, 5,
        ];
        
        Mesh { vertices, indices }
    }
}
