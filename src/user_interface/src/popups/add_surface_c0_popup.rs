use egui::Context;
use backend::cqrs::cqrs::CQRS;
use crate::popups::popup::Popup;

pub struct AddSurfaceC0Popup {
    is_closed: bool,
}

impl AddSurfaceC0Popup {
    pub fn new() -> Self {
        Self {
            is_closed: false,
        }
    }
}

impl Popup for AddSurfaceC0Popup {
    fn build(&mut self, cqrs: &mut CQRS, context: &Context) {
        egui::Window::new("Add Surface C0")
            .show(context, |ui| {
                ui.label("Add Surface C0");
                
                if ui.button("Close").clicked() {
                    self.is_closed = true;
                }
            });
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }
}
