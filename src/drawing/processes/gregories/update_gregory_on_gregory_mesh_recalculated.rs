use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend::cqrs::gregories::gregory_details::GregoryDetails;
use backend_events::gregories::gregory_mesh_recalculated::GregoryMeshRecalculated;
use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::cqrs::CQRS;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateGregoryOnGregoryMeshRecalculated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<GregoryMeshRecalculated> for UpdateGregoryOnGregoryMeshRecalculated {
    fn consume(&self, event: &GregoryMeshRecalculated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let gregory = self.cqrs.get(&GregoryDetails {
            gregory_id: event.gregory_id,
        });
        drawing_storage
            .gregories
            .iter_mut()
            .filter(|g| g.0 == &event.gregory_id)
            .for_each(|g| {
                g.1.update_mesh(&gregory.points, &gregory.vectors, &self.display)
            });
    }
}

impl AnyConsumer for UpdateGregoryOnGregoryMeshRecalculated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
