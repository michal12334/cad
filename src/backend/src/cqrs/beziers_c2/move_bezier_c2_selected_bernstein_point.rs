use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::cqrs::points::point_details::LittleTransformerDTO;
use crate::domain::events::point_moved::PointMoved;
use crate::domain::transformer::LittleTransformer;

pub struct MoveBezierC2SelectedBernsteinPoint {
    pub bezier_id: u64,
    pub transformer: LittleTransformerDTO,
}

impl Command<MoveBezierC2SelectedBernsteinPoint> for MoveBezierC2SelectedBernsteinPoint {
    fn execute(command: &MoveBezierC2SelectedBernsteinPoint, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend
            .storage
            .beziers_c2
            .get_mut(&command.bezier_id)
            .unwrap();
        let point = bezier.get_point_movement_to_move_selected_bernstein_point(LittleTransformer {
            position: command.transformer.position,
        });
        if let Some((point_id, delta)) = point {
            let point = backend.storage.points.get_mut(&point_id).unwrap();
            point.transformer.position.0 += delta.position.0;
            point.transformer.position.1 += delta.position.1;
            point.transformer.position.2 += delta.position.2;
            let event = Rc::new(PointMoved::new(point.id, point.transformer.position));
            drop(backend);
            let backend = app_state.borrow();
            backend.services.event_publisher.publish(event);
        }
    }
}
