use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPointDTO;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::LittleTransformerDTO;

pub struct BezierC2Details {
    pub id: u64,
}

pub struct BezierC2DTO {
    pub id: u64,
    pub name: String,
    pub bernstein_points: Vec<BezierC2BernsteinPointDTO>,
    pub b_spline_points: Vec<BezierC2BSplinePointDTO>,
}

pub struct BezierC2BSplinePointDTO {
    pub id: u64,
    pub name: String,
}

impl Query<BezierC2Details, BezierC2DTO> for BezierC2Details {
    fn get(query: &BezierC2Details, app_state: Rc<RefCell<Backend>>) -> BezierC2DTO {
        let app_state = app_state.borrow();
        let bezier_c2 = app_state.storage.beziers_c2.get(&query.id).unwrap();
        BezierC2DTO {
            id: bezier_c2.id,
            name: bezier_c2.name.clone(),
            bernstein_points: bezier_c2
                .bernstein_points
                .iter()
                .map(|bp| BezierC2BernsteinPointDTO {
                    transformer: LittleTransformerDTO {
                        position: bp.transformer.position,
                    },
                })
                .collect(),
            b_spline_points: bezier_c2
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
        }
    }
}
