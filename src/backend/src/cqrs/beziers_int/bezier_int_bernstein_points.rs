use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::LittleTransformerDTO;

pub struct BezierIntBernsteinPoints {
    pub id: u64,
}

pub struct BezierIntBernsteinPointDTO {
    pub transformer: LittleTransformerDTO,
}

impl Query<BezierIntBernsteinPoints, Vec<BezierIntBernsteinPointDTO>> for BezierIntBernsteinPoints {
    fn get(query: &BezierIntBernsteinPoints, app_state: Rc<RefCell<Backend>>) -> Vec<BezierIntBernsteinPointDTO> {
        let app_state = app_state.borrow();
        let bezier_int = app_state.storage.beziers_int.get(&query.id).unwrap();
        bezier_int.bernstein_points
            .iter()
            .map(|point| {
                BezierIntBernsteinPointDTO {
                    transformer: LittleTransformerDTO {
                        position: point.transformer.position,
                    },
                }
            })
            .collect()
    }
}
