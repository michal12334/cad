use egui::{DragValue, Resize, ScrollArea, Slider, Widget};
use itertools::Itertools;

use backend::cqrs::common::new_id::NewId;
use backend::cqrs::common::select_objects::{ObjectTypeDTO, SelectionObjectDTO, SelectObjects};
use backend::cqrs::common::transform_selected_objects::TransformSelectedObjects;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::cursors::cursor_details::CursorDetails;
use backend::cqrs::cursors::cursor_details::CursorDTO;
use backend::cqrs::cursors::transform_cursor::TransformCursor;
use backend::cqrs::points::add_point::AddPoint;
use backend::cqrs::points::all_points::AllPoints;
use backend::cqrs::points::point_details::{LittleTransformerDTO, PointDetails, PointDTO};
use backend::cqrs::points::rename_point::RenamePoint;
use backend::cqrs::points::transform_point::TransformPoint;
use backend::cqrs::toruses::add_torus::AddTorus;
use backend::cqrs::toruses::all_toruses::AllToruses;
use backend::cqrs::toruses::rename_torus::RenameTorus;
use backend::cqrs::toruses::torus_details::{TorusDetails, TorusDTO, TransformerDTO};
use backend::cqrs::toruses::transform_torus::TransformTours;
use backend::cqrs::toruses::update_torus::UpdateTorus;
use math::operations::multiply_quaternions;

use crate::object::Object;
use crate::object::Object::{Point, Torus};
use crate::object_id::ObjectId;

