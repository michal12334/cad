use crate::domain::bezier_int::BezierIntPoint;
use crate::object::Object;
use crate::ui::Ui;
use backend::cqrs::cqrs::CQRS;
use backend_events::bezier_int_points_deleted::BezierIntPointsDeleted;
use backend_events::point_added_to_bezier_int::PointAddedToBezierInt;
use infrastructure::consumer::{AnyConsumer, Consumer};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SyncBezierIntAddedPointWithBackend {
    pub ui: Rc<RefCell<Ui>>,
}

impl Consumer<PointAddedToBezierInt> for SyncBezierIntAddedPointWithBackend {
    fn consume(&self, event: &PointAddedToBezierInt) {
        let mut ui = self.ui.borrow_mut();
        ui.objects
            .iter_mut()
            .filter(|object| object.get_id() == event.bezier_id)
            .for_each(|object| match object {
                Object::BezierInt(bezier) => {
                    let point = BezierIntPoint {
                        id: event.point_id,
                        name: event.point_name.clone(),
                        is_selected: false,
                    };
                    bezier.points.push(point);
                }
                _ => {}
            });
    }
}

pub struct SyncBezierIntPointsDeletedWithBackend {
    pub ui: Rc<RefCell<Ui>>,
}

impl Consumer<BezierIntPointsDeleted> for SyncBezierIntPointsDeletedWithBackend {
    fn consume(&self, event: &BezierIntPointsDeleted) {
        let mut ui = self.ui.borrow_mut();
        ui.objects
            .iter_mut()
            .filter(|object| object.get_id() == event.id)
            .for_each(|object| match object {
                Object::BezierInt(bezier) => {
                    bezier
                        .points
                        .retain(|point| !event.deleted_points.contains(&point.id));
                }
                _ => {}
            });
    }
}

impl AnyConsumer for SyncBezierIntAddedPointWithBackend {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for SyncBezierIntPointsDeletedWithBackend {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
