use std::{collections::VecDeque, fmt::Display};

use bit_vec::BitVec;
use bitflags::bitflags;
use itertools::Itertools;
use line_drawing::Bresenham;
use math::vector3::Vector3;
use nalgebra::{Matrix4, Vector2, Vector4};

use crate::extensions::iterator_extensions::IteratorExtensions;

use super::intersection_object::IntersectionObject;

pub struct Intersection {
    pub id: u64,
    pub name: String,
    pub object1_id: IntersectionObjectId,
    pub object2_id: IntersectionObjectId,
    pub intersection_points: Vec<Vector3>,
    pub uv_points: Vec<(f32, f32)>,
    pub st_points: Vec<(f32, f32)>,
    pub texture_size: usize,
    pub uv_texture: Vec<BitVec>,
    pub st_texture: Vec<BitVec>,
    pub wrap: bool,
    pub uv_draw: TextureDraw,
    pub st_draw: TextureDraw,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntersectionObjectId {
    Torus(u64),
    SurfaceC0(u64),
    SurfaceC2(u64),
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureDraw: u32 {
        const True = 0b00000001;
        const False = 0b00000010;

        const Both = Self::True.bits() | Self::False.bits();
    }
}

impl Display for IntersectionObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntersectionObjectId::Torus(id) => write!(f, "T{}", id),
            IntersectionObjectId::SurfaceC0(id) => write!(f, "SC0 {}", id),
            IntersectionObjectId::SurfaceC2(id) => write!(f, "SC2 {}", id),
        }
    }
}

impl Intersection {
    pub fn from_objects(
        id: u64,
        name: String,
        object1_id: IntersectionObjectId,
        object2_id: IntersectionObjectId,
        object1: &IntersectionObject,
        object2: &IntersectionObject,
        cursor_position: &Vector3,
        texture_size: usize,
        newton_factor: f32,
        rough: bool,
        max_distance: f32,
    ) -> Option<Self> {
        let uvst = Self::find_starting_points(object1, object2, cursor_position)?;

        println!("uvst: {:?}", uvst);

        let intersection = Self::find_intersection(
            object1,
            object2,
            uvst.0,
            uvst.1,
            newton_factor,
            rough,
            max_distance,
            object1_id == object2_id,
        )?;

        let uv_points = intersection
            .0
            .iter()
            .map(|uv| (uv.x, uv.y))
            .collect::<Vec<_>>();
        let st_points = intersection
            .1
            .iter()
            .map(|st| (st.x, st.y))
            .collect::<Vec<_>>();
        let uv_texture = Self::get_texture(
            texture_size,
            &uv_points,
            object1.value_range,
            object1.wrap_u,
            object1.wrap_v,
            intersection.2,
        );
        let st_texture = Self::get_texture(
            texture_size,
            &st_points,
            object2.value_range,
            object2.wrap_u,
            object2.wrap_v,
            intersection.2,
        );

        Some(Self {
            id,
            name,
            object1_id,
            object2_id,
            intersection_points: intersection
                .0
                .iter()
                .map(|uv| object1.get_value(uv.x, uv.y))
                .collect(),
            uv_points,
            st_points,
            texture_size,
            uv_texture,
            st_texture,
            wrap: intersection.2,
            uv_draw: TextureDraw::Both,
            st_draw: TextureDraw::Both,
        })
    }

    pub fn set_uv_draw(&mut self, draw: TextureDraw) {
        self.uv_draw = draw;
    }

    pub fn set_st_draw(&mut self, draw: TextureDraw) {
        self.st_draw = draw;
    }

