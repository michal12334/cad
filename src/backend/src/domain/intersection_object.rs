use math::vector3::Vector3;

use super::intersection::IntersectionObjectId;

pub struct IntersectionObject {
    pub id: IntersectionObjectId,
    pub value_range: (f32, f32),
    pub value_getter: Box<dyn Fn(f32, f32) -> Vector3>,
    pub wrap_u: bool,
    pub wrap_v: bool,
}

impl IntersectionObject {
    pub fn new(
        id: IntersectionObjectId,
        value_range: (f32, f32),
        value_getter: impl Fn(f32, f32) -> Vector3 + 'static,
        wrap_u: bool,
        wrap_v: bool,
    ) -> Self {
        Self {
            id,
            value_range,
            value_getter: Box::new(value_getter),
            wrap_u,
            wrap_v,
        }
    }

    pub fn get_value(&self, u: f32, v: f32) -> Vector3 {
        (self.value_getter)(u, v)
    }

    pub fn closest_point(&self, point: &Vector3) -> (f32, f32) {
        let (u_start, v_start) = self.value_range;
        let mut closest_uv = (u_start, v_start);
        let mut min_distance = (self.get_value(u_start, v_start) - *point).length();

        for i in 0..100 {
            let u = (i as f32) * 0.01 * self.value_range.0;
            for j in 0..100 {
                let v = (j as f32) * 0.01 * self.value_range.1;
                let current_point = self.get_value(u, v);
                let distance = (current_point - *point).length();
                if distance < min_distance {
                    min_distance = distance;
                    closest_uv = (u, v);
                }
            }
        }

        closest_uv
    }

    pub fn get_grad(&self, u: f32, v: f32) -> (Vector3, Vector3) {
        let h = 0.0001;
        let um = Self::clamp(u - h, 0.0, self.value_range.0, self.wrap_u);
        let up = Self::clamp(u + h, 0.0, self.value_range.0, self.wrap_u);
        let vm = Self::clamp(v - h, 0.0, self.value_range.1, self.wrap_v);
        let vp = Self::clamp(v + h, 0.0, self.value_range.1, self.wrap_v);

        let grad_x = (self.get_value(up, v) - self.get_value(um, v)) / (2.0 * h);
        let grad_y = (self.get_value(u, vp) - self.get_value(u, vm)) / (2.0 * h);

        (grad_x, grad_y)
    }

    pub fn get_normal(&self, u: f32, v: f32) -> nalgebra::Vector3<f32> {
        let (grad_x, grad_y) = self.get_grad(u, v);
        grad_x
            .to_nalgebra()
            .cross(&grad_y.to_nalgebra())
            .normalize()
    }

    pub fn clamp_uv(&self, u: f32, v: f32) -> (f32, f32) {
        let u_clamped = Self::clamp(u, 0.0, self.value_range.0, self.wrap_u);
        let v_clamped = Self::clamp(v, 0.0, self.value_range.1, self.wrap_v);
        (u_clamped, v_clamped)
    }

    fn clamp(v: f32, min: f32, max: f32, wrap: bool) -> f32 {
        if wrap {
            if v < min {
                v + max - min
            } else if v > max {
                v - max + min // fixed the cursor position
            } else {
                v
            }
        } else {
            v.clamp(min, max)
        }
    }
}
