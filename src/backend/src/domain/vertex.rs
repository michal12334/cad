use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new() -> Self {
        Vertex {
            position: [0.0, 0.0, 0.0],
        }
    }
}

#[derive(Copy, Clone)]
pub struct VertexUV {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

implement_vertex!(VertexUV, position, uv);

impl VertexUV {
    pub fn new() -> Self {
        VertexUV {
            position: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0],
        }
    }
}
