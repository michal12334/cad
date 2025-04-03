use backend::domain::vertex::Vertex;
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, VertexBuffer};

use math::vector3::Vector3;

pub struct Gregory {
    pub id: u64,
    pub tess_level: u8,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u16>,
}

impl Gregory {
    pub fn new(
        id: u64,
        tess_level: u8,
        points: &[Vector3],
        display: &Display<WindowSurface>,
    ) -> Self {
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
            PrimitiveType::Patches {
                vertices_per_patch: 20,
            },
            &(0..(points.len())).map(|x| x as u16).collect::<Vec<_>>(),
        )
        .unwrap();

        Self {
            id,
            tess_level,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn update_mesh(&mut self, points: &[Vector3], display: &Display<WindowSurface>) {
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
            PrimitiveType::Patches {
                vertices_per_patch: 20,
            },
            &(0..(points.len())).map(|x| x as u16).collect::<Vec<_>>(),
        )
        .unwrap();

        self.vertex_buffer = vertex_buffer;
        self.index_buffer = index_buffer;
    }

    pub fn update_settings(&mut self, tess_level: u8) {
        self.tess_level = tess_level;
    }
}
