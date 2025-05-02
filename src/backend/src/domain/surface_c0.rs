use math::vector3::Vector3;

use super::{
    intersection::IntersectionObjectId, intersection_object::IntersectionObject, point::Point,
};

pub struct SurfaceC0 {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub tess_level: u8,
    pub points: Vec<SurfaceC0Point>,
    pub size: (u32, u32),
    pub is_cylinder: bool,
}

#[derive(Clone)]
pub struct SurfaceC0Point {
    pub id: u64,
}

impl SurfaceC0 {
    pub fn new(id: u64, points: Vec<SurfaceC0Point>, size: (u32, u32), is_cylinder: bool) -> Self {
        Self {
            id,
            name: format!("SurfaceC0 {}", id),
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
        points: Vec<SurfaceC0Point>,
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
                self.points[i] = SurfaceC0Point { id: new_point };
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

        let wrap_u = (0..self.size.1 as usize)
            .flat_map(|x| [3 * x, 3 * x + 1, 3 * x + 2, 3 * x + 3])
            .all(|v| {
                self.points[v].id
                    == self.points[3 * self.size.0 as usize * (3 * self.size.1 as usize + 1) + v].id
            });
        let wrap_v = (0..self.size.0 as usize)
            .flat_map(|x| [3 * x, 3 * x + 1, 3 * x + 2, 3 * x + 3])
            .all(|u| {
                self.points[(3 * u + 3) * (3 * self.size.1 as usize + 1)].id
                    == self.points
                        [(3 * u + 3) * (3 * self.size.1 as usize + 1) + 3 * self.size.1 as usize]
                        .id
            });

        IntersectionObject::new(
            IntersectionObjectId::SurfaceC0(self.id),
            (self.size.0 as f32, self.size.1 as f32),
            move |u, v| {
                let mut ui = u as usize;
                let mut vi = v as usize;

                if ui == size.0 as usize {
                    ui = ui - 1;
                }
                if vi == size.1 as usize {
                    vi = vi - 1;
                }

                let u = u - ui as f32;
                let v = v - vi as f32;

                let patch = [
                    (3 * ui, 3 * vi),
                    (3 * ui + 1, 3 * vi),
                    (3 * ui + 2, 3 * vi),
                    (3 * ui + 3, 3 * vi),
                    (3 * ui, 3 * vi + 1),
                    (3 * ui + 1, 3 * vi + 1),
                    (3 * ui + 2, 3 * vi + 1),
                    (3 * ui + 3, 3 * vi + 1),
                    (3 * ui, 3 * vi + 2),
                    (3 * ui + 1, 3 * vi + 2),
                    (3 * ui + 2, 3 * vi + 2),
                    (3 * ui + 3, 3 * vi + 2),
                    (3 * ui, 3 * vi + 3),
                    (3 * ui + 1, 3 * vi + 3),
                    (3 * ui + 2, 3 * vi + 3),
                    (3 * ui + 3, 3 * vi + 3),
                ]
                .iter()
                .map(|&(x, y)| points[x * (size.1 as usize * 3 + 1) + y])
                .collect::<Vec<_>>();

                let iu = 1.0 - u;
                let b0u = iu * iu * iu;
                let b1u = 3.0 * iu * iu * u;
                let b2u = 3.0 * iu * u * u;
                let b3u = u * u * u;
                let iv = 1.0 - v;
                let b0v = iv * iv * iv;
                let b1v = 3.0 * iv * iv * v;
                let b2v = 3.0 * iv * v * v;
                let b3v = v * v * v;

                b0v * (b0u * patch[0] + b1u * patch[1] + b2u * patch[2] + b3u * patch[3])
                    + b1v * (b0u * patch[4] + b1u * patch[5] + b2u * patch[6] + b3u * patch[7])
                    + b2v * (b0u * patch[8] + b1u * patch[9] + b2u * patch[10] + b3u * patch[11])
                    + b3v * (b0u * patch[12] + b1u * patch[13] + b2u * patch[14] + b3u * patch[15])
            },
            wrap_u,
            wrap_v,
        )
    }
}