pub struct Ui {
    objects: Vec<Object>,
    selected_objects: Vec<ObjectId>,
    cursor: Option<CursorDTO>,
    cursor_selected: bool,
    pointer_is_over_area: bool,
    control_pressed: bool,
    group_transformation: Option<TransformerDTO>,
    previous_group_transformation: Option<TransformerDTO>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            selected_objects: Vec::new(),
            cursor: None,
            cursor_selected: false,
            pointer_is_over_area: false,
            control_pressed: false,
            group_transformation: None,
            previous_group_transformation: None,
        }
    }

    pub fn is_pointer_over_area(&self) -> bool {
        self.pointer_is_over_area
    }

    pub fn set_control_pressed(&mut self, control_pressed: bool) {
        self.control_pressed = control_pressed;
    }

    pub fn fetch_objects(&mut self, cqrs: &mut CQRS) {
        self.objects = cqrs
            .get(&AllToruses)
            .iter()
            .map(|torus| Torus(torus.clone()))
            .chain(
                cqrs.get(&AllPoints)
                    .iter()
                    .map(|point| Point(point.clone())),
            )
            .sorted_by_key(|object| object.get_id())
            .collect();
        self.selected_objects.clear();
    }
    
    pub fn change_point_selection(&mut self, id: u64, cqrs: &mut CQRS) {
        let is_selected = self
            .selected_objects
            .iter()
            .any(|so| so.get_id() == id);
        match is_selected {
            true => {
                if self.control_pressed {
                    self.selected_objects.retain(|so| so.get_id() != id);
                    self.group_transformation = None;
                    self.previous_group_transformation = None;
                } else {
                    self.selected_objects.clear();
                }
                cqrs.execute(&SelectObjects {
                    objects: self
                        .selected_objects
                        .iter()
                        .map(|so| SelectionObjectDTO {
                            id: so.get_id(),
                            object_type: so.get_type(),
                        })
                        .collect(),
                });
            }
            false => {
                self.cursor_selected = false;
                if !self.control_pressed {
                    self.selected_objects.clear();
                } else {
                    self.group_transformation = None;
                    self.previous_group_transformation = None;
                }
                self.selected_objects.push(ObjectId::Point(id));
                cqrs.execute(&SelectObjects {
                    objects: self
                        .selected_objects
                        .iter()
                        .map(|so| SelectionObjectDTO {
                            id: so.get_id(),
                            object_type: so.get_type(),
                        })
                        .collect(),
                });
            }
        }
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
            cqrs.execute(&AddPoint { id });
            self.objects.push(Point(cqrs.get(&PointDetails { id })));
        }
    }

    fn build_object_selection_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        Resize::default().id_source("resize_1").show(ui, |ui| {
            ScrollArea::vertical().id_source("a").show(ui, |ui| {
                let cursor = self.cursor.as_ref().unwrap();
                if ui
                    .selectable_label(self.cursor_selected, &cursor.name)
                    .clicked()
                {
                    self.selected_objects.clear();
                    cqrs.execute(&SelectObjects { objects: vec![] });
                    self.cursor_selected = !self.cursor_selected;
                }

                for object in self.objects.iter_mut() {
                    let object_id = object.get_id();
                    let object_type = object.get_type();
                    let is_selected = self
                        .selected_objects
                        .iter()
                        .any(|so| so.get_id() == object_id);
                    if ui
                        .selectable_label(is_selected, object.get_name())
                        .clicked()
                    {
                        match is_selected {
                            true => {
                                if self.control_pressed {
                                    self.selected_objects.retain(|so| so.get_id() != object_id);
                                    self.group_transformation = None;
                                    self.previous_group_transformation = None;
                                } else {
                                    self.selected_objects.clear();
                                }
                                cqrs.execute(&SelectObjects {
                                    objects: self
                                        .selected_objects
                                        .iter()
                                        .map(|so| SelectionObjectDTO {
                                            id: so.get_id(),
                                            object_type: so.get_type(),
                                        })
                                        .collect(),
                                });
                            }
                            false => {
                                self.cursor_selected = false;
                                if !self.control_pressed {
                                    self.selected_objects.clear();
                                } else {
                                    self.group_transformation = None;
                                    self.previous_group_transformation = None;
                                }
                                self.selected_objects.push(match object_type {
                                    ObjectTypeDTO::Torus => ObjectId::Torus(object_id),
                                    ObjectTypeDTO::Point => ObjectId::Point(object_id),
                                });
                                cqrs.execute(&SelectObjects {
                                    objects: self
                                        .selected_objects
                                        .iter()
                                        .map(|so| SelectionObjectDTO {
                                            id: so.get_id(),
                                            object_type: so.get_type(),
                                        })
                                        .collect(),
                                });
                            }
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
                    self.build_cursor_transformation_panel(ui, cqrs);
                } else if self.selected_objects.len() == 1 {
                    self.build_single_object_transformation_panel(ui, cqrs);
                } else if self.selected_objects.len() > 1 {
                    self.build_multiple_object_transformation_panel(ui, cqrs);
                }
            });
        });
    }

    fn build_cursor_transformation_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        let cursor = self.cursor.as_mut().unwrap();

        let mut transformer_drags = vec![];
        
        ui.horizontal(|ui| {
            transformer_drags.push(DragValue::new(&mut cursor.transformer.position.0).speed(0.01).ui(ui));
            ui.label("position X");
        });
        ui.horizontal(|ui| {
            transformer_drags.push(DragValue::new(&mut cursor.transformer.position.1).speed(0.01).ui(ui));
            ui.label("position Y");
        });
        ui.horizontal(|ui| {
            transformer_drags.push(DragValue::new(&mut cursor.transformer.position.2).speed(0.01).ui(ui));
            ui.label("position Z");
        });

        if transformer_drags.iter().any(|f| f.changed()) {
            cqrs.execute(&TransformCursor {
                transformer: LittleTransformerDTO {
                    position: cursor.transformer.position,
                },
            });
        }
    }
    
    fn build_single_object_transformation_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        let object = self
            .objects
            .iter_mut()
            .find(|t| t.get_id() == self.selected_objects[0].get_id())
            .unwrap();
        match object {
            Torus(torus) => {
                Ui::build_torus_transformation_panel(ui, cqrs, torus);
            }
            Point(point) => {
                Ui::build_point_transformation_panel(ui, cqrs, point);
            }
        }
    }

    fn build_multiple_object_transformation_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        if self.group_transformation.is_none() {
            self.group_transformation = Some(TransformerDTO {
                position: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0, 1.0),
                scale: (1.0, 1.0, 1.0),
            });
            self.previous_group_transformation = Some(TransformerDTO {
                position: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0, 1.0),
                scale: (1.0, 1.0, 1.0),
            });
        }
        
        let group_transformer = self.group_transformation.as_mut().unwrap();

        let mut transformer_sliders = vec![];
        
        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut group_transformer.position.0).speed(0.01).ui(ui));
            ui.label("position X");
        });
        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut group_transformer.position.1).speed(0.01).ui(ui));
            ui.label("position Y");
        });
        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut group_transformer.position.2).speed(0.01).ui(ui));
            ui.label("position Z");
        });

        transformer_sliders.append(&mut vec![
            Slider::new(&mut group_transformer.scale.0, 0.1..=5.0)
                .text("scale X")
                .ui(ui),
            Slider::new(&mut group_transformer.scale.1, 0.1..=5.0)
                .text("scale Y")
                .ui(ui),
            Slider::new(&mut group_transformer.scale.2, 0.1..=5.0)
                .text("scale Z")
                .ui(ui),
            Slider::new(
                &mut group_transformer.rotation.0,
                -1.0..=1.0,
            )
                .step_by(0.1)
                .text("rotation X")
                .ui(ui),
            Slider::new(
                &mut group_transformer.rotation.1,
                -1.0..=1.0,
            )
                .step_by(0.1)
                .text("rotation Y")
                .ui(ui),
            Slider::new(
                &mut group_transformer.rotation.2,
                -1.0..=1.0,
            )
                .step_by(0.1)
                .text("rotation Z")
                .ui(ui),
            Slider::new(
                &mut group_transformer.rotation.3,
                -1.0..=1.0,
            )
                .step_by(0.1)
                .text("rotation W")
                .ui(ui),
        ]);

        if transformer_sliders.iter().any(|f| f.changed()) {
            let delta = TransformerDTO {
                position: (
                    group_transformer.position.0 - self.previous_group_transformation.as_ref().unwrap().position.0,
                    group_transformer.position.1 - self.previous_group_transformation.as_ref().unwrap().position.1,
                    group_transformer.position.2 - self.previous_group_transformation.as_ref().unwrap().position.2,
                ),
                rotation: multiply_quaternions(
                    (
                        group_transformer.rotation.0,
                        group_transformer.rotation.1,
                        group_transformer.rotation.2,
                        group_transformer.rotation.3,
                    ),
                    (
                        -self.previous_group_transformation.as_ref().unwrap().rotation.0,
                        -self.previous_group_transformation.as_ref().unwrap().rotation.1,
                        -self.previous_group_transformation.as_ref().unwrap().rotation.2,
                        self.previous_group_transformation.as_ref().unwrap().rotation.3,
                    ),
                ),
                scale: (
                    group_transformer.scale.0 / self.previous_group_transformation.as_ref().unwrap().scale.0,
                    group_transformer.scale.1 / self.previous_group_transformation.as_ref().unwrap().scale.1,
                    group_transformer.scale.2 / self.previous_group_transformation.as_ref().unwrap().scale.2,
                ),
            };

            cqrs.execute(&TransformSelectedObjects {
                transformer: delta
            });

            for so in self.selected_objects.iter() {
                match so {
                    ObjectId::Torus(id) => {
                        let torus = self
                            .objects
                            .iter_mut()
                            .find(|t| t.get_id() == *id)
                            .unwrap();
                        *torus = Torus(cqrs.get(&TorusDetails { id: *id }));
                    }
                    ObjectId::Point(id) => {
                        let point = self
                            .objects
                            .iter_mut()
                            .find(|t| t.get_id() == *id)
                            .unwrap();
                        *point = Point(cqrs.get(&PointDetails { id: *id }));
                    }
                }
            }

            self.previous_group_transformation = Some(group_transformer.clone());
        }
    }

    fn build_torus_transformation_panel(ui: &mut egui::Ui, cqrs: &mut CQRS, torus: &mut TorusDTO) {
        if ui.text_edit_singleline(&mut torus.name).lost_focus() {
            cqrs.execute(&RenameTorus {
                id: torus.id,
                name: torus.name.clone(),
            });
            *torus = cqrs.get(&TorusDetails { id: torus.id });
        }

        let torus_sliders = vec![
            Slider::new(&mut torus.major_radius, 0.01..=5.0)
                .text("major radius")
                .ui(ui),
            Slider::new(&mut torus.minor_radius, 0.01..=5.0)
                .text("minor radius")
                .ui(ui),
            Slider::new(&mut torus.major_segments, 1..=1000)
                .text("major segments")
                .ui(ui),
            Slider::new(&mut torus.minor_segments, 1..=1000)
                .text("minor segments")
                .ui(ui),
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

        let mut transformer_sliders = vec![];

        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut torus.transformer.position.0).speed(0.01).ui(ui));
            ui.label("position X");
        });
        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut torus.transformer.position.1).speed(0.01).ui(ui));
            ui.label("position Y");
        });
        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut torus.transformer.position.2).speed(0.01).ui(ui));
            ui.label("position Z");
        });

        transformer_sliders.append(&mut vec![
            Slider::new(&mut torus.transformer.scale.0, 0.1..=5.0)
                .text("scale X")
                .ui(ui),
            Slider::new(&mut torus.transformer.scale.1, 0.1..=5.0)
                .text("scale Y")
                .ui(ui),
            Slider::new(&mut torus.transformer.scale.2, 0.1..=5.0)
                .text("scale Z")
                .ui(ui),
            Slider::new(
                &mut torus.transformer.rotation.0,
                -1.0..=1.0,
            )
                .text("rotation X")
                .ui(ui),
            Slider::new(
                &mut torus.transformer.rotation.1,
                -1.0..=1.0,
            )
                .text("rotation Y")
                .ui(ui),
            Slider::new(
                &mut torus.transformer.rotation.2,
                -1.0..=1.0,
            )
                .text("rotation Z")
                .ui(ui),
            Slider::new(
                &mut torus.transformer.rotation.3,
                -1.0..=1.0,
            )
                .text("rotation W")
                .ui(ui),
        ]);

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
    
    fn build_point_transformation_panel(ui: &mut egui::Ui, cqrs: &mut CQRS, point: &mut PointDTO) {
        if ui.text_edit_singleline(&mut point.name).lost_focus() {
            cqrs.execute(&RenamePoint {
                id: point.id,
                name: point.name.clone(),
            });
            *point = cqrs.get(&PointDetails { id: point.id });
        }

        let mut transformer_sliders = vec![];

        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut point.transformer.position.0).speed(0.01).ui(ui));
            ui.label("position X");
        });
        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut point.transformer.position.1).speed(0.01).ui(ui));
            ui.label("position Y");
        });
        ui.horizontal(|ui| {
            transformer_sliders.push(DragValue::new(&mut point.transformer.position.2).speed(0.01).ui(ui));
            ui.label("position Z");
        });

        if transformer_sliders.iter().any(|f| f.changed()) {
            cqrs.execute(&TransformPoint {
                id: point.id,
                transformer: LittleTransformerDTO {
                    position: point.transformer.position,
                },
            });
            *point = cqrs.get(&PointDetails { id: point.id });
        }
    }
}
