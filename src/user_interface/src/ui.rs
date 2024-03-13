use backend::cqrs::cqrs::{Command, CQRS};
use backend::cqrs::torus_details::TorusDetails;
use backend::cqrs::update_torus::UpdateTorus;
use backend::cqrs::update_transformer::UpdateTransformer;
use crate::torus::Torus;
use crate::typed_text_buffer::TypedTextBuffer;

pub struct Ui {
    torus: Option<Torus>,
}

impl Ui {
    pub fn new() -> Self {
        Self { torus: None }
    }
    
    pub fn build<'a>(&'a mut self, cqrs: &'a mut CQRS<'a>) -> impl FnMut(&egui::Context) + '_ {
        move |egui_ctx| {
            egui::SidePanel::left("side_panel").exact_width(183.0).show(egui_ctx, |ui| {
                if self.torus.is_none() {
                    self.torus = Some(Torus::from_dto(&cqrs.get(&TorusDetails {})));
                }
                
                let torus_boxes = vec![
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().major_radius),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().minor_radius),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().major_segments),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().minor_segments),
                ];
                
                if torus_boxes.iter().any(|f| f.changed()) { 
                    let torus = self.torus.as_ref().unwrap();
                    cqrs.execute(&UpdateTorus {
                        major_radius: torus.major_radius.value(),
                        minor_radius: torus.minor_radius.value(),
                        major_segments: torus.major_segments.value(),
                        minor_segments: torus.minor_segments.value(),
                    });
                    self.torus = Some(Torus::from_dto(&cqrs.get(&TorusDetails {})));
                }

                let transformer_boxes = vec![
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().position.0),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().position.1),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().position.2),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().rotation.0),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().rotation.1),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().rotation.2),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().scale.0),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().scale.1),
                    ui.text_edit_singleline(&mut self.torus.as_mut().unwrap().scale.2),
                ];
                
                if transformer_boxes.iter().any(|f| f.changed()) { 
                    let torus = self.torus.as_ref().unwrap();
                    cqrs.execute(&UpdateTransformer {
                        position: (
                            torus.position.0.value(),
                            torus.position.1.value(),
                            torus.position.2.value(),
                        ),
                        rotation: (
                            torus.rotation.0.value(),
                            torus.rotation.1.value(),
                            torus.rotation.2.value(),
                        ),
                        scale: (
                            torus.scale.0.value(),
                            torus.scale.1.value(),
                            torus.scale.2.value(),
                        ),
                    });
                    self.torus = Some(Torus::from_dto(&cqrs.get(&TorusDetails {})));
                }
            });
        }
    }
}
