use std::{any::Any, cell::RefCell, rc::Rc};

use backend_events::intersections::intersection_textures_draw_set::{
    IntersectionObjectIdDTO, IntersectionTexturesDrawSet,
};
use infrastructure::{
    consumer::{AnyConsumer, Consumer},
    event_bus::EventBus,
};

use crate::drawing::processes::surfaces_c0::update_surface_c0_texture::UpdateSurfaceC0Texture;

pub struct UpdateObjectsTexturesOnIntersectionTexturesDrawSet {
    pub bus: Rc<RefCell<EventBus>>,
}

impl Consumer<IntersectionTexturesDrawSet> for UpdateObjectsTexturesOnIntersectionTexturesDrawSet {
    fn consume(&self, event: &IntersectionTexturesDrawSet) {
        match event.id1 {
            IntersectionObjectIdDTO::Torus(id) => {}
            IntersectionObjectIdDTO::SurfaceC0(id) => self
                .bus
                .borrow()
                .publish(Rc::new(UpdateSurfaceC0Texture { id })),
            IntersectionObjectIdDTO::SurfaceC2(id) => {}
        }
        match event.id2 {
            IntersectionObjectIdDTO::Torus(id) => {}
            IntersectionObjectIdDTO::SurfaceC0(id) => self
                .bus
                .borrow()
                .publish(Rc::new(UpdateSurfaceC0Texture { id })),
            IntersectionObjectIdDTO::SurfaceC2(id) => {}
        }
    }
}

impl AnyConsumer for UpdateObjectsTexturesOnIntersectionTexturesDrawSet {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
