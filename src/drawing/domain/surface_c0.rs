use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, Rect, Texture2d, VertexBuffer};

use backend::cqrs::points::point_details::PointDTO;
use backend::domain::vertex::VertexUV;

pub struct SurfaceC0 {
    pub id: u64,
    pub draw_polygon: bool,
    pub tess_level: u8,
    pub is_cylinder: bool,
    pub vertex_buffer: VertexBuffer<VertexUV>,
    pub surface_index_buffer: IndexBuffer<u32>,
    pub polygon_index_buffer: IndexBuffer<u32>,
    pub uvs: Vec<(f32, f32)>,
    pub texture: Texture2d,
}

impl SurfaceC0 {
    pub fn new(
        id: u64,
        points: &[PointDTO],
        size: (u32, u32),
        display: &Display<WindowSurface>,
        is_cylinder: bool,
    ) -> Self {
        let uvs = (0..points.len())
            .map(|i| {
                let s0 = size.0 as usize * 3 + 1;
                let s1 = size.1 as usize * 3 + 1;
                let v = i / s1;
                let u = i % s1;
                let v = v as f32 / (s0 - 1) as f32;
                let u = u as f32 / (s1 - 1) as f32;
                (u, v)
            })
            .collect::<Vec<_>>();

        let vertex_buffer = VertexBuffer::new(
            display,
            &points
                .iter()
                .zip(uvs.iter())
                .map(|(p, (u, v))| VertexUV {
                    position: [
                        p.transformer.position.0 as f32,
                        p.transformer.position.1 as f32,
                        p.transformer.position.2 as f32,
                    ],
                    uv: [*u, *v],
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let surface_index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::Patches {
                vertices_per_patch: 16,
            },
            &(0..size.0)
                .flat_map(|x| (0..size.1).map(move |y| (x, y)))
                .flat_map(|(x, y)| {
                    [
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
                    ]
                })
                .map(|(x, y)| x * (size.1 * 3 + 1) + y)
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let polygon_index_buffer = IndexBuffer::new(
            display,
            PrimitiveType::LinesList,
            &(0..(size.0 * 3 + 1))
                .flat_map(|x| (0..(size.1 * 3 + 1)).map(move |y| (x, y)))
                .flat_map(|(x, y)| {
                    if x < size.0 * 3 && y < size.1 * 3 {
                        vec![(x, y), (x + 1, y), (x, y), (x, y + 1)]
                    } else if x < size.0 * 3 {
                        vec![(x, y), (x + 1, y)]
                    } else if y < size.1 * 3 {
                        vec![(x, y), (x, y + 1)]
                    } else {
                        vec![]
                    }
                })
                .map(|(x, y)| x * (size.1 * 3 + 1) + y)
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let texture = Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            1,
            1,
        )
        .unwrap();

        let data = vec![vec![1f32; 1]; 1];

        texture.write(
            Rect {
                left: 0,
                bottom: 0,
                width: 1,
                height: 1,
            },
            data.clone(),
        );

        Self {
            id,
            draw_polygon: false,
            tess_level: 4,
            is_cylinder,
            vertex_buffer,
            surface_index_buffer,
            polygon_index_buffer,
            uvs,
            texture,
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
            &points
                .iter()
                .zip(self.uvs.iter())
                .map(|(p, (u, v))| VertexUV {
                    position: [
                        p.transformer.position.0 as f32,
                        p.transformer.position.1 as f32,
                        p.transformer.position.2 as f32,
                    ],
                    uv: [*u, *v],
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }

    pub fn update_texture(&mut self, texture: Texture2d) {
        self.texture = texture;
    }
}
