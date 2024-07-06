use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Query;

pub struct BezierIntDetails {
    pub id: u64,
}

pub struct BezierIntDTO {
    pub id: u64,
    pub name: String,
    pub points: Vec<BezierIntPointDTO>,
}

pub struct BezierIntPointDTO {
    pub id: u64,
    pub name: String,
}

impl Query<BezierIntDetails, BezierIntDTO> for BezierIntDetails {
    fn get(query: &BezierIntDetails, app_state: Rc<RefCell<Backend>>) -> BezierIntDTO {
        let app_state = app_state.borrow();
        let bezier_int = app_state.storage.beziers_int.get(&query.id).unwrap();
        BezierIntDTO {
            id: bezier_int.id,
            name: bezier_int.name.clone(),
            points: bezier_int
                .points
                .iter()
                .map(|bp| {
                    let p = app_state.storage.points.get(&bp.id).unwrap();
                    BezierIntPointDTO {
                        id: p.id,
                        name: p.name.clone(),
                    }
                })
                .collect(),
        }
    }
}
