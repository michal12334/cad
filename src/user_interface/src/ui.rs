use egui::{Frame, Label, Resize, ScrollArea, Slider, TopBottomPanel, Widget};
use backend::cqrs::add_torus::AddTorus;
use backend::cqrs::cqrs::{Command, CQRS};
use backend::cqrs::new_id::NewId;
use backend::cqrs::torus_details::{TorusDetails, TorusDTO, TransformerDTO};
use backend::cqrs::transform_torus::TransformTours;
use backend::cqrs::update_torus::UpdateTorus;
use crate::torus::Torus;
use crate::typed_text_buffer::TypedTextBuffer;

pub struct Ui {
    toruses: Vec<TorusDTO>,
}

impl Ui {
    pub fn new() -> Self {
        Self { toruses: vec![] }
    }
    
    pub fn build<'a>(&'a mut self, cqrs: &'a mut CQRS<'a>) -> impl FnMut(&egui::Context) + '_ {
        move |egui_ctx| {
            egui::Window::new("panel").show(egui_ctx, |ui| {
                if ui.button("Add Torus").clicked() {
                    let id = cqrs.handle(&NewId {});
                    cqrs.execute(&AddTorus {
                        id,
                        major_radius: 1.0,
                        minor_radius: 0.5,
                        major_segments: 100,
                        minor_segments: 100,
                    });
                    self.toruses.push(cqrs.get(&TorusDetails { id }));
                }
                
                Resize::default().id_source("resize_1").show(ui, |ui| {
                    ScrollArea::vertical().id_source("a").show(ui, |ui| {
                        for torus in self.toruses.iter_mut() {
                            if ui.selectable_label(false, format!("Torus {}", torus.id)).clicked() {}
                        }
                    });
                });

                Resize::default().id_source("resize_2").show(ui, |ui| {
                    ScrollArea::vertical().id_source("a2").show(ui, |ui| {
                        for torus in self.toruses.iter_mut() {
                            let torus_sliders = vec![
                                Slider::new(&mut torus.major_radius, 0.01..=5.0).text("major radius").ui(ui),
                                Slider::new(&mut torus.minor_radius, 0.01..=5.0).text("minor radius").ui(ui),
                                Slider::new(&mut torus.major_segments, 1..=1000).text("major segments").ui(ui),
                                Slider::new(&mut torus.minor_segments, 1..=1000).text("minor segments").ui(ui),
                            ];
    
                            if torus_sliders.iter().any(|f| f.changed()) {
                                cqrs.execute(&UpdateTorus {
                                    id: torus.id,
                                    major_radius: torus.major_radius,
                                    minor_radius: torus.minor_radius,
                                    major_segments: torus.major_segments,
                                    minor_segments: torus.minor_segments,
                                });
                                *torus = cqrs.get(&TorusDetails { id: torus.id });
                            }
    
                            let transformers_sliders = vec![
                                Slider::new(&mut torus.transformer.position.0, -5.0..=5.0).text("position X").ui(ui),
                                Slider::new(&mut torus.transformer.position.1, -5.0..=5.0).text("position Y").ui(ui),
                                Slider::new(&mut torus.transformer.position.2, -5.0..=5.0).text("position Z").ui(ui),
                                Slider::new(&mut torus.transformer.scale.0, 0.1..=5.0).text("scale X").ui(ui),
                                Slider::new(&mut torus.transformer.scale.1, 0.1..=5.0).text("scale Y").ui(ui),
                                Slider::new(&mut torus.transformer.scale.2, 0.1..=5.0).text("scale Z").ui(ui),
                                Slider::new(&mut torus.transformer.rotation.0, -std::f64::consts::PI..=std::f64::consts::PI).text("rotation X").ui(ui),
                                Slider::new(&mut torus.transformer.rotation.1, -std::f64::consts::PI..=std::f64::consts::PI).text("rotation Y").ui(ui),
                                Slider::new(&mut torus.transformer.rotation.2, -std::f64::consts::PI..=std::f64::consts::PI).text("rotation Z").ui(ui),
                            ];
    
                            if transformers_sliders.iter().any(|f| f.changed()) {
                                cqrs.execute(&TransformTours {
                                    id: torus.id,
                                    transformer: TransformerDTO {
                                        position: torus.transformer.position,
                                        rotation: torus.transformer.rotation,
                                        scale: torus.transformer.scale,
                                    },
                                });
                                *torus = cqrs.get(&TorusDetails { id: torus.id });
                            }
                        }
                    });
                });
            });
        }
    }
}
