use crate::domain::torus::Torus;
use crate::domain::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn from_torus(torus: &Torus) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..torus.major_segments {
            for j in 0..torus.minor_segments {
                let u = i as f64 / torus.major_segments as f64 * 2.0 * std::f64::consts::PI;
                let v = j as f64 / torus.minor_segments as f64 * 2.0 * std::f64::consts::PI;

                let x = (torus.major_radius + torus.minor_radius * v.cos()) * u.cos();
                let y = (torus.major_radius + torus.minor_radius * v.cos()) * u.sin();
                let z = torus.minor_radius * v.sin();

                vertices.push(Vertex {
                    position: [x as f32, y as f32, z as f32],
                });

                indices.push(j + i * torus.minor_segments);
                indices.push(((j + 1) % torus.minor_segments) + i * torus.minor_segments);
                indices.push(j + i * torus.minor_segments);
                indices.push(j + ((i + 1) % torus.major_segments) * torus.minor_segments);
            }
        }

        Mesh { vertices, indices }
    }
}
