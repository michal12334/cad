use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend::cqrs::gregories::gregory_details::GregoryDetails;
use backend_events::gregories::gregory_created::GregoryCreated;
use glium::glutin::surface::WindowSurface;
use glium::Display;

use backend::cqrs::cqrs::CQRS;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::domain::gregory::Gregory;
use crate::drawing::drawing_storage::DrawingStorage;

pub struct AddGregoryOnGregoryCreated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
    pub cqrs: CQRS,
    pub display: Rc<Display<WindowSurface>>,
}

impl Consumer<GregoryCreated> for AddGregoryOnGregoryCreated {
    fn consume(&self, event: &GregoryCreated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        let gregory = self.cqrs.get(&GregoryDetails {
            gregory_id: event.gregory_id,
        });
        drawing_storage.gregories.insert(
            event.gregory_id,
            Gregory::new(
                event.gregory_id,
                gregory.tess_level,
                &gregory.points,
                &gregory.vectors,
                gregory.draw_vectors,
                &self.display,
            ),
        );
    }
}

impl AnyConsumer for AddGregoryOnGregoryCreated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
