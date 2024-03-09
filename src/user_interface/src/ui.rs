use backend::cqrs::cqrs::{Command, CQRS};
use backend::cqrs::update_torus::UpdateTorus;
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
    
    pub fn build<'a>(&'a mut self, cqrs: &'a mut CQRS<'a>) -> impl FnMut(&egui::Context) + '_ {
        move |egui_ctx| {
            egui::SidePanel::left("side_panel").exact_width(183.0).show(egui_ctx, |ui| {
                ui.heading("Hello World!");
                if ui.button("Quit").clicked() {
                }
                if ui.text_edit_singleline(&mut self.position_x).changed() {
                    cqrs.execute(&UpdateTorus {
                        major_radius: 1.0,
                        minor_radius: 0.25,
                        major_segments: 32,
                        minor_segments: 16,
                    });
                }
            });
        }
    }
}
