use backend::cqrs::common::select_objects::ObjectTypeDTO;

pub enum ObjectId {
    Torus(u64),
    Point(u64),
    BezierC0(u64),
    BezierC2(u64),
    BezierInt(u64),
    SurfaceC0(u64),
    SurfaceC2(u64),
}

impl ObjectId {
    pub fn get_id(&self) -> u64 {
        match self {
            ObjectId::Torus(id) => *id,
            ObjectId::Point(id) => *id,
            ObjectId::BezierC0(id) => *id,
            ObjectId::BezierC2(id) => *id,
            ObjectId::BezierInt(id) => *id,
            ObjectId::SurfaceC0(id) => *id,
            ObjectId::SurfaceC2(id) => *id,
        }
    }

    pub fn get_type(&self) -> ObjectTypeDTO {
        match self {
            ObjectId::Torus(_) => ObjectTypeDTO::Torus,
            ObjectId::Point(_) => ObjectTypeDTO::Point,
            ObjectId::BezierC0(_) => ObjectTypeDTO::BezierC0,
            ObjectId::BezierC2(_) => ObjectTypeDTO::BezierC2,
            ObjectId::BezierInt(_) => ObjectTypeDTO::BezierInt,
            ObjectId::SurfaceC0(_) => ObjectTypeDTO::SurfaceC0,
            ObjectId::SurfaceC2(_) => ObjectTypeDTO::SurfaceC2,
        }
    }
}
