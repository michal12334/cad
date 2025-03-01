use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::beziers_int::bezier_int_details::{BezierIntDTO, BezierIntPointDTO};
use crate::cqrs::cqrs::Query;

pub struct AllBeziersInt;

impl Query<AllBeziersInt, Vec<BezierIntDTO>> for AllBeziersInt {
    fn get(_: &AllBeziersInt, app_state: Rc<RefCell<Backend>>) -> Vec<BezierIntDTO> {
        let app_state = app_state.borrow();
        app_state
            .storage
            .beziers_int
            .values()
            .map(|bezier| BezierIntDTO {
                id: bezier.id,
                name: bezier.name.clone(),
                points: bezier
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
            })
            .collect()
    }
}
