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
