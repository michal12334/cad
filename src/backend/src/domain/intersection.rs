use itertools::Itertools;
use math::vector3::Vector3;
use nalgebra::{Matrix4, Vector2, Vector4};

use super::intersection_object::IntersectionObject;

pub struct Intersection {
    pub id: u64,
    pub name: String,
    pub object1_id: IntersectionObjectId,
    pub object2_id: IntersectionObjectId,
    pub intersection_points: Vec<Vector3>,
    pub uv_points: Vec<(f32, f32)>,
    pub st_points: Vec<(f32, f32)>,
}

pub enum IntersectionObjectId {
    Torus(u64),
    SurfaceC0(u64),
    SurfaceC2(u64),
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
    ) -> Self {
        let uvst = Self::find_starting_points(object1, object2, cursor_position);

        let intersection = Self::find_intersection(
            object1,
            object2,
            uvst.unwrap().0,
            uvst.unwrap().1,
            0.2,
            false,
            0.0000001,
            false,
        );

        Self {
            id,
            name,
            object1_id,
            object2_id,
            intersection_points: intersection
                .0
                .iter()
                .map(|uv| object1.get_value(uv.x, uv.y))
                .chain(
                    intersection
                        .1
                        .iter()
                        .map(|st| object2.get_value(st.x, st.y)),
                )
                .collect(),
            uv_points: intersection.0.iter().map(|uv| (uv.x, uv.y)).collect(),
            st_points: intersection.1.iter().map(|st| (st.x, st.y)).collect(),
        }
    }

    fn find_starting_points(
        object1: &IntersectionObject,
        object2: &IntersectionObject,
        cursor_position: &Vector3,
    ) -> Option<(Vector2<f32>, Vector2<f32>)> {
        let x = (0..100)
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
            .sorted_by(|x, y| {
                let dx = (x.1 - *cursor_position).to_nalgebra().norm();
                let dy = (y.1 - *cursor_position).to_nalgebra().norm();

                dx.partial_cmp(&dy).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(_, _, _, u, v, s, t)| {
                let uv = Vector2::new(u, v);
                let st = Vector2::new(s, t);
                (uv, st)
            })
            .next();

        x
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
    ) -> (Vec<Vector2<f32>>, Vec<Vector2<f32>>) {
        let mut uv_points = vec![uv_start];
        let mut st_points = vec![st_start];

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
                            return (uv_points, st_points);
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

            if out_of_bounds
                && (object1.get_value(uv_newton.x, uv_newton.y)
                    - object2.get_value(st_newton.x, st_newton.y))
                .to_nalgebra()
                .norm()
                    > 0.1
            {
                if found_first_bound {
                    return (uv_points, st_points);
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
                break;
            }

            if uv_points.len() > 1000000 {
                break;
            }

            println!("{}", uv_points.len());
        }

        (uv_points, st_points)
    }
}
