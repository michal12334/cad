use std::f32::consts::PI;

use math::vector3::Vector3;
use math::vector4::Vector4;

use crate::domain::transformer::Transformer;

use super::intersection::IntersectionObjectId;
use super::intersection_object::IntersectionObject;

pub struct Torus {
    pub id: u64,
    pub name: String,

    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,

    pub transformer: Transformer,
}

impl Torus {
    pub fn new(
        id: u64,
        major_radius: f64,
        minor_radius: f64,
        major_segments: u32,
        minor_segments: u32,
        transformer: Transformer,
    ) -> Self {
        Self {
            id,
            name: format!("Torus {}", id),
            major_radius,
            minor_radius,
            major_segments,
            minor_segments,
            transformer,
        }
    }

    pub fn new_with_name(
        id: u64,
        name: String,
        major_radius: f64,
        minor_radius: f64,
        major_segments: u32,
        minor_segments: u32,
        transformer: Transformer,
    ) -> Self {
        Self {
            id,
            name,
            major_radius,
            minor_radius,
            major_segments,
            minor_segments,
            transformer,
        }
    }

    pub fn update(
        &mut self,
        major_radius: f64,
        minor_radius: f64,
        major_segments: u32,
        minor_segments: u32,
    ) {
        self.major_radius = major_radius;
        self.minor_radius = minor_radius;
        self.major_segments = major_segments;
        self.minor_segments = minor_segments;
    }

    pub fn transform(
        &mut self,
        position: (f64, f64, f64),
        rotation: (f64, f64, f64, f64),
        scale: (f64, f64, f64),
    ) {
        self.transformer.position = position;
        self.transformer.rotation = rotation;
        self.transformer.scale = scale;
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_intersection_object(&self) -> IntersectionObject {
        let major_radius = self.major_radius as f32;
        let minor_radius = self.minor_radius as f32;
        let model_matrix = self.transformer.get_model_matrix();

        IntersectionObject::new(
            IntersectionObjectId::Torus(self.id),
            (2.0 * PI, 2.0 * PI),
            move |u, v| {
                let x = (major_radius + minor_radius * v.cos()) * u.cos();
                let z = (major_radius + minor_radius * v.cos()) * u.sin();
                let y = minor_radius * v.sin();

                let p = Vector3::new(x, y, z);
                let p = Vector4::from_vector3(p, 1.0);
                let p = model_matrix * p;
                p.to_vector3()
            },
            true,
            true,
        )
    }
}
