use std::{any::Any, cell::RefCell, rc::Rc};

use backend_events::gregories::gregory_created::GregoryCreated;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::{domain::gregory::Gregory, object::Object, ui::Ui};

pub struct SyncGregoryCreation {
    pub ui: Rc<RefCell<Ui>>,
}

impl Consumer<GregoryCreated> for SyncGregoryCreation {
    fn consume(&self, event: &GregoryCreated) {
        let mut ui = self.ui.borrow_mut();
        ui.objects.push(Object::Gregory(Gregory {
            id: event.gregory_id,
            name: event.name.clone(),
            tess_level: event.tess_level,
        }));
    }
}

impl AnyConsumer for SyncGregoryCreation {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
