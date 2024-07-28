use glium::{Display, IndexBuffer, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use backend::cqrs::points::point_details::PointDTO;
use backend::domain::vertex::Vertex;

pub struct SurfaceC2 {
    pub id: u64,
    pub draw_polygon: bool,
    pub tess_level: u8,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub surface_index_buffer: IndexBuffer<u32>,
    pub polygon_index_buffer: IndexBuffer<u32>,
}

impl SurfaceC2 {
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
                .flat_map(|(x, y)| [
                    (3 * x, 3 * y),
                    (3 * x + 1, 3 * y),
                    (3 * x + 2, 3 * y),
                    (3 * x + 3, 3 * y),
                    (3 * x, 3 * y + 1),
                    (3 * x + 1, 3 * y + 1),
                    (3 * x + 2, 3 * y + 1),
                    (3 * x + 3, 3 * y + 1),
                    (3 * x, 3 * y + 2),
                    (3 * x + 1, 3 * y + 2),
                    (3 * x + 2, 3 * y + 2),
                    (3 * x + 3, 3 * y + 2),
                    (3 * x, 3 * y + 3),
                    (3 * x + 1, 3 * y + 3),
                    (3 * x + 2, 3 * y + 3),
                    (3 * x + 3, 3 * y + 3),
                    (3 * x, 3 * y),
                    (3 * x, 3 * y + 1),
                    (3 * x, 3 * y + 2),
                    (3 * x, 3 * y + 3),
                    (3 * x + 1, 3 * y),
                    (3 * x + 1, 3 * y + 1),
                    (3 * x + 1, 3 * y + 2),
                    (3 * x + 1, 3 * y + 3),
                    (3 * x + 2, 3 * y),
                    (3 * x + 2, 3 * y + 1),
                    (3 * x + 2, 3 * y + 2),
                    (3 * x + 2, 3 * y + 3),
                    (3 * x + 3, 3 * y),
                    (3 * x + 3, 3 * y + 1),
                    (3 * x + 3, 3 * y + 2),
                    (3 * x + 3, 3 * y + 3),
                ])
                .map(|(x, y)| x * (size.1 * 3 + 1) + y)
                .collect::<Vec<_>>())
            .unwrap();

        let polygon_index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::LinesList,
            &(0..(size.0 * 3 + 1))
                .flat_map(|x| (0..(size.1 * 3 + 1)).map(move |y| (x, y)))
                .flat_map(|(x, y)| 
                    if x < size.0 * 3 && y < size.1 * 3 {
                        vec![
                            (x, y),
                            (x + 1, y),
                            (x, y),
                            (x, y + 1),
                        ]
                    } else if x < size.0 * 3 {
                        vec![
                            (x, y),
                            (x + 1, y),
                        ]
                    } else if y < size.1 * 3 {
                        vec![
                            (x, y),
                            (x, y + 1),
                        ]
                    } else {
                        vec![]
                    })
                .map(|(x, y)| x * (size.1 * 3 + 1) + y)
                .collect::<Vec<_>>())
            .unwrap();

        Self {
            id,
            draw_polygon: false,
            tess_level: 8,
            vertex_buffer,
            surface_index_buffer,
            polygon_index_buffer,
        }
    }

    pub fn set_draw_polygon(&mut self, draw_polygon: bool) {
        self.draw_polygon = draw_polygon;
    }

    pub fn set_tess_level(&mut self, tess_level: u8) {
        self.tess_level = tess_level;
    }

    pub fn update_points(&mut self, points: &[PointDTO], display: &Display<WindowSurface>) {
        self.vertex_buffer = VertexBuffer::new(
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
    }
}
