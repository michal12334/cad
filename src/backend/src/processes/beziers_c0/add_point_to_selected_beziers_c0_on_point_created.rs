use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::backend::Backend;
use crate::domain::bezier_c0::BezierC0Point;
use crate::domain::events::point_added_to_bezier_c0::PointAddedToBezierC0;
use crate::domain::events::point_created::PointCreated;

pub struct AddPointToSelectedBeziersC0OnPointCreated {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointCreated> for AddPointToSelectedBeziersC0OnPointCreated {
    fn consume(&self, event: &PointCreated) {
        let backend = self.backend.as_ptr();
        let storage = unsafe { &mut (*backend).storage };
        storage
            .selected_objects
            .iter()
            .filter(|object| object.bezier_c0_id.is_some())
            .for_each(|object| {
                let id = object.bezier_c0_id.unwrap();
                let bezier_c0 = storage.beziers_c0.get_mut(&id).unwrap();
                bezier_c0.add_point(BezierC0Point { id: event.id });
                unsafe {
                    let backend = &*backend;
                    let event =
                        Rc::new(PointAddedToBezierC0::new(event.id, id, event.name.clone()));
                    backend.services.event_publisher.publish(event);
                }
            });
    }
}

impl AnyConsumer for AddPointToSelectedBeziersC0OnPointCreated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
