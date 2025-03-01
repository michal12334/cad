use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use backend::cqrs::cqrs::CQRS;
use backend_events::points::selected_points_merged::SelectedPointsMerged;
use infrastructure::consumer::{AnyConsumer, Consumer};

use crate::ui::Ui;

pub struct FetchObjectsOnSelectedPointsMerged {
    pub ui: Rc<RefCell<Ui>>,
    pub cqrs: CQRS,
}

impl Consumer<SelectedPointsMerged> for FetchObjectsOnSelectedPointsMerged {
    fn consume(&self, _: &SelectedPointsMerged) {
        self.ui.borrow_mut().fetch_objects(&self.cqrs);
    }
}

impl AnyConsumer for FetchObjectsOnSelectedPointsMerged {
    fn consume_any(&self, message: Rc<dyn Any>) {
        self.consume_any_impl(message);
    }
}
