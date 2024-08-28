use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend_events::surfaces_c2::surface_c2_point_moved::SurfaceC2PointMoved;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::backend::Backend;
use crate::domain::events::points::point_moved::PointMoved;

pub struct MoveSurfaceC2PointOnPointMoved {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointMoved> for MoveSurfaceC2PointOnPointMoved {
    fn consume(&self, event: &PointMoved) {
        let backend = self.backend.as_ptr();
        let storage = unsafe { &mut (*backend).storage };
        let publisher = unsafe { &(*backend).services.event_publisher };
        storage.surfaces_c2.values().for_each(|surface_c2| {
            if surface_c2.points.iter().any(|point| point.id == event.id) {
                publisher.publish(Rc::new(SurfaceC2PointMoved::new(surface_c2.id)));
            }
        });
    }
}

impl AnyConsumer for MoveSurfaceC2PointOnPointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
