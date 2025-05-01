use std::{any::Any, cell::RefCell, rc::Rc};

use backend_events::intersections::intersection_textures_draw_set::{
    IntersectionObjectIdDTO, TextureDraw,
};
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{
    backend::Backend,
    domain::{
        events::intersections::{
            intersection_created::IntersectionCreated,
            intersection_textures_draw_set::IntersectionTexturesDrawSet,
        },
        intersection::IntersectionObjectId,
    },
};

pub struct IntersectionCreatedPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<IntersectionCreated> for IntersectionCreatedPublisher {
    fn consume(&self, event: &IntersectionCreated) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::intersections::intersection_created::IntersectionCreated::new(
                event.id,
                event.name.clone(),
                event.uv_texture.clone(),
                event.st_texture.clone(),
                event.points.clone(),
                event.wrap,
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

impl AnyConsumer for IntersectionCreatedPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

pub struct IntersectionTexturesDrawSetPublisher {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<IntersectionTexturesDrawSet> for IntersectionTexturesDrawSetPublisher {
    fn consume(&self, event: &IntersectionTexturesDrawSet) {
        let backend = self.backend.borrow();
        let event = Rc::new(
            backend_events::intersections::intersection_textures_draw_set::IntersectionTexturesDrawSet::new(
                event.id,
                TextureDraw::from_bits(event.uv_draw.bits()).unwrap(),
                TextureDraw::from_bits(event.st_draw.bits()).unwrap(),
                map_id(&event.id1),
                map_id(&event.id2),
            ),
        );
        backend.services.event_publisher.publish(event);
    }
}

fn map_id(domain_id: &IntersectionObjectId) -> IntersectionObjectIdDTO {
    match domain_id {
        IntersectionObjectId::Torus(id) => IntersectionObjectIdDTO::Torus(*id),
        IntersectionObjectId::SurfaceC0(id) => IntersectionObjectIdDTO::SurfaceC0(*id),
        IntersectionObjectId::SurfaceC2(id) => IntersectionObjectIdDTO::SurfaceC2(*id),
    }
}

impl AnyConsumer for IntersectionTexturesDrawSetPublisher {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
