use backend::cqrs::common::select_objects::ObjectTypeDTO;

pub enum ObjectId {
    Torus(u64),
    Point(u64),
}

impl ObjectId {
    pub fn get_id(&self) -> u64 {
        match self {
            ObjectId::Torus(id) => *id,
            ObjectId::Point(id) => *id,
        }
    }

    pub fn get_type(&self) -> ObjectTypeDTO {
        match self {
            ObjectId::Torus(_) => ObjectTypeDTO::Torus,
            ObjectId::Point(_) => ObjectTypeDTO::Point,
        }
    }
}
