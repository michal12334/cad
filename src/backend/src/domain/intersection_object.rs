use math::vector3::Vector3;

pub struct IntersectionObject {
    pub value_range: (f32, f32),
    pub value_getter: Box<dyn Fn(f32, f32) -> Vector3>,
    pub wrap_x: bool,
    pub wrap_y: bool,
}

impl IntersectionObject {
    pub fn new(
        value_range: (f32, f32),
        value_getter: impl Fn(f32, f32) -> Vector3 + 'static,
        wrap_x: bool,
        wrap_y: bool,
    ) -> Self {
        Self {
            value_range,
            value_getter: Box::new(value_getter),
            wrap_x,
            wrap_y,
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
}
