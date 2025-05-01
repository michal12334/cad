use crate::domain::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn from_cursor() -> Self {
        let vertices = vec![
            Vertex {
                position: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.0, 0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 3, 4, 5];

        Mesh { vertices, indices }
    }
}
