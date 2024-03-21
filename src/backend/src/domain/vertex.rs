use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(position: (f64, f64, f64)) -> Self {
        Vertex {
            position: [position.0 as f32, position.1 as f32, position.2 as f32],
        }
    }
}
