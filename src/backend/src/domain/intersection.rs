use math::vector3::Vector3;

use super::intersection_object::IntersectionObject;

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
    ) -> Self {
        Self {
            id,
            name,
            object1_id,
            object2_id,
            intersection_points: vec![object1.get_value(0.0, 0.0), object2.get_value(0.0, 0.0)],
        }
    }
}
