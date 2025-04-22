use math::vector3::Vector3;

use super::{
    intersection::IntersectionObjectId, intersection_object::IntersectionObject, point::Point,
};

pub struct SurfaceC2 {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub tess_level: u8,
    pub points: Vec<SurfaceC2Point>,
    pub size: (u32, u32),
    pub is_cylinder: bool,
}

#[derive(Clone)]
pub struct SurfaceC2Point {
    pub id: u64,
}

impl SurfaceC2 {
    pub fn new(id: u64, points: Vec<SurfaceC2Point>, size: (u32, u32), is_cylinder: bool) -> Self {
        Self {
            id,
            name: format!("SurfaceC2 {}", id),
            draw_polygon: false,
            tess_level: 4,
            points,
            size,
            is_cylinder,
        }
    }

    pub fn new_with_name(
        id: u64,
        name: String,
        points: Vec<SurfaceC2Point>,
        size: (u32, u32),
        is_cylinder: bool,
    ) -> Self {
        Self {
            id,
            name,
            draw_polygon: false,
            tess_level: 4,
            points,
            size,
            is_cylinder,
        }
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_draw_polygon(&mut self, draw_polygon: bool) {
        self.draw_polygon = draw_polygon;
    }

    pub fn set_tess_level(&mut self, tess_level: u8) {
        self.tess_level = tess_level;
    }

    pub fn replace_point(&mut self, old_point: u64, new_point: u64) {
        for i in 0..self.points.len() {
            if self.points[i].id == old_point {
                self.points[i] = SurfaceC2Point { id: new_point };
            }
        }
    }

    pub fn get_intersection_object(&self, points: &[Point]) -> IntersectionObject {
        let points = self
            .points
            .iter()
            .map(|p| {
                let point = points.iter().find(|&point| point.id == p.id).unwrap();
                Vector3::new(
                    point.transformer.position.0 as f32,
                    point.transformer.position.1 as f32,
                    point.transformer.position.2 as f32,
                )
            })
            .collect::<Vec<_>>();

        let size = self.size;

        IntersectionObject::new(
            IntersectionObjectId::SurfaceC2(self.id),
            (self.size.0 as f32, self.size.1 as f32),
            move |u, v| {
                let ui = u as usize;
                let vi = v as usize;

                let u = u - u.floor();
                let v = v - v.floor();

                let patch = [
                    (ui, vi),
                    (ui + 1, vi),
                    (ui + 2, vi),
                    (ui + 3, vi),
                    (ui, vi + 1),
                    (ui + 1, vi + 1),
                    (ui + 2, vi + 1),
                    (ui + 3, vi + 1),
                    (ui, vi + 2),
                    (ui + 1, vi + 2),
                    (ui + 2, vi + 2),
                    (ui + 3, vi + 2),
                    (ui, vi + 3),
                    (ui + 1, vi + 3),
                    (ui + 2, vi + 3),
                    (ui + 3, vi + 3),
                ]
                .iter()
                .map(|&(x, y)| points[x * (size.1 as usize + 3) + y])
                .collect::<Vec<_>>();

                let u2 = u * u;
                let u3 = u2 * u;
                let b0u = (-u3 + 3.0 * u2 - 3.0 * u + 1.0) / 6.0;
                let b1u = (3.0 * u3 - 6.0 * u2 + 4.0) / 6.0;
                let b2u = (-3.0 * u3 + 3.0 * u2 + 3.0 * u + 1.0) / 6.0;
                let b3u = u3 / 6.0;
                let v2 = v * v;
                let v3 = v2 * v;
                let b0v = (-v3 + 3.0 * v2 - 3.0 * v + 1.0) / 6.0;
                let b1v = (3.0 * v3 - 6.0 * v2 + 4.0) / 6.0;
                let b2v = (-3.0 * v3 + 3.0 * v2 + 3.0 * v + 1.0) / 6.0;
                let b3v = v3 / 6.0;

                b0v * (b0u * patch[0] + b1u * patch[1] + b2u * patch[2] + b3u * patch[3])
                    + b1v * (b0u * patch[4] + b1u * patch[5] + b2u * patch[6] + b3u * patch[7])
                    + b2v * (b0u * patch[8] + b1u * patch[9] + b2u * patch[10] + b3u * patch[11])
                    + b3v * (b0u * patch[12] + b1u * patch[13] + b2u * patch[14] + b3u * patch[15])
            },
            false,
            false,
        )
    }
}
