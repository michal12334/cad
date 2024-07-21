use backend::cqrs::cqrs::CQRS;
use crate::object::Object;

pub trait Popup {
    fn build(&mut self, cqrs: &mut CQRS, context: &egui::Context) -> Vec<Object>;
    fn is_closed(&self) -> bool;
}