    fn find_starting_points(
        object1: &IntersectionObject,
        object2: &IntersectionObject,
        cursor_position: &Vector3,
    ) -> Option<(Vector2<f32>, Vector2<f32>)> {
        let self_intersection = object1.id == object2.id;

        (0..100)
            .cartesian_product(0..100)
            .cartesian_product(0..100)
            .cartesian_product(0..100)
            .map(|(((u, v), s), t)| {
                let u = (u as f32) * 0.01 * object1.value_range.0;
                let v = (v as f32) * 0.01 * object1.value_range.1;
                let s = (s as f32) * 0.01 * object2.value_range.0;
                let t = (t as f32) * 0.01 * object2.value_range.1;

                let p = object1.get_value(u, v);
                let q = object2.get_value(s, t);

                ((p - q).to_nalgebra().norm(), p, q, u, v, s, t)
            })
            .filter(|(dist, _, _, _, _, _, _)| *dist < 0.1)
            .filter(|(_, _, _, u, v, s, t)| {
                if !self_intersection {
                    return true;
                }

                (u - s).abs() > 0.1
                    && (v - t).abs() > 0.1
                    && (!object1.wrap_u || (*u > 0.1 && *s > 0.1))
                    && (!object1.wrap_v || (*v > 0.1 && *t > 0.1))
            })
            .min_by(|x, y| {
                let dx = (x.1 - *cursor_position).to_nalgebra().norm();
                let dy = (y.1 - *cursor_position).to_nalgebra().norm();

                let lx = x.0.log(0.1) as i32;
                let ly = y.0.log(0.1) as i32;

                if lx == ly {
                    dx.partial_cmp(&dy).unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    lx.cmp(&ly)
                }
            })
            .map(|(_, _, _, u, v, s, t)| {
                let uv = Vector2::new(u, v);
                let st = Vector2::new(s, t);
                (uv, st)
            })
    }

