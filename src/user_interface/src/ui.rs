use backend::cqrs::cqrs::{Command, CQRS};
use backend::cqrs::torus_details::TorusDetails;
use backend::cqrs::update_torus::UpdateTorus;
use crate::typed_text_buffer::TypedTextBuffer;

pub struct Ui { }

impl Ui {
    pub fn new() -> Self {
        Self { }
    }
    
    pub fn build<'a>(&'a mut self, cqrs: &'a mut CQRS<'a>) -> impl FnMut(&egui::Context) + '_ {
        move |egui_ctx| {
            egui::SidePanel::left("side_panel").exact_width(183.0).show(egui_ctx, |ui| {
                let torus = cqrs.get(&TorusDetails {});
                let mut major_radius = TypedTextBuffer::new(torus.major_radius);
                let mut minor_radius = TypedTextBuffer::new(torus.minor_radius);
                let mut major_segments = TypedTextBuffer::new(torus.major_segments);
                let mut minor_segments = TypedTextBuffer::new(torus.minor_segments);
                if ui.text_edit_singleline(&mut major_radius).changed() {
                    cqrs.execute(&UpdateTorus {
                        major_radius: major_radius.value(),
                        minor_radius: torus.minor_radius,
                        major_segments: torus.major_segments,
                        minor_segments: torus.minor_segments,
                    });
                }
                if ui.text_edit_singleline(&mut minor_radius).changed() {
                    cqrs.execute(&UpdateTorus {
                        major_radius: torus.major_radius,
                        minor_radius: minor_radius.value(),
                        major_segments: torus.major_segments,
                        minor_segments: torus.minor_segments,
                    });
                }
                if ui.text_edit_singleline(&mut major_segments).changed() {
                    cqrs.execute(&UpdateTorus {
                        major_radius: torus.major_radius,
                        minor_radius: torus.minor_radius,
                        major_segments: major_segments.value(),
                        minor_segments: torus.minor_segments,
                    });
                }
                if ui.text_edit_singleline(&mut minor_segments).changed() {
                    cqrs.execute(&UpdateTorus {
                        major_radius: torus.major_radius,
                        minor_radius: torus.minor_radius,
                        major_segments: torus.major_segments,
                        minor_segments: minor_segments.value(),
                    });
                }
            });
        }
    }
}
