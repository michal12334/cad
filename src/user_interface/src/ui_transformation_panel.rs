use egui::{DragValue, Resize, ScrollArea, Slider, Widget};

use backend::cqrs::common::transform_selected_objects::TransformSelectedObjects;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::cursors::transform_cursor::TransformCursor;
use backend::cqrs::points::point_details::{LittleTransformerDTO, PointDTO, PointDetails};
use backend::cqrs::points::rename_point::RenamePoint;
use backend::cqrs::points::transform_point::TransformPoint;
use backend::cqrs::toruses::rename_torus::RenameTorus;
use backend::cqrs::toruses::torus_details::{TorusDTO, TorusDetails, TransformerDTO};
use backend::cqrs::toruses::transform_torus::TransformTours;
use backend::cqrs::toruses::update_torus::UpdateTorus;
use math::operations::multiply_quaternions;
use crate::domain::bezier_c0::BezierC0;

use crate::object::Object::{BeziersC0, Point, Torus};
use crate::object_id::ObjectId;
use crate::ui::Ui;

impl Ui {
    pub fn build_selected_object_transformation_panel(
        &mut self,
        ui: &mut egui::Ui,
        cqrs: &mut CQRS,
    ) {
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
            transformer_drags.push(
                DragValue::new(&mut cursor.transformer.position.0)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("position X");
        });
        ui.horizontal(|ui| {
            transformer_drags.push(
                DragValue::new(&mut cursor.transformer.position.1)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("position Y");
        });
        ui.horizontal(|ui| {
            transformer_drags.push(
                DragValue::new(&mut cursor.transformer.position.2)
                    .speed(0.01)
                    .ui(ui),
            );
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
            BeziersC0(bezier) => {
                Ui::build_bezier_transformation_panel(ui, cqrs, bezier);
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

        ui.label("Position");

        ui.horizontal(|ui| {
            transformer_sliders.push(
                DragValue::new(&mut group_transformer.position.0)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("X");
            transformer_sliders.push(
                DragValue::new(&mut group_transformer.position.1)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("Y");
            transformer_sliders.push(
                DragValue::new(&mut group_transformer.position.2)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("Z");
        });

        ui.label("Scale");

        ui.horizontal(|ui| {
            transformer_sliders.push(
                DragValue::new(&mut group_transformer.scale.0)
                    .speed(0.01)
                    .clamp_range(0.01..=100.0)
                    .ui(ui),
            );
            ui.label("X");
            transformer_sliders.push(
                DragValue::new(&mut group_transformer.scale.1)
                    .speed(0.01)
                    .clamp_range(0.01..=100.0)
                    .ui(ui),
            );
            ui.label("Y");
            transformer_sliders.push(
                DragValue::new(&mut group_transformer.scale.2)
                    .speed(0.01)
                    .clamp_range(0.01..=100.0)
                    .ui(ui),
            );
            ui.label("Z");
        });

        transformer_sliders.append(&mut vec![
            Slider::new(&mut group_transformer.rotation.0, -1.0..=1.0)
                .step_by(0.1)
                .text("rotation X")
                .ui(ui),
            Slider::new(&mut group_transformer.rotation.1, -1.0..=1.0)
                .step_by(0.1)
                .text("rotation Y")
                .ui(ui),
            Slider::new(&mut group_transformer.rotation.2, -1.0..=1.0)
                .step_by(0.1)
                .text("rotation Z")
                .ui(ui),
            Slider::new(&mut group_transformer.rotation.3, -1.0..=1.0)
                .step_by(0.1)
                .text("rotation W")
                .ui(ui),
        ]);

        if transformer_sliders.iter().any(|f| f.changed()) {
            let delta = TransformerDTO {
                position: (
                    group_transformer.position.0
                        - self
                            .previous_group_transformation
                            .as_ref()
                            .unwrap()
                            .position
                            .0,
                    group_transformer.position.1
                        - self
                            .previous_group_transformation
                            .as_ref()
                            .unwrap()
                            .position
                            .1,
                    group_transformer.position.2
                        - self
                            .previous_group_transformation
                            .as_ref()
                            .unwrap()
                            .position
                            .2,
                ),
                rotation: multiply_quaternions(
                    (
                        group_transformer.rotation.0,
                        group_transformer.rotation.1,
                        group_transformer.rotation.2,
                        group_transformer.rotation.3,
                    ),
                    (
                        -self
                            .previous_group_transformation
                            .as_ref()
                            .unwrap()
                            .rotation
                            .0,
                        -self
                            .previous_group_transformation
                            .as_ref()
                            .unwrap()
                            .rotation
                            .1,
                        -self
                            .previous_group_transformation
                            .as_ref()
                            .unwrap()
                            .rotation
                            .2,
                        self.previous_group_transformation
                            .as_ref()
                            .unwrap()
                            .rotation
                            .3,
                    ),
                ),
                scale: (
                    group_transformer.scale.0
                        / self.previous_group_transformation.as_ref().unwrap().scale.0,
                    group_transformer.scale.1
                        / self.previous_group_transformation.as_ref().unwrap().scale.1,
                    group_transformer.scale.2
                        / self.previous_group_transformation.as_ref().unwrap().scale.2,
                ),
            };

            cqrs.execute(&TransformSelectedObjects { transformer: delta });

            for so in self.selected_objects.iter() {
                match so {
                    ObjectId::Torus(id) => {
                        let torus = self.objects.iter_mut().find(|t| t.get_id() == *id).unwrap();
                        *torus = Torus(cqrs.get(&TorusDetails { id: *id }));
                    }
                    ObjectId::Point(id) => {
                        let point = self.objects.iter_mut().find(|t| t.get_id() == *id).unwrap();
                        *point = Point(cqrs.get(&PointDetails { id: *id }));
                    }
                    _ => {}
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

        ui.label("Position");

        ui.horizontal(|ui| {
            transformer_sliders.push(
                DragValue::new(&mut torus.transformer.position.0)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("X");
            transformer_sliders.push(
                DragValue::new(&mut torus.transformer.position.1)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("Y");
            transformer_sliders.push(
                DragValue::new(&mut torus.transformer.position.2)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("Z");
        });

        ui.label("Scale");

        ui.horizontal(|ui| {
            transformer_sliders.push(
                DragValue::new(&mut torus.transformer.scale.0)
                    .speed(0.01)
                    .clamp_range(0.01..=100.0)
                    .ui(ui),
            );
            ui.label("X");
            transformer_sliders.push(
                DragValue::new(&mut torus.transformer.scale.1)
                    .speed(0.01)
                    .clamp_range(0.01..=100.0)
                    .ui(ui),
            );
            ui.label("Y");
            transformer_sliders.push(
                DragValue::new(&mut torus.transformer.scale.2)
                    .speed(0.01)
                    .clamp_range(0.01..=100.0)
                    .ui(ui),
            );
            ui.label("Z");
        });

        transformer_sliders.append(&mut vec![
            Slider::new(&mut torus.transformer.rotation.0, -1.0..=1.0)
                .text("rotation X")
                .ui(ui),
            Slider::new(&mut torus.transformer.rotation.1, -1.0..=1.0)
                .text("rotation Y")
                .ui(ui),
            Slider::new(&mut torus.transformer.rotation.2, -1.0..=1.0)
                .text("rotation Z")
                .ui(ui),
            Slider::new(&mut torus.transformer.rotation.3, -1.0..=1.0)
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

        ui.label("Position");

        ui.horizontal(|ui| {
            transformer_sliders.push(
                DragValue::new(&mut point.transformer.position.0)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("X");
            transformer_sliders.push(
                DragValue::new(&mut point.transformer.position.1)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("Y");
            transformer_sliders.push(
                DragValue::new(&mut point.transformer.position.2)
                    .speed(0.01)
                    .ui(ui),
            );
            ui.label("Z");
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

    fn build_bezier_transformation_panel(ui: &mut egui::Ui, cqrs: &mut CQRS, bezier: &mut BezierC0) {
        Resize::default().id_source("resize_bezier_c0").show(ui, |ui| {
            ScrollArea::vertical().id_source("scroll_bezier_c0").show(ui, |ui| {
                for point in bezier.points.iter_mut() {
                    if ui.selectable_label(point.is_selected, &point.name).clicked() { 
                        point.is_selected = !point.is_selected;
                    } 
                }
            })
        });
    }
}
