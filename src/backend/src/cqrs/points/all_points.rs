use crate::app_state::AppState;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::{LittleTransformerDTO, PointDTO};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AllPoints;

impl Query<AllPoints, Vec<PointDTO>> for AllPoints {
    fn get(_query: &AllPoints, app_state: Rc<RefCell<AppState>>) -> Vec<PointDTO> {
        let app_state = app_state.borrow();
        app_state
            .storage
            .points
            .values()
            .map(|point| PointDTO {
                id: point.id,
                name: point.name.clone(),
                transformer: LittleTransformerDTO {
                    position: point.transformer.position,
                },
            })
            .collect()
    }
}
