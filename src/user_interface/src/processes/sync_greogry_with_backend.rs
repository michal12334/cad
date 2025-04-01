use std::{any::Any, cell::RefCell, rc::Rc};

use backend_events::gregories::{gregory_created::GregoryCreated, gregory_renamed::GregoryRenamed};
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

pub struct SyncGregoryName {
    pub ui: Rc<RefCell<Ui>>,
}

impl Consumer<GregoryRenamed> for SyncGregoryName {
    fn consume(&self, event: &GregoryRenamed) {
        let mut ui = self.ui.borrow_mut();
        ui.objects
            .iter_mut()
            .filter(|object| object.get_id() == event.gregory_id)
            .for_each(|object| match object {
                Object::Gregory(gregory) => {
                    gregory.name = event.name.clone();
                }
                _ => {}
            });
    }
}

impl AnyConsumer for SyncGregoryCreation {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl AnyConsumer for SyncGregoryName {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
