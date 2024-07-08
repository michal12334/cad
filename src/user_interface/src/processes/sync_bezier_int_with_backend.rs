use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend::cqrs::cqrs::CQRS;
use backend_events::point_added_to_bezier_int::PointAddedToBezierInt;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::domain::bezier_int::BezierIntPoint;
use crate::object::Object;
use crate::ui::Ui;

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

impl AnyConsumer for SyncBezierIntAddedPointWithBackend {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
