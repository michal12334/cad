use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::beziers_c2::bezier_c2_selected_bernstein_point_set::BezierC2SelectedBernsteinPointSet;

pub struct SetBezierC2SelectedBernsteinPoint {
    pub id: u64,
    pub selected_bernstein_point: Option<usize>,
}

impl Command<SetBezierC2SelectedBernsteinPoint> for SetBezierC2SelectedBernsteinPoint {
    fn execute(command: &SetBezierC2SelectedBernsteinPoint, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let bezier = backend.storage.beziers_c2.get_mut(&command.id).unwrap();
        bezier.set_selected_bernstein_point(command.selected_bernstein_point);
        let event = Rc::new(BezierC2SelectedBernsteinPointSet::new(
            bezier.id,
            bezier.selected_bernstein_point,
        ));
        drop(backend);
        let backend = app_state.borrow();
        backend.services.event_publisher.publish(event);
    }
}
