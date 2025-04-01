use std::{any::Any, cell::RefCell, rc::Rc};

use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{
    backend::Backend,
    domain::events::{
        gregories::gregory_mesh_recalculated::GregoryMeshRecalculated,
        points::point_moved::PointMoved,
    },
};

pub struct RecalculateGregoriesOnPointMoved {
    pub backend: Rc<RefCell<Backend>>,
}

impl Consumer<PointMoved> for RecalculateGregoriesOnPointMoved {
    fn consume(&self, event: &PointMoved) {
        let backend = self.backend.as_ptr();
        let storage = unsafe { &mut (*backend).storage };
        let publisher = unsafe { &(*backend).services.event_publisher };

        let points = storage.points.clone();

        let mut events = vec![];

        for g in storage
            .gregories
            .values_mut()
            .filter(|g| g.related_points().contains(&event.id))
        {
            g.recalculate_mesh(&points);
            events.push(GregoryMeshRecalculated::new(g.id));
        }

        for e in events {
            publisher.publish(Rc::new(e));
        }
    }
}

impl AnyConsumer for RecalculateGregoriesOnPointMoved {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
