use std::{any::Any, cell::RefCell, rc::Rc};

use backend_events::gregories::gregory_settings_updated::GregorySettingsUpdated;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::drawing::drawing_storage::DrawingStorage;

pub struct UpdateGreogrySettingsOnGregorySettingsUpdated {
    pub drawing_storage: Rc<RefCell<DrawingStorage>>,
}

impl Consumer<GregorySettingsUpdated> for UpdateGreogrySettingsOnGregorySettingsUpdated {
    fn consume(&self, message: &GregorySettingsUpdated) {
        let mut drawing_storage = self.drawing_storage.borrow_mut();
        drawing_storage
            .gregories
            .iter_mut()
            .filter(|g| g.0 == &message.gregory_id)
            .for_each(|g| g.1.update_settings(message.tess_level));
    }
}

impl AnyConsumer for UpdateGreogrySettingsOnGregorySettingsUpdated {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
