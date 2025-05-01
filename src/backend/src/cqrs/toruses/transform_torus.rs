use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::cqrs::toruses::torus_details::TransformerDTO;
use crate::domain::events::toruses::torus_transformed::TorusTransformed;

pub struct TransformTours {
    pub id: u64,
    pub transformer: TransformerDTO,
}

impl Command<TransformTours> for TransformTours {
    fn execute(command: &TransformTours, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();
        let torus = backend.storage.toruses.get_mut(&command.id).unwrap();
        torus.transform(
            command.transformer.position,
            command.transformer.rotation,
            command.transformer.scale,
        );

        let event = TorusTransformed::new(
            torus.id,
            torus.transformer.position,
            torus.transformer.rotation,
            torus.transformer.scale,
        );

        drop(backend);

        let backend = app_state.borrow();
        backend.services.event_publisher.publish(Rc::new(event));
    }
}
