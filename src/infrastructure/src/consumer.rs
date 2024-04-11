use std::any::Any;
use std::rc::Rc;

pub trait AnyConsumer {
    fn consume_any(&self, message: Rc<dyn Any>);
}

pub trait Consumer<T> {
    fn consume(&self, message: &T);
}

impl<T: 'static> AnyConsumer for dyn Consumer<T> {
    fn consume_any(&self, message: Rc<dyn Any>) {
        if let Some(message) = message.downcast_ref::<T>() {
            self.consume(message);
        }
    }
}
