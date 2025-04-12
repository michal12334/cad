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
}
