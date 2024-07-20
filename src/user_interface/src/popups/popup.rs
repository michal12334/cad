use backend::cqrs::cqrs::CQRS;

pub trait Popup {
    fn build(&mut self, cqrs: &mut CQRS, context: &egui::Context);
    fn is_closed(&self) -> bool;
}
