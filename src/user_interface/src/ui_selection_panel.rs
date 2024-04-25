use egui::{Resize, ScrollArea};

use backend::cqrs::common::select_objects::{ObjectTypeDTO, SelectObjects, SelectionObjectDTO};
use backend::cqrs::cqrs::CQRS;

use crate::object_id::ObjectId;
use crate::ui::Ui;

impl Ui {
    pub fn build_object_selection_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
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
                                    ObjectTypeDTO::BezierC0 => ObjectId::BeziersC0(object_id),
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
}
