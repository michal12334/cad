use crate::consumer::Consumer;

pub struct Topic<T> {
    consumers: Vec<Box<dyn Consumer<T>>>,
}

impl <T> Topic<T> {
    pub fn new() -> Self {
        Self {
            consumers: Vec::new(),
        }
    }
    
    pub fn add_consumer(&mut self, consumer: impl Consumer<T> + 'static) {
        self.consumers.push(Box::new(consumer));
    }

    pub fn publish(&self, message: T) {
        for consumer in self.consumers.iter() {
            consumer.consume(&message);
        }
    }
}
