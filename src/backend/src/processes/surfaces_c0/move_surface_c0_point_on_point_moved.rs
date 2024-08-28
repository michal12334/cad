use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::surfaces_c0::surface_c0_point_moved::SurfaceC0PointMoved;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::backend::Backend;
use crate::domain::events::points::point_moved::PointMoved;

pub struct MoveSurfaceC0PointOnPointMoved {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointMoved> for MoveSurfaceC0PointOnPointMoved {
    fn consume(&self, event: &PointMoved) {
        let backend = self.backend.as_ptr();
        let storage = unsafe { &mut (*backend).storage };
        let publisher = unsafe { &(*backend).services.event_publisher };
        storage.surfaces_c0.values().for_each(|surface_c0| {
            if surface_c0.points.iter().any(|point| point.id == event.id) {
                publisher.publish(Rc::new(SurfaceC0PointMoved::new(surface_c0.id)));
            }
        });
    }
}

impl AnyConsumer for MoveSurfaceC0PointOnPointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
