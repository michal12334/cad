use egui::{Slider, Widget};
use backend::cqrs::cqrs::{Command, CQRS};
use backend::cqrs::torus_details::{TorusDetails, TorusDTO};
use backend::cqrs::update_torus::UpdateTorus;
use backend::cqrs::update_transformer::UpdateTransformer;
use crate::torus::Torus;
use crate::typed_text_buffer::TypedTextBuffer;

pub struct Ui {
    torus: Option<TorusDTO>,
    a: f32,
}

impl Ui {
    pub fn new() -> Self {
        Self { torus: None, a: 0.0 }
    }
    
    pub fn build<'a>(&'a mut self, cqrs: &'a mut CQRS<'a>) -> impl FnMut(&egui::Context) + '_ {
        move |egui_ctx| {
            egui::Window::new("panel").show(egui_ctx, |ui| {
                if self.torus.is_none() {
                    self.torus = Some(cqrs.get(&TorusDetails {}));
                }
                
                let torus_sliders = vec![
                    Slider::new(&mut self.torus.as_mut().unwrap().major_radius, 0.01..=5.0).text("major radius").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().minor_radius, 0.01..=5.0).text("minor radius").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().major_segments, 1..=1000).text("major segments").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().minor_segments, 1..=1000).text("minor segments").ui(ui),
                ];
                
                if torus_sliders.iter().any(|f| f.changed()) { 
                    let torus = self.torus.as_ref().unwrap();
                    cqrs.execute(&UpdateTorus {
                        major_radius: torus.major_radius,
                        minor_radius: torus.minor_radius,
                        major_segments: torus.major_segments,
                        minor_segments: torus.minor_segments,
                    });
                    self.torus = Some(cqrs.get(&TorusDetails {}));
                }

                let transformers_sliders = vec![
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.position.0, -5.0..=5.0).text("position X").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.position.1, -5.0..=5.0).text("position Y").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.position.2, -5.0..=5.0).text("position Z").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.scale.0, 0.1..=5.0).text("scale X").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.scale.1, 0.1..=5.0).text("scale Y").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.scale.2, 0.1..=5.0).text("scale Z").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.rotation.0, -std::f64::consts::PI..=std::f64::consts::PI).text("rotation X").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.rotation.1, -std::f64::consts::PI..=std::f64::consts::PI).text("rotation Y").ui(ui),
                    Slider::new(&mut self.torus.as_mut().unwrap().transformer.rotation.2, -std::f64::consts::PI..=std::f64::consts::PI).text("rotation Z").ui(ui),
                ];

                if transformers_sliders.iter().any(|f| f.changed()) {
                    let torus = self.torus.as_ref().unwrap();
                    cqrs.execute(&UpdateTransformer {
                        position: torus.transformer.position,
                        rotation: torus.transformer.rotation,
                        scale: torus.transformer.scale,
                    });
                    self.torus = Some(cqrs.get(&TorusDetails {}));
                }
            });
        }
    }
}
