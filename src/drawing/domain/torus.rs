use backend::domain::vertex::VertexUV;
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, Rect, Texture2d, VertexBuffer};
use math::matrix4::Matrix4;
use math::vector4::Vector4;

pub struct Torus {
    pub id: u64,
    pub vertex_buffer: VertexBuffer<VertexUV>,
    pub index_buffer: IndexBuffer<u32>,
    pub model_matrix: Matrix4,
    pub texture: Texture2d,
}

impl Torus {
    pub fn new(
        id: u64,
        major_radius: f64,
        minor_radius: f64,
        major_segments: u32,
        minor_segments: u32,
        position: (f64, f64, f64),
        rotation: (f64, f64, f64, f64),
        scale: (f64, f64, f64),
        display: &Display<WindowSurface>,
    ) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..major_segments {
            for j in 0..minor_segments {
                let u = i as f64 / major_segments as f64 * 2.0 * std::f64::consts::PI;
                let v = j as f64 / minor_segments as f64 * 2.0 * std::f64::consts::PI;

                let x = (major_radius + minor_radius * v.cos()) * u.cos();
                let z = (major_radius + minor_radius * v.cos()) * u.sin();
                let y = minor_radius * v.sin();

                vertices.push(VertexUV {
                    position: [x as f32, y as f32, z as f32],
                    uv: [
                        j as f32 / (minor_segments - 1) as f32,
                        i as f32 / (major_segments - 1) as f32,
                    ],
                });

                indices.push(j + i * minor_segments);
                indices.push(((j + 1) % minor_segments) + i * minor_segments);
                indices.push(j + i * minor_segments);
                indices.push(j + ((i + 1) % major_segments) * minor_segments);
            }
        }

        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

        let index_buffer = IndexBuffer::new(display, PrimitiveType::LineStrip, &indices).unwrap();

        let model_matrix =
            Matrix4::translation(position.0 as f32, position.1 as f32, position.2 as f32)
                * Matrix4::rotation_quaternion(Vector4::new(
                    rotation.0 as f32,
                    rotation.1 as f32,
                    rotation.2 as f32,
                    rotation.3 as f32,
                ))
                * Matrix4::scale(scale.0 as f32, scale.1 as f32, scale.2 as f32);

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
            vertex_buffer,
            index_buffer,
            model_matrix,
            texture,
        }
    }

    pub fn update(
        &mut self,
        major_radius: f64,
        minor_radius: f64,
        major_segments: u32,
        minor_segments: u32,
        display: &Display<WindowSurface>,
    ) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..major_segments {
            for j in 0..minor_segments {
                let u = i as f64 / major_segments as f64 * 2.0 * std::f64::consts::PI;
                let v = j as f64 / minor_segments as f64 * 2.0 * std::f64::consts::PI;

                let x = (major_radius + minor_radius * v.cos()) * u.cos();
                let z = (major_radius + minor_radius * v.cos()) * u.sin();
                let y = minor_radius * v.sin();

                vertices.push(VertexUV {
                    position: [x as f32, y as f32, z as f32],
                    uv: [
                        j as f32 / (minor_segments - 1) as f32,
                        i as f32 / (major_segments - 1) as f32,
                    ],
                });

                indices.push(j + i * minor_segments);
                indices.push(((j + 1) % minor_segments) + i * minor_segments);
                indices.push(j + i * minor_segments);
                indices.push(j + ((i + 1) % major_segments) * minor_segments);
            }
        }

        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

        let index_buffer = IndexBuffer::new(display, PrimitiveType::LineStrip, &indices).unwrap();

        self.vertex_buffer = vertex_buffer;
        self.index_buffer = index_buffer;
    }

    pub fn transform(
        &mut self,
        position: (f64, f64, f64),
        rotation: (f64, f64, f64, f64),
        scale: (f64, f64, f64),
    ) {
        self.model_matrix =
            Matrix4::translation(position.0 as f32, position.1 as f32, position.2 as f32)
                * Matrix4::rotation_quaternion(Vector4::new(
                    rotation.0 as f32,
                    rotation.1 as f32,
                    rotation.2 as f32,
                    rotation.3 as f32,
                ))
                * Matrix4::scale(scale.0 as f32, scale.1 as f32, scale.2 as f32);
    }

    pub fn update_texture(&mut self, texture: Texture2d) {
        self.texture = texture;
    }
}
