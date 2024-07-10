use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::{LittleTransformerDTO, PointDTO};

pub struct BezierC0Points {
    pub id: u64,
}

impl Query<BezierC0Points, Vec<PointDTO>> for BezierC0Points {
    fn get(query: &BezierC0Points, app_state: Rc<RefCell<Backend>>) -> Vec<PointDTO> {
        let app_state = app_state.borrow();
        let bezier_c0 = app_state.storage.beziers_c0.get(&query.id).unwrap();
        bezier_c0
            .points
            .iter()
            .map(|point| {
                let p = app_state.storage.points.get(&point.id).unwrap();
                PointDTO {
                    id: p.id,
                    name: p.name.clone(),
                    transformer: LittleTransformerDTO {
                        position: p.transformer.position,
                    },
                }
            })
            .collect()
    }
}
