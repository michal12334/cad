use std::{any::Any, cell::RefCell, rc::Rc};

use backend_events::intersections::{
    intersection_created::IntersectionCreated, intersection_deleted::IntersectionDeleted,
};
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{
    domain::intersection::{Intersection, TextureDraw},
    object::Object,
    ui::Ui,
};

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
            uv_draw: TextureDraw::Both,
            st_draw: TextureDraw::Both,
        }));
    }
}

impl AnyConsumer for SyncIntersectionCreation {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct SyncIntersectionDeletion {
    pub ui: Rc<RefCell<Ui>>,
}

impl Consumer<IntersectionDeleted> for SyncIntersectionDeletion {
    fn consume(&self, event: &IntersectionDeleted) {
        let mut ui = self.ui.borrow_mut();
        ui.objects.retain(|object| {
            if let Object::Intersection(intersection) = object {
                intersection.id != event.id
            } else {
                true
            }
        });
        ui.selected_objects
            .retain(|object| object.get_id() != event.id);
    }
}

impl AnyConsumer for SyncIntersectionDeletion {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
