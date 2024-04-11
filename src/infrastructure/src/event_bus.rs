use std::any::Any;
use std::rc::Rc;
use crate::consumer::AnyConsumer;

pub struct EventBus {
    consumers: Vec<Box<dyn AnyConsumer>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            consumers: Vec::new(),
        }
    }
    
    pub fn add_consumer(&mut self, consumer: impl AnyConsumer + 'static) {
        self.consumers.push(Box::new(consumer));
    }

    pub fn publish(&self, message: Rc<dyn Any>) {
        for consumer in self.consumers.iter() {
            consumer.consume_any(message.clone());
        }
    }
}
