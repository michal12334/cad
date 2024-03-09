use crate::text_buffers::F32TextBuffer;

pub struct Ui {
    position_x: F32TextBuffer,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            position_x: F32TextBuffer::new(),
        }
    }
    
    pub fn build(&mut self) -> impl FnMut(&egui::Context) + '_ {
        |egui_ctx| {
            egui::SidePanel::left("side_panel").exact_width(183.0).show(egui_ctx, |ui| {
                ui.heading("Hello World!");
                if ui.button("Quit").clicked() {
                }
                if ui.text_edit_singleline(&mut self.position_x).changed() {
                    
                }
            });
        }
    }
}
