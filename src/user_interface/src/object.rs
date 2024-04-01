use backend::cqrs::common::select_objects::ObjectTypeDTO;
use backend::cqrs::points::point_details::PointDTO;
use backend::cqrs::toruses::torus_details::TorusDTO;

pub enum Object {
    Torus(TorusDTO),
    Point(PointDTO),
}

impl Object {
    pub fn get_id(&self) -> u64 {
        match self {
            Object::Torus(torus) => torus.id,
            Object::Point(point) => point.id,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Object::Torus(torus) => torus.name.clone(),
            Object::Point(point) => point.name.clone(),
        }
    }

    pub fn get_type(&self) -> ObjectTypeDTO {
        match self {
            Object::Torus(_) => ObjectTypeDTO::Torus,
            Object::Point(_) => ObjectTypeDTO::Point,
        }
    }
}
