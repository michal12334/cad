use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::backend::Backend;
use crate::domain::events::point_added_to_bezier_int::PointAddedToBezierInt;
use crate::domain::events::point_created::PointCreated;

pub struct AddPointToSelectedBezierIntOnPointCreated {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointCreated> for AddPointToSelectedBezierIntOnPointCreated {
    fn consume(&self, event: &PointCreated) {
        let backend = self.backend.as_ptr();
        let storage = unsafe { &mut (*backend).storage };
        storage
            .selected_objects
            .iter()
            .filter(|object| object.bezier_int_id.is_some())
            .for_each(|object| {
                let id = object.bezier_int_id.unwrap();
                let bezier = storage.beziers_int.get_mut(&id).unwrap();
                let storage = unsafe { &mut (*backend).storage };
                let points: Vec<_> = bezier.points.iter().map(|p| storage.points.get(&p.id).unwrap().clone()).collect();
                let mut points = points;
                let point = storage.points.get(&event.id).unwrap();
                points.push(point.clone());
                bezier.update_points(points);
                unsafe {
                    let backend = &*backend;
                    let event =
                        Rc::new(PointAddedToBezierInt::new(event.id, id, event.name.clone()));
                    backend.services.event_publisher.publish(event);
                }
            });
    }
}

impl AnyConsumer for AddPointToSelectedBezierIntOnPointCreated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
