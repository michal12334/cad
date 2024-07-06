use glium::glutin::surface::WindowSurface;
use glium::{Display, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use backend::cqrs::beziers_int::bezier_int_bernstein_points::BezierIntBernsteinPointDTO;
use backend::domain::vertex::Vertex;

pub struct BezierInt {
    pub id: u64,
    pub bernstein_points: Vec<Vertex>,
    pub vertex_buffer: Option<VertexBuffer<Vertex>>,
    pub index_buffer: Option<IndexBuffer<u16>>,
}

impl BezierInt {
    pub fn new(id: u64, bernstein_points: &[BezierIntBernsteinPointDTO], display: &Display<WindowSurface>) -> Self {
        let bernstein_points = bernstein_points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();
        
        let (vertex_buffer, index_buffer) = Self::get_buffers(&bernstein_points, display);
        
        Self {
            id,
            bernstein_points,
            vertex_buffer,
            index_buffer,
        }
    }
    
    fn get_buffers(bernstein_points: &[Vertex], display: &Display<WindowSurface>) -> (Option<VertexBuffer<Vertex>>, Option<IndexBuffer<u16>>) {
        if bernstein_points.len() < 4 {
            return (None, None);
        } else { 
            let vertex_buffer = VertexBuffer::new(display, &bernstein_points).ok();
            let index_buffer = IndexBuffer::new(
                display,
                PrimitiveType::LinesListAdjacency,
                &(0..(bernstein_points.len() as u16 - 3))
                    .step_by(3)
                    .flat_map(|f| [f, f + 1, f + 2, f + 3])
                    .collect::<Vec<u16>>(),
            ).ok();
            (vertex_buffer, index_buffer)
        }
    }
}
