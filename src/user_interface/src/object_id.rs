use backend::cqrs::common::select_objects::ObjectTypeDTO;

pub enum ObjectId {
    Torus(u64),
    Point(u64),
    BeziersC0(u64),
    BezierC2(u64),
}

impl ObjectId {
    pub fn get_id(&self) -> u64 {
        match self {
            ObjectId::Torus(id) => *id,
            ObjectId::Point(id) => *id,
            ObjectId::BeziersC0(id) => *id,
            ObjectId::BezierC2(id) => *id,
        }
    }

    pub fn get_type(&self) -> ObjectTypeDTO {
        match self {
            ObjectId::Torus(_) => ObjectTypeDTO::Torus,
            ObjectId::Point(_) => ObjectTypeDTO::Point,
            ObjectId::BeziersC0(_) => ObjectTypeDTO::BezierC0,
            ObjectId::BezierC2(_) => ObjectTypeDTO::BezierC2,
        }
    }
}
