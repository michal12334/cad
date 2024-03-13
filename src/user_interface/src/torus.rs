use backend::cqrs::torus_details::TorusDTO;
use crate::typed_text_buffer::TypedTextBuffer;

pub struct Torus {
    pub major_radius: TypedTextBuffer<f64>,
    pub minor_radius: TypedTextBuffer<f64>,
    pub major_segments: TypedTextBuffer<u32>,
    pub minor_segments: TypedTextBuffer<u32>,
    pub position: (TypedTextBuffer<f64>, TypedTextBuffer<f64>, TypedTextBuffer<f64>),
    pub rotation: (TypedTextBuffer<f64>, TypedTextBuffer<f64>, TypedTextBuffer<f64>),
    pub scale: (TypedTextBuffer<f64>, TypedTextBuffer<f64>, TypedTextBuffer<f64>),
}

impl Torus {
    pub fn from_dto(dto: &TorusDTO) -> Self {
        Self {
            major_radius: TypedTextBuffer::new(dto.major_radius),
            minor_radius: TypedTextBuffer::new(dto.minor_radius),
            major_segments: TypedTextBuffer::new(dto.major_segments),
            minor_segments: TypedTextBuffer::new(dto.minor_segments),
            position: (
                TypedTextBuffer::new(dto.transformer.position.0),
                TypedTextBuffer::new(dto.transformer.position.1),
                TypedTextBuffer::new(dto.transformer.position.2),
            ),
            rotation: (
                TypedTextBuffer::new(dto.transformer.rotation.0),
                TypedTextBuffer::new(dto.transformer.rotation.1),
                TypedTextBuffer::new(dto.transformer.rotation.2),
            ),
            scale: (
                TypedTextBuffer::new(dto.transformer.scale.0),
                TypedTextBuffer::new(dto.transformer.scale.1),
                TypedTextBuffer::new(dto.transformer.scale.2),
            ),
        }
    }
}
