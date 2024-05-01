use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use backend_events::bezier_c0_renamed::BezierC0Renamed;
use infrastructure::consumer::{AnyConsumer, Consumer};
use crate::object::Object::BeziersC0;
use crate::ui::Ui;

pub struct SyncBezierC0WithBackend {
    pub ui: Rc<RefCell<Ui>>,
}

impl AnyConsumer for SyncBezierC0WithBackend {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}

impl Consumer<BezierC0Renamed> for SyncBezierC0WithBackend {
    fn consume(&self, event: &BezierC0Renamed) {
        let mut ui = self.ui.borrow_mut();
        ui.objects.iter_mut()
            .filter(|object| object.get_id() == event.id)
            .for_each(|object| {
                match object { 
                    BeziersC0(bezier_c0) => {
                        bezier_c0.name = event.name.clone();
                    }
                    _ => {}
                }
            })
    }
}