    fn find_intersection(
        object1: &IntersectionObject,
        object2: &IntersectionObject,
        uv_start: Vector2<f32>,
        st_start: Vector2<f32>,
        newton_factor: f32,
        rough: bool,
        max_distance: f32,
        self_intersection: bool,
    ) -> Option<(Vec<Vector2<f32>>, Vec<Vector2<f32>>, bool)> {
        let mut uv_points = vec![];
        let mut st_points = vec![];

        let mut uv = uv_start;
        let mut st = st_start;
        let mut uv_newton = uv_start;
        let mut st_newton = st_start;

        let mut t = nalgebra::Vector3::zeros();
        let mut found_first_bound = false;

        loop {
            let mut step = 0.05;
            let mut distance = f32::MAX;
            let pos_r3 = object1.get_value(uv_newton.x, uv_newton.y).to_nalgebra();
            let mut out_of_bounds = false;
            let mut t_now = object1
                .get_normal(uv_newton.x, uv_newton.y)
                .cross(&object2.get_normal(st_newton.x, st_newton.y))
                .normalize();

            if found_first_bound {
                t_now = -t_now;
            }

            if t_now.dot(&t.normalize()).abs() > 0.995 {
                t_now = t;
            }

            if t.dot(&t_now) < 0.0 {
                t_now = -t_now;
            }

            let mut i = 0;
            while i < 150 {
                i += 1;
                let p = object1.get_value(uv_newton.x, uv_newton.y).to_nalgebra();
                let q = object2.get_value(st_newton.x, st_newton.y).to_nalgebra();
                let grad_p = object1.get_grad(uv_newton.x, uv_newton.y);
                let grad_q = object2.get_grad(st_newton.x, st_newton.y);
                let free = p - q;
                let free = Vector4::new(free.x, free.y, free.z, (p - pos_r3).dot(&t_now) - step);
                let distance_newton = free.norm();
                if distance_newton < max_distance {
                    uv = uv_newton;
                    st = st_newton;
                    break;
                }
                if distance_newton > distance {
                    if step <= 0.0001 {
                        if found_first_bound {
                            return Some((uv_points, st_points, false));
                        }
                        found_first_bound = true;
                        break;
                    }
                    step /= 2.0;
                    distance = f32::MAX;
                    uv_newton = uv;
                    st_newton = st;
                    i = 0;
                    continue;
                }
                distance = distance_newton;
                let jacobian = Matrix4::new(
                    grad_p.0.x,
                    grad_p.0.y,
                    grad_p.0.z,
                    grad_p.0.to_nalgebra().dot(&t_now),
                    grad_p.1.x,
                    grad_p.1.y,
                    grad_p.1.z,
                    grad_p.1.to_nalgebra().dot(&t_now),
                    -grad_q.0.x,
                    -grad_q.0.y,
                    -grad_q.0.z,
                    0.0,
                    -grad_q.1.x,
                    -grad_q.1.y,
                    -grad_q.1.z,
                    0.0,
                );
                let mut uvst = Vector4::new(uv_newton.x, uv_newton.y, st_newton.x, st_newton.y);
                let duvst = math::matrix4::Matrix4::from_nalgebra(jacobian)
                    .get_transposed()
                    .get_inversed()
                    .to_nalgebra()
                    * free;
                uvst -= duvst * newton_factor;
                uv_newton = Vector2::new(uvst.x, uvst.y);
                st_newton = Vector2::new(uvst.z, uvst.w);
                if uv_newton.x < 0.0
                    || uv_newton.x > object1.value_range.0
                    || uv_newton.y < 0.0
                    || uv_newton.y > object1.value_range.1
                    || st_newton.x < 0.0
                    || st_newton.x > object2.value_range.0
                    || st_newton.y < 0.0
                    || st_newton.y > object2.value_range.1
                {
                    out_of_bounds = true;
                }
                if out_of_bounds || rough || duvst.norm() < 0.001 {
                    let uvn = object1.clamp_uv(uv_newton.x, uv_newton.y);
                    let stn = object2.clamp_uv(st_newton.x, st_newton.y);
                    uv_newton = Vector2::new(uvn.0, uvn.1);
                    st_newton = Vector2::new(stn.0, stn.1);
                    break;
                }
            }
            if !found_first_bound {
                uv_points.push(uv_newton);
                st_points.push(st_newton);
            } else {
                uv_points.insert(0, uv_newton);
                st_points.insert(0, st_newton);
            }

            if (out_of_bounds
                && (object1.get_value(uv_newton.x, uv_newton.y)
                    - object2.get_value(st_newton.x, st_newton.y))
                .to_nalgebra()
                .norm()
                    > 0.1)
                || (self_intersection
                    && ((uv_newton - st_newton).norm() < 0.01
                        || (object1.get_value(uv_newton.x, uv_newton.y)
                            - object2.get_value(st_newton.x, st_newton.y))
                        .to_nalgebra()
                        .norm()
                            > 0.1))
            {
                if found_first_bound {
                    return Some((uv_points, st_points, false));
                }
                uv = uv_start;
                st = st_start;
                uv_newton = uv_start;
                st_newton = st_start;
                t = nalgebra::Vector3::zeros();
                found_first_bound = true;
            } else {
                t = t_now;
            }

            if !found_first_bound
                && uv_points.len() > 2
                && (object1.get_value(uv_points[0].x, uv_points[0].y)
                    - object1.get_value(
                        uv_points[uv_points.len() - 1].x,
                        uv_points[uv_points.len() - 1].y,
                    ))
                .to_nalgebra()
                .norm()
                    < step
            {
                return Some((uv_points, st_points, true));
            }

            if uv_points.len() > 1000000 {
                break;
            }

            println!("{}", uv_points.len());
        }

        None
    }

