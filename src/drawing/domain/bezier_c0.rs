use glium::{Display, IndexBuffer, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use backend::cqrs::points::point_details::PointDTO;
use backend::domain::vertex::Vertex;

pub struct BezierC0 {
    pub id: u64,
    pub points: Vec<Vertex>,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u16>,
}

impl BezierC0 {
    pub fn from_dto(id: u64, points: &[PointDTO], display: &Display<WindowSurface>) -> Self {
        let points = points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();
        
        let (vertex_buffer, index_buffer) = {
            let mut points = points.clone();
            while points.len() % 3 != 1 {
                points.push(Vertex {
                    position: [0.0, 0.0, 0.0],
                });
            }
            (
                VertexBuffer::new(display, &points).unwrap(),
                IndexBuffer::new(
                    display,
                    PrimitiveType::LinesListAdjacency,
                    &(0..(points.len() as u16 - 3))
                        .step_by(3)
                        .flat_map(|f| [f, f + 1, f + 2, f + 3])
                        .collect::<Vec<u16>>(),
                )
                .unwrap()
            )
        };
        
        Self {
            id,
            points,
            vertex_buffer,
            index_buffer,
        }
    }
}
