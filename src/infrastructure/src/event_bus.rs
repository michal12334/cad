use std::any::Any;
use std::rc::Rc;

use crate::consumer::{AnyConsumer, Consumer};

pub struct EventBus {
    consumers: Vec<Box<dyn AnyConsumer>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            consumers: Vec::new(),
        }
    }

    pub fn add_consumer<TMessage: 'static, T: Consumer<TMessage> + 'static>(
        &mut self,
        consumer: T,
    ) {
        self.consumers.push(Box::new(consumer));
    }

    pub fn publish(&self, message: Rc<dyn Any>) {
        for consumer in self.consumers.iter() {
            consumer.consume_any(message.clone());
        }
    }
}
