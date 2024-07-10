use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPointDTO;
use crate::cqrs::beziers_c2::bezier_c2_details::{BezierC2BSplinePointDTO, BezierC2DTO};
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::LittleTransformerDTO;

pub struct AllBeziersC2;

impl Query<AllBeziersC2, Vec<BezierC2DTO>> for AllBeziersC2 {
    fn get(query: &AllBeziersC2, app_state: Rc<RefCell<Backend>>) -> Vec<BezierC2DTO> {
        let app_state = app_state.borrow();
        app_state
            .storage
            .beziers_c2
            .values()
            .map(|bezier| BezierC2DTO {
                id: bezier.id,
                name: bezier.name.clone(),
                bernstein_points: bezier
                    .bernstein_points
                    .iter()
                    .map(|bp| BezierC2BernsteinPointDTO {
                        transformer: LittleTransformerDTO {
                            position: bp.transformer.position,
                        },
                    })
                    .collect(),
                b_spline_points: bezier
                    .b_spline_points
                    .iter()
                    .map(|bp| {
                        let p = app_state.storage.points.get(&bp.id).unwrap();
                        BezierC2BSplinePointDTO {
                            id: p.id,
                            name: p.name.clone(),
                        }
                    })
                    .collect(),
            })
            .collect()
    }
}
