use glium::{Display, IndexBuffer, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use backend::cqrs::points::point_details::PointDTO;
use backend::domain::vertex::Vertex;

pub struct SurfaceC0 {
    pub id: u64,
    pub draw_polygon: bool,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub surface_index_buffer: IndexBuffer<u32>,
    pub polygon_index_buffer: IndexBuffer<u32>,
}

impl SurfaceC0 {
    pub fn new(id: u64, points: &[PointDTO], size: (u32, u32), display: &Display<WindowSurface>) -> Self {
        let vertex_buffer = VertexBuffer::new(
            display,
            &points.iter()
                .map(|p| Vertex {
                    position: [
                        p.transformer.position.0 as f32,
                        p.transformer.position.1 as f32,
                        p.transformer.position.2 as f32,
                    ],
                })
                .collect::<Vec<Vertex>>())
            .unwrap();

        let surface_index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::Patches { vertices_per_patch: 16 },
            &(0..size.0)
                .flat_map(|x| (0..size.1).map(move |y| (x, y)))
                .flat_map(|(x, y)| vec![
                    (4 * x, 4 * y),
                    (4 * x + 1, 4 * y),
                    (4 * x + 2, 4 * y),
                    (4 * x + 3, 4 * y),
                    (4 * x, 4 * y + 1),
                    (4 * x + 1, 4 * y + 1),
                    (4 * x + 2, 4 * y + 1),
                    (4 * x + 3, 4 * y + 1),
                    (4 * x, 4 * y + 2),
                    (4 * x + 1, 4 * y + 2),
                    (4 * x + 2, 4 * y + 2),
                    (4 * x + 3, 4 * y + 2),
                    (4 * x, 4 * y + 3),
                    (4 * x + 1, 4 * y + 3),
                    (4 * x + 2, 4 * y + 3),
                    (4 * x + 3, 4 * y + 3),
                ])
                .map(|(x, y)| x + y * size.0)
                .collect::<Vec<_>>())
            .unwrap();

        let polygon_index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::LinesList,
            &(0..size.0 - 1)
                .flat_map(|x| (0..size.1 - 1).map(move |y| (x, y)))
                .flat_map(|(x, y)| vec![
                    (x, y),
                    (x + 1, y),
                    (x, y),
                    (x, y + 1),
                ])
                .map(|(x, y)| x + y * size.0)
                .collect::<Vec<_>>())
            .unwrap();

        Self {
            id,
            draw_polygon: false,
            vertex_buffer,
            surface_index_buffer,
            polygon_index_buffer,
        }
    }

    pub fn set_draw_polygon(&mut self, draw_polygon: bool) {
        self.draw_polygon = draw_polygon;
    }
}
