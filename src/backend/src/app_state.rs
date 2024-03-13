use crate::domain::mesh::Mesh;
use crate::domain::torus::Torus;
use crate::domain::transformer::Transformer;

pub struct AppState {
    pub torus: Torus,
    pub transformer: Transformer,
    pub mesh: Mesh,
}

impl AppState {
    pub fn new() -> Self {
        let torus = Torus {
            major_radius: 0.5,
            minor_radius: 0.25,
            major_segments: 32,
            minor_segments: 16,
        };
        let transformer = Transformer {
            position: (0.0, 0.0, 2.0),
            rotation: (1.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
        };
        let mesh = Mesh::from_torus(&torus);
        AppState {
            torus,
            transformer,
            mesh,
        }
    }
}
