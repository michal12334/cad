use std::cell::RefCell;
use std::rc::Rc;
use crate::backend::Backend;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::{LittleTransformerDTO, PointDTO};

pub struct BezierC2BSplinePoints {
    pub id: u64,
}

impl Query<BezierC2BSplinePoints, Vec<PointDTO>> for BezierC2BSplinePoints {
    fn get(query: &BezierC2BSplinePoints, app_state: Rc<RefCell<Backend>>) -> Vec<PointDTO> {
        let app_state = app_state.borrow();
        let bezier = app_state.storage.beziers_c2.get(&query.id).unwrap();
        bezier.b_spline_points
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
