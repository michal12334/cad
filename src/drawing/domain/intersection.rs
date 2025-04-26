use backend::domain::vertex::Vertex;
use backend::extensions::iterator_extensions::IteratorExtensions;
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, VertexBuffer};
use math::vector3::Vector3;

pub struct Intersection {
    pub id: u64,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u16>,
}

impl Intersection {
    pub fn new(id: u64, points: &[Vector3], wrap: bool, display: &Display<WindowSurface>) -> Self {
        let vertex_buffer = VertexBuffer::new(
            display,
            &points
                .iter()
                .map(|p| Vertex {
                    position: [p.x, p.y, p.z],
                })
                .collect::<Vec<Vertex>>(),
        )
        .unwrap();

        let index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::LineStrip,
            &(0..(points.len()))
                .chain_if([0usize].into_iter(), wrap)
                .map(|x| x as u16)
                .collect::<Vec<_>>(),
        )
        .unwrap();

        Self {
            id,
            vertex_buffer,
            index_buffer,
        }
    }
}
