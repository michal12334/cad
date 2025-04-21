use backend::cqrs::common::select_objects::ObjectTypeDTO;
use backend::cqrs::points::point_details::PointDTO;
use backend::cqrs::surfaces_c0::surface_c0_details::SurfaceC0DTO;
use backend::cqrs::surfaces_c2::surface_c2_details::SurfaceC2DTO;
use backend::cqrs::toruses::torus_details::TorusDTO;

use crate::domain::bezier_c0::BezierC0;
use crate::domain::bezier_c2::BezierC2;
use crate::domain::bezier_int::BezierInt;
use crate::domain::gregory::Gregory;
use crate::domain::intersection::Intersection;

pub enum Object {
    Torus(TorusDTO),
    Point(PointDTO),
    BezierC0(BezierC0),
    BezierC2(BezierC2),
    BezierInt(BezierInt),
    SurfaceC0(SurfaceC0DTO),
    SurfaceC2(SurfaceC2DTO),
    Gregory(Gregory),
    Intersection(Intersection),
}

impl Object {
    pub fn get_id(&self) -> u64 {
        match self {
            Object::Torus(torus) => torus.id,
            Object::Point(point) => point.id,
            Object::BezierC0(bezier_c0) => bezier_c0.id,
            Object::BezierC2(bezier_c2) => bezier_c2.id,
            Object::BezierInt(bezier_int) => bezier_int.id,
            Object::SurfaceC0(surface_c0) => surface_c0.id,
            Object::SurfaceC2(surface_c2) => surface_c2.id,
            Object::Gregory(gregory) => gregory.id,
            Object::Intersection(intersection) => intersection.id,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Object::Torus(torus) => torus.name.clone(),
            Object::Point(point) => point.name.clone(),
            Object::BezierC0(bezier_c0) => bezier_c0.name.clone(),
            Object::BezierC2(bezier_c2) => bezier_c2.name.clone(),
            Object::BezierInt(bezier_int) => bezier_int.name.clone(),
            Object::SurfaceC0(surface_c0) => surface_c0.name.clone(),
            Object::SurfaceC2(surface_c2) => surface_c2.name.clone(),
            Object::Gregory(gregory) => gregory.name.clone(),
            Object::Intersection(intersection) => intersection.name.clone(),
        }
    }

    pub fn get_type(&self) -> ObjectTypeDTO {
        match self {
            Object::Torus(_) => ObjectTypeDTO::Torus,
            Object::Point(_) => ObjectTypeDTO::Point,
            Object::BezierC0(_) => ObjectTypeDTO::BezierC0,
            Object::BezierC2(_) => ObjectTypeDTO::BezierC2,
            Object::BezierInt(_) => ObjectTypeDTO::BezierInt,
            Object::SurfaceC0(_) => ObjectTypeDTO::SurfaceC0,
            Object::SurfaceC2(_) => ObjectTypeDTO::SurfaceC2,
            Object::Gregory(_) => ObjectTypeDTO::Gregory,
            Object::Intersection(_) => ObjectTypeDTO::Intersection,
        }
    }
}
