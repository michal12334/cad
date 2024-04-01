use backend::cqrs::cursors::cursor_details::CursorDetails;
use egui::{Frame, Label, Resize, ScrollArea, Slider, TopBottomPanel, Widget};
use backend::cqrs::toruses::add_torus::AddTorus;
use backend::cqrs::cqrs::{Command, CQRS};
use backend::cqrs::common::new_id::NewId;
use backend::cqrs::points::point_details::{LittleTransformerDTO, PointDetails};
use backend::cqrs::common::select_objects::{ObjectTypeDTO, SelectionObjectDTO, SelectObjects};
use backend::cqrs::cursors::cursor_details::CursorDTO;
use backend::cqrs::cursors::transform_cursor::TransformCursor;
use backend::cqrs::points::add_point::AddPoint;
use backend::cqrs::points::rename_point::RenamePoint;
use backend::cqrs::toruses::torus_details::{TorusDetails, TorusDTO, TransformerDTO};
use backend::cqrs::points::transform_point::TransformPoint;
use backend::cqrs::toruses::rename_torus::RenameTorus;
use backend::cqrs::toruses::transform_torus::TransformTours;
use backend::cqrs::toruses::update_torus::UpdateTorus;
use crate::object::Object;
use crate::object::Object::{Point, Torus};
use crate::object_id::ObjectId;

pub struct Ui {
    objects: Vec<Object>,
    selected_objects: Vec<ObjectId>,
    cursor: Option<CursorDTO>,
    cursor_selected: bool,
    pointer_is_over_area: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            selected_objects: Vec::new(),
            cursor: None,
            cursor_selected: false,
            pointer_is_over_area: false
        }
    }
    
    pub fn is_pointer_over_area(&self) -> bool {
        self.pointer_is_over_area
    }
    
    pub fn build<'a>(&'a mut self, cqrs: &'a mut CQRS<'a>) -> impl FnMut(&egui::Context) + '_ {
        self.cursor = Some(cqrs.get(&CursorDetails {}));
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
                let cursor = self.cursor.as_ref().unwrap();
                if ui.selectable_label(self.cursor_selected, &cursor.name).clicked() {
                    self.selected_objects.clear();
                    cqrs.execute(&SelectObjects { objects: vec![] });
                    self.cursor_selected = !self.cursor_selected;
                }
                
                for object in self.objects.iter_mut() {
                    let object_id = object.get_id();
                    let object_type = object.get_type();
                    let is_selected = self.selected_objects.iter().any(|so| so.get_id() == object_id);
                    if ui.selectable_label(is_selected, object.get_name()).clicked() {
                        match is_selected { 
                            true => {
                                cqrs.execute(&SelectObjects { objects: vec![] });
                                // self.selected_objects.retain(|so| so.get_id() != object_id);
                                self.selected_objects.clear();
                            }, 
                            false => {
                                cqrs.execute(&SelectObjects { objects: vec![ SelectionObjectDTO { id: object_id, object_type, } ] });
                                self.cursor_selected = false;
                                self.selected_objects.clear();
                                self.selected_objects.push(match object_type { 
                                    ObjectTypeDTO::Torus => ObjectId::Torus(object_id),
                                    ObjectTypeDTO::Point => ObjectId::Point(object_id),
                                });
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
                if self.cursor_selected {
                    let cursor = self.cursor.as_mut().unwrap();
                    
                    let transformer_sliders = vec![
                        Slider::new(&mut cursor.transformer.position.0, -5.0..=5.0).text("position X").ui(ui),
                        Slider::new(&mut cursor.transformer.position.1, -5.0..=5.0).text("position Y").ui(ui),
                        Slider::new(&mut cursor.transformer.position.2, -5.0..=5.0).text("position Z").ui(ui),
                    ];

                    if transformer_sliders.iter().any(|f| f.changed()) {
                        cqrs.execute(&TransformCursor {
                            transformer: LittleTransformerDTO {
                                position: cursor.transformer.position,
                            },
                        });
                    }
                    
                    return;
                }
                
                if self.selected_objects.len() != 1 {
                    return;
                }
                
                let object = self.objects.iter_mut().find(|t| t.get_id() == self.selected_objects[0].get_id()).unwrap();
                match object {
                    Torus(torus) => {
                        if ui.text_edit_singleline(&mut torus.name).lost_focus() {
                            cqrs.execute(&RenameTorus {
                                id: torus.id,
                                name: torus.name.clone(),
                            });
                            *torus = cqrs.get(&TorusDetails { id: torus.id });
                        }
                        
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

                        let transformer_sliders = vec![
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

                        if transformer_sliders.iter().any(|f| f.changed()) {
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
                    Point(point) => {
                        if ui.text_edit_singleline(&mut point.name).lost_focus() {
                            cqrs.execute(&RenamePoint {
                                id: point.id,
                                name: point.name.clone(),
                            });
                            *point = cqrs.get(&PointDetails { id: point.id });
                        }
                        
                        let transformer_sliders = vec![
                            Slider::new(&mut point.transformer.position.0, -5.0..=5.0).text("position X").ui(ui),
                            Slider::new(&mut point.transformer.position.1, -5.0..=5.0).text("position Y").ui(ui),
                            Slider::new(&mut point.transformer.position.2, -5.0..=5.0).text("position Z").ui(ui),
                        ];

                        if transformer_sliders.iter().any(|f| f.changed()) {
                            cqrs.execute(&TransformPoint {
                                id: point.id,
                                transformer: LittleTransformerDTO {
                                    position: point.transformer.position,
                                },
                            });
                            *point = cqrs.get(&PointDetails { id: point.id });
                        }
                    },
                }
            });
        });
    }
}
