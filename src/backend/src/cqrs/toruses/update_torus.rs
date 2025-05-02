use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::toruses::torus_updated::TorusUpdated;

pub struct UpdateTorus {
    pub id: u64,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub major_segments: u32,
    pub minor_segments: u32,
}

impl Command<UpdateTorus> for UpdateTorus {
    fn execute(command: &UpdateTorus, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        if command.minor_radius >= command.major_radius {
            return;
        }

        let torus = backend.storage.toruses.get_mut(&command.id).unwrap();
        torus.update(
            command.major_radius,
            command.minor_radius,
            command.major_segments,
            command.minor_segments,
        );

        let event = TorusUpdated::new(
            torus.id,
            torus.major_radius,
            torus.minor_radius,
            torus.major_segments,
            torus.minor_segments,
        );

        drop(backend);

        let backend = app_state.borrow();
        backend.services.event_publisher.publish(Rc::new(event));
    }
}
