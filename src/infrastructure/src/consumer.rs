use std::any::Any;
use std::rc::Rc;

pub trait AnyConsumer {
    fn consume_any(&self, message: Rc<dyn Any>);
}

pub trait Consumer<T: 'static>: AnyConsumer {
    fn consume_any_impl(&self, message: Rc<dyn Any>) {
        if let Some(message) = message.downcast_ref::<T>() {
            self.consume(message);
        }
    }

    fn consume(&self, message: &T);
}
