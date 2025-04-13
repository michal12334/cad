use math::vector3::Vector3;

use super::{cursor, intersection_object::IntersectionObject};

pub struct Intersection {
    pub id: u64,
    pub name: String,
    pub object1_id: IntersectionObjectId,
    pub object2_id: IntersectionObjectId,
    pub intersection_points: Vec<Vector3>,
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
        let object1_s = object1.closest_point(cursor_position);
        let object2_s = object2.closest_point(cursor_position);

        Self {
            id,
            name,
            object1_id,
            object2_id,
            intersection_points: vec![
                object1.get_value(object1_s.0, object1_s.1),
                object2.get_value(object2_s.0, object2_s.1),
            ],
        }
    }
}
