use std::{any::Any, cell::RefCell, rc::Rc};

use backend_events::intersections::intersection_created::IntersectionCreated;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{domain::intersection::Intersection, object::Object, ui::Ui};

pub struct SyncIntersectionCreation {
    pub ui: Rc<RefCell<Ui>>,
}

impl Consumer<IntersectionCreated> for SyncIntersectionCreation {
    fn consume(&self, event: &IntersectionCreated) {
        let mut ui = self.ui.borrow_mut();
        ui.objects.push(Object::Intersection(Intersection {
            id: event.id,
            name: event.name.clone(),
            uv_texture_handle: None,
            uv_texture: Intersection::get_texture(&event.uv_texture),
            st_texture_handle: None,
            st_texture: Intersection::get_texture(&event.st_texture),
        }));
    }
}

impl AnyConsumer for SyncIntersectionCreation {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
