use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::LittleTransformerDTO;

pub struct BezierC2BernsteinPoints {
    pub id: u64,
}

pub struct BezierC2BernsteinPointDTO {
    pub transformer: LittleTransformerDTO,
}

impl Query<BezierC2BernsteinPoints, Vec<BezierC2BernsteinPointDTO>> for BezierC2BernsteinPoints {
    fn get(query: &BezierC2BernsteinPoints, app_state: Rc<RefCell<Backend>>) -> Vec<BezierC2BernsteinPointDTO> {
        let app_state = app_state.borrow();
        let bezier_c2 = app_state.storage.beziers_c2.get(&query.id).unwrap();
        bezier_c2.bernstein_points
            .iter()
            .map(|point| {
                BezierC2BernsteinPointDTO {
                    transformer: LittleTransformerDTO {
                        position: point.transformer.position,
                    },
                }
            })
            .collect()
    }
}
