use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Query;

pub struct BezierC0Details {
    pub id: u64,
}

pub struct BezierC0DTO {
    pub id: u64,
    pub name: String,
    pub points: Vec<BezierC0PointDTO>,
}

pub struct BezierC0PointDTO {
    pub id: u64,
}

impl Query<BezierC0Details, BezierC0DTO> for BezierC0Details {
    fn get(query: &BezierC0Details, app_state: Rc<RefCell<Backend>>) -> BezierC0DTO {
        let app_state = app_state.borrow();
        let bezier_c0 = app_state.storage.beziers_c0.get(&query.id).unwrap();
        BezierC0DTO {
            id: bezier_c0.id,
            name: bezier_c0.name.clone(),
            points: bezier_c0.points.iter().map(|point| BezierC0PointDTO { id: point.id }).collect(),
        }
    }
}