    fn get_texture(
        texture_size: usize,
        uv_points: &[(f32, f32)],
        value_ranges: (f32, f32),
        wrap_u: bool,
        wrap_v: bool,
        wrap: bool,
    ) -> Vec<BitVec> {
        let mut result = vec![BitVec::from_elem(texture_size, false); texture_size];

        uv_points
            .iter()
            .chain_if([uv_points[0]].iter(), wrap)
            .map(|(u, v)| {
                let u = (u / value_ranges.0 * texture_size as f32).round() as usize;
                let v = (v / value_ranges.1 * texture_size as f32).round() as usize;

                (u as i64, v as i64)
            })
            .tuple_windows()
            .for_each(|(p1, p2)| {
                let (p1, p2) = [
                    Some((p1, p2)),
                    if wrap_u {
                        Some(((p1.0 + texture_size as i64, p1.1), p2))
                    } else {
                        None
                    },
                    if wrap_u {
                        Some(((p1.0 - texture_size as i64, p1.1), p2))
                    } else {
                        None
                    },
                    if wrap_v {
                        Some(((p1.0, p1.1 + texture_size as i64), p2))
                    } else {
                        None
                    },
                    if wrap_v {
                        Some(((p1.0, p1.1 - texture_size as i64), p2))
                    } else {
                        None
                    },
                    if wrap_u && wrap_v {
                        Some(((p1.0 + texture_size as i64, p1.1 - texture_size as i64), p2))
                    } else {
                        None
                    },
                    if wrap_u && wrap_v {
                        Some(((p1.0 - texture_size as i64, p1.1 + texture_size as i64), p2))
                    } else {
                        None
                    },
                    if wrap_u && wrap_v {
                        Some(((p1.0 + texture_size as i64, p1.1 + texture_size as i64), p2))
                    } else {
                        None
                    },
                    if wrap_u && wrap_v {
                        Some(((p1.0 - texture_size as i64, p1.1 - texture_size as i64), p2))
                    } else {
                        None
                    },
                ]
                .into_iter()
                .filter_map(|x| x)
                .min_by(|a, b| {
                    let a = (a.0 .0 - a.1 .0).abs() + (a.0 .1 - a.1 .1).abs();
                    let b = (b.0 .0 - b.1 .0).abs() + (b.0 .1 - b.1 .1).abs();
                    a.cmp(&b)
                })
                .unwrap();

                let b = Bresenham::new(p1, p2);

                for (u, v) in b {
                    let u = if u < 0 {
                        u + texture_size as i64
                    } else if u >= texture_size as i64 {
                        u - texture_size as i64
                    } else {
                        u
                    };
                    let v = if v < 0 {
                        v + texture_size as i64
                    } else if v >= texture_size as i64 {
                        v - texture_size as i64
                    } else {
                        v
                    };
                    let u = u as usize;
                    let v = v as usize;
                    result[u].set(v, true);
                }
            });

        let mut queue = VecDeque::new();
        let mut visited = vec![false; texture_size * texture_size];

        let first_point = result
            .iter()
            .enumerate()
            .flat_map(|(i, x)| x.iter().enumerate().map(move |(j, y)| (i, j, y)))
            .find(|(_, _, y)| !*y)
            .unwrap();
        queue.push_back((first_point.0, first_point.1));
        visited[first_point.0 * texture_size + first_point.1] = true;

        while !queue.is_empty() {
            let (u, v) = queue.pop_front().unwrap();

            result[u].set(v, true);

            for (du, dv) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let mut new_u = u as isize + du;
                let mut new_v = v as isize + dv;

                if wrap_u {
                    if new_u < 0 {
                        new_u += texture_size as isize;
                    } else if new_u >= texture_size as isize {
                        new_u -= texture_size as isize;
                    }
                }
                if wrap_v {
                    if new_v < 0 {
                        new_v += texture_size as isize;
                    } else if new_v >= texture_size as isize {
                        new_v -= texture_size as isize;
                    }
                }

                if new_u >= 0
                    && new_u < texture_size as isize
                    && new_v >= 0
                    && new_v < texture_size as isize
                {
                    let new_u = new_u as usize;
                    let new_v = new_v as usize;
                    if !visited[new_u * texture_size + new_v] && !result[new_u][new_v] {
                        queue.push_back((new_u, new_v));
                        visited[new_u * texture_size + new_v] = true;
                    }
                }
            }
        }

        result
    }
}
