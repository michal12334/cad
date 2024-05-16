use crate::backend::Backend;
use crate::cqrs::beziers_c0::bezier_c0_details::{BezierC0DTO, BezierC0PointDTO};
use crate::cqrs::cqrs::Query;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AllBeziersC0;

impl Query<AllBeziersC0, Vec<BezierC0DTO>> for AllBeziersC0 {
    fn get(_query: &AllBeziersC0, app_state: Rc<RefCell<Backend>>) -> Vec<BezierC0DTO> {
        let app_state = app_state.borrow();
        app_state
            .storage
            .beziers_c0
            .values()
            .map(|bezier| BezierC0DTO {
                id: bezier.id,
                name: bezier.name.clone(),
                points: bezier
                    .points
                    .iter()
                    .map(|bp| {
                        let p = app_state.storage.points.get(&bp.id).unwrap();
                        BezierC0PointDTO {
                            id: p.id,
                            name: p.name.clone(),
                        }
                    })
                    .collect(),
            })
            .collect()
    }
}
