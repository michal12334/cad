use egui::{Frame, Label, Resize, ScrollArea, Slider, TopBottomPanel, Widget};
use backend::cqrs::add_point::AddPoint;
use backend::cqrs::add_torus::AddTorus;
use backend::cqrs::cqrs::{Command, CQRS};
use backend::cqrs::new_id::NewId;
use backend::cqrs::point_details::PointDetails;
use backend::cqrs::select_objects::SelectObjects;
use backend::cqrs::torus_details::{TorusDetails, TorusDTO, TransformerDTO};
use backend::cqrs::transform_torus::TransformTours;
use backend::cqrs::update_torus::UpdateTorus;
use crate::object::Object;
use crate::object::Object::{Point, Torus};

pub struct Ui {
    objects: Vec<Object>,
    // toruses: Vec<TorusDTO>,
    selected_object: Option<u64>,
    pointer_is_over_area: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            selected_object: None,
            pointer_is_over_area: false
        }
    }
    
    pub fn is_pointer_over_area(&self) -> bool {
        self.pointer_is_over_area
    }
    
    pub fn build<'a>(&'a mut self, cqrs: &'a mut CQRS<'a>) -> impl FnMut(&egui::Context) + '_ {
        move |egui_ctx| {
            egui::Window::new("panel").show(egui_ctx, |ui| {
                if egui_ctx.is_pointer_over_area() {
                    self.pointer_is_over_area = true;
                } else {
                    self.pointer_is_over_area = false;
                }
                
                ScrollArea::vertical().id_source("a3").show(ui, |ui| {
                    self.build_object_addition_panel(ui, cqrs);
                    self.build_object_selection_panel(ui, cqrs);
                    self.build_selected_object_transformation_panel(ui, cqrs);
                });
            });
        }
    }

    fn build_object_addition_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        if ui.button("Add Torus").clicked() {
            let id = cqrs.handle(&NewId {});
            cqrs.execute(&AddTorus {
                id,
                major_radius: 1.0,
                minor_radius: 0.5,
                major_segments: 100,
                minor_segments: 100,
            });
            self.objects.push(Torus(cqrs.get(&TorusDetails { id })));
        }
        if ui.button("Add Point").clicked() {
            let id = cqrs.handle(&NewId {});
            cqrs.execute(&AddPoint {
                id,
            });
            self.objects.push(Point(cqrs.get(&PointDetails { id })));
        }
    }

    fn build_object_selection_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        Resize::default().id_source("resize_1").show(ui, |ui| {
            ScrollArea::vertical().id_source("a").show(ui, |ui| {
                for object in self.objects.iter_mut() {
                    let object_id = object.get_id();
                    if ui.selectable_label(Some(object_id) == self.selected_object, format!("Torus {}", object_id)).clicked() {
                        self.selected_object = match Some(object_id) == self.selected_object { 
                            true => {
                                cqrs.execute(&SelectObjects { objects: vec![] });
                                None
                            }, 
                            false => {
                                cqrs.execute(&SelectObjects { objects: vec![object_id] });
                                Some(object_id)
                            },
                        }
                    }
                }
            });
        });
    }
    
    fn build_selected_object_transformation_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        Resize::default().id_source("resize_2").show(ui, |ui| {
            ScrollArea::vertical().id_source("a2").show(ui, |ui| {
                let object = match self.selected_object {
                    Some(id) => self.objects.iter_mut().find(|t| t.get_id() == id).unwrap(),
                    None => return,
                };
                match object {
                    Torus(torus) => {
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
                    _ => {},
                }
            });
        });
    }
}
