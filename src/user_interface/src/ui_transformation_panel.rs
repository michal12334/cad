use egui::{ComboBox, DragValue, Resize, ScrollArea, Slider, Widget};

use backend::cqrs::beziers_c0::add_point_to_bezier_c0::AddPointToBezierC0;
use backend::cqrs::beziers_c0::delete_bezier_c0_points::DeleteBezierC0Points;
use backend::cqrs::beziers_c0::rename_bezier_c0::RenameBezierC0;
use backend::cqrs::beziers_c0::set_bezier_c0_draw_polygon::SetBezierC0DrawPolygon;
use backend::cqrs::beziers_c2::add_point_to_bezier_c2::AddPointToBezierC2;
use backend::cqrs::beziers_c2::delete_bezier_c2_points::DeleteBezierC2Points;
use backend::cqrs::beziers_c2::move_bezier_c2_selected_bernstein_point::MoveBezierC2SelectedBernsteinPoint;
use backend::cqrs::beziers_c2::rename_bezier_c2::RenameBezierC2;
use backend::cqrs::beziers_c2::set_bezier_c2_draw_b_spline_polygon::SetBezierC2DrawBSplinePolygon;
use backend::cqrs::beziers_c2::set_bezier_c2_draw_bernstein_points::SetBezierC2DrawBernsteinPoints;
use backend::cqrs::beziers_c2::set_bezier_c2_draw_bernstein_polygon::SetBezierC2DrawBernsteinPolygon;
use backend::cqrs::beziers_c2::set_bezier_c2_selected_bernstein_point::SetBezierC2SelectedBernsteinPoint;
use backend::cqrs::beziers_int::add_point_to_bezier_int::AddPointToBezierInt;
use backend::cqrs::beziers_int::delete_bezier_int_points::DeleteBezierIntPoints;
use backend::cqrs::beziers_int::rename_bezier_int::RenameBezierInt;
use backend::cqrs::common::transform_selected_objects::TransformSelectedObjects;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::cursors::transform_cursor::TransformCursor;
use backend::cqrs::points::all_points::AllPoints;
use backend::cqrs::points::point_details::{LittleTransformerDTO, PointDTO, PointDetails};
use backend::cqrs::points::rename_point::RenamePoint;
use backend::cqrs::points::transform_point::TransformPoint;
use backend::cqrs::surfaces_c0::rename_surface_c0::RenameSurfaceC0;
use backend::cqrs::surfaces_c0::select_surface_c0_points::SelectSurfaceC0Points;
use backend::cqrs::surfaces_c0::surface_c0_details::SurfaceC0DTO;
use backend::cqrs::surfaces_c0::surface_c0_points::SurfaceC0Points;
use backend::cqrs::surfaces_c0::update_surface_c0::UpdateSurfaceC0;
use backend::cqrs::surfaces_c2::rename_surface_c2::RenameSurfaceC2;
use backend::cqrs::surfaces_c2::select_surface_c2_points::SelectSurfaceC2Points;
use backend::cqrs::surfaces_c2::surface_c2_details::SurfaceC2DTO;
use backend::cqrs::surfaces_c2::update_surface_c2::UpdateSurfaceC2;
use backend::cqrs::toruses::rename_torus::RenameTorus;
use backend::cqrs::toruses::torus_details::{TorusDTO, TorusDetails, TransformerDTO};
use backend::cqrs::toruses::transform_torus::TransformTours;
use backend::cqrs::toruses::update_torus::UpdateTorus;
use math::operations::multiply_quaternions;

use crate::domain::bezier_c0::BezierC0;
use crate::domain::bezier_c2::BezierC2;
use crate::domain::bezier_int::BezierInt;
use crate::object::Object;
use crate::object_id::ObjectId;
use crate::ui::Ui;

impl Ui {
    pub fn build_selected_object_transformation_panel(
        &mut self,
        ui: &mut egui::Ui,
        cqrs: &mut CQRS,
    ) {
        Resize::default()
            .id_source("resize_2")
            .default_height(450.0)
            .show(ui, |ui| {
                ScrollArea::vertical().id_source("a2").show(ui, |ui| {
                    if self.cursor_selected {
                        self.build_cursor_transformation_panel(ui, cqrs);
                    } else if self.selected_objects.len() == 1 {
                        self.build_single_object_transformation_panel(ui, cqrs);
                    } else if self.selected_objects.len() > 1 {
                        self.build_multiple_object_transformation_panel(ui, cqrs);
                    } else {
                        self.build_stereoscopy_settings_panel(ui);
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
            Object::Torus(torus) => {
                Ui::build_torus_transformation_panel(ui, cqrs, torus);
            }
            Object::Point(point) => {
                Ui::build_point_transformation_panel(ui, cqrs, point);
            }
            Object::BezierC0(bezier) => {
                let points = cqrs.get(&AllPoints {});
                Ui::build_bezier_c0_transformation_panel(ui, cqrs, bezier, &points);
            }
            Object::BezierC2(bezier) => {
                let points = cqrs.get(&AllPoints {});
                Ui::build_bezier_c2_transformation_panel(ui, cqrs, bezier, &points);
            }
            Object::BezierInt(bezier) => {
                let points = cqrs.get(&AllPoints {});
                Ui::build_bezier_int_transformation_panel(ui, cqrs, bezier, &points);
            }
            Object::SurfaceC0(surface) => {
                Ui::build_surface_c0_transformation_panel(ui, cqrs, surface);
            }
            Object::SurfaceC2(surface) => {
                Ui::build_surface_c2_transformation_panel(ui, cqrs, surface);
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
                        *torus = Object::Torus(cqrs.get(&TorusDetails { id: *id }));
                    }
                    ObjectId::Point(id) => {
                        let point = self.objects.iter_mut().find(|t| t.get_id() == *id).unwrap();
                        *point = Object::Point(cqrs.get(&PointDetails { id: *id }));
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

    fn build_bezier_c2_transformation_panel(
        ui: &mut egui::Ui,
        cqrs: &mut CQRS,
        bezier: &mut BezierC2,
        points: &[PointDTO],
    ) {
        if ui.text_edit_singleline(&mut bezier.name).lost_focus() {
            cqrs.execute(&RenameBezierC2 {
                id: bezier.id,
                name: bezier.name.clone(),
            });
        }

        Resize::default()
            .id_source("resize_bezier_c2")
            .default_height(140.0)
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .id_source("scroll_bezier_c2")
                    .show(ui, |ui| {
                        for point in bezier.b_spline_points.iter_mut() {
                            if ui
                                .selectable_label(point.is_selected, &point.name)
                                .clicked()
                            {
                                point.is_selected = !point.is_selected;
                            }
                        }
                    })
            });

        ui.horizontal(|ui| {
            ComboBox::from_id_source("Bezier c2 select point")
                .selected_text(if let Some(p) = &bezier.selected_point {
                    &p.1
                } else {
                    ""
                })
                .show_ui(ui, |ui| {
                    for point in points
                        .iter()
                        .filter(|p| !bezier.b_spline_points.iter().any(|bp| bp.id == p.id))
                    {
                        if ui.selectable_label(false, &point.name).clicked() {
                            bezier.selected_point = Some((point.id, point.name.clone()));
                        }
                    }
                });
            if ui.button("Add Point").clicked() {
                if let Some((id, _name)) = &bezier.selected_point {
                    cqrs.execute(&AddPointToBezierC2 {
                        id: bezier.id,
                        point_id: *id,
                    });
                }
                bezier.selected_point = None;
            }
        });

        if ui.button("Delete Points").clicked() {
            cqrs.execute(&DeleteBezierC2Points {
                id: bezier.id,
                points: bezier
                    .b_spline_points
                    .iter()
                    .filter(|p| p.is_selected)
                    .map(|p| p.id)
                    .collect(),
            });
        }

        if ui
            .checkbox(&mut bezier.draw_b_spline_polygon, "Draw B-Spline Polygon")
            .changed()
        {
            cqrs.execute(&SetBezierC2DrawBSplinePolygon {
                id: bezier.id,
                draw_b_spline_polygon: bezier.draw_b_spline_polygon,
            });
        }

        if ui
            .checkbox(&mut bezier.draw_bernstein_polygon, "Draw Bernstein Polygon")
            .changed()
        {
            cqrs.execute(&SetBezierC2DrawBernsteinPolygon {
                id: bezier.id,
                draw_bernstein_polygon: bezier.draw_bernstein_polygon,
            });
        }

        if ui
            .checkbox(&mut bezier.draw_bernstein_points, "Draw Bernstein Points")
            .changed()
        {
            cqrs.execute(&SetBezierC2DrawBernsteinPoints {
                id: bezier.id,
                draw_bernstein_points: bezier.draw_bernstein_points,
            });
        }

        Resize::default()
            .id_source("resize_bezier_c2_bernstein")
            .default_height(140.0)
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .id_source("scroll_bezier_c2_bernstein")
                    .show(ui, |ui| {
                        for i in 0..bezier.bernstein_points.len() {
                            let selected = bezier.selected_bernstein_point == Some(i);
                            if ui
                                .selectable_label(selected, &format!("Bernstein Point {}", i))
                                .clicked()
                            {
                                bezier.selected_bernstein_point =
                                    if selected { None } else { Some(i) };
                                cqrs.execute(&SetBezierC2SelectedBernsteinPoint {
                                    id: bezier.id,
                                    selected_bernstein_point: bezier.selected_bernstein_point,
                                });
                            }
                        }
                    })
            });

        if let Some(i) = bezier.selected_bernstein_point {
            let mut transformer_drags = vec![];

            ui.horizontal(|ui| {
                ui.label("X");
                transformer_drags.push(
                    DragValue::new(&mut bezier.bernstein_points[i].x)
                        .speed(0.01)
                        .ui(ui),
                );
                ui.label("Y");
                transformer_drags.push(
                    DragValue::new(&mut bezier.bernstein_points[i].y)
                        .speed(0.01)
                        .ui(ui),
                );
                ui.label("Z");
                transformer_drags.push(
                    DragValue::new(&mut bezier.bernstein_points[i].z)
                        .speed(0.01)
                        .ui(ui),
                );
            });

            if transformer_drags.iter().any(|f| f.changed()) {
                cqrs.execute(&MoveBezierC2SelectedBernsteinPoint {
                    bezier_id: bezier.id,
                    transformer: LittleTransformerDTO {
                        position: (
                            bezier.bernstein_points[i].x,
                            bezier.bernstein_points[i].y,
                            bezier.bernstein_points[i].z,
                        ),
                    },
                });
            }
        }
    }

    fn build_bezier_c0_transformation_panel(
        ui: &mut egui::Ui,
        cqrs: &mut CQRS,
        bezier: &mut BezierC0,
        points: &[PointDTO],
    ) {
        if ui.text_edit_singleline(&mut bezier.name).lost_focus() {
            cqrs.execute(&RenameBezierC0 {
                id: bezier.id,
                name: bezier.name.clone(),
            });
        }

        Resize::default()
            .id_source("resize_bezier_c0")
            .default_height(320.0)
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .id_source("scroll_bezier_c0")
                    .show(ui, |ui| {
                        for point in bezier.points.iter_mut() {
                            if ui
                                .selectable_label(point.is_selected, &point.name)
                                .clicked()
                            {
                                point.is_selected = !point.is_selected;
                            }
                        }
                    })
            });

        ui.horizontal(|ui| {
            ComboBox::from_id_source("Bezier c0 select point")
                .selected_text(if let Some(p) = &bezier.selected_point {
                    &p.1
                } else {
                    ""
                })
                .show_ui(ui, |ui| {
                    for point in points
                        .iter()
                        .filter(|p| !bezier.points.iter().any(|bp| bp.id == p.id))
                    {
                        if ui.selectable_label(false, &point.name).clicked() {
                            bezier.selected_point = Some((point.id, point.name.clone()));
                        }
                    }
                });
            if ui.button("Add Point").clicked() {
                if let Some((id, _name)) = &bezier.selected_point {
                    cqrs.execute(&AddPointToBezierC0 {
                        id: bezier.id,
                        point_id: *id,
                    });
                }
                bezier.selected_point = None;
            }
        });

        if ui.button("Delete Points").clicked() {
            cqrs.execute(&DeleteBezierC0Points {
                id: bezier.id,
                points: bezier
                    .points
                    .iter()
                    .filter(|p| p.is_selected)
                    .map(|p| p.id)
                    .collect(),
            });
        }

        if ui
            .checkbox(&mut bezier.draw_polygon, "Draw Polygon")
            .changed()
        {
            cqrs.execute(&SetBezierC0DrawPolygon {
                id: bezier.id,
                draw_polygon: bezier.draw_polygon,
            });
        }
    }

    fn build_bezier_int_transformation_panel(
        ui: &mut egui::Ui,
        cqrs: &mut CQRS,
        bezier: &mut BezierInt,
        points: &[PointDTO],
    ) {
        if ui.text_edit_singleline(&mut bezier.name).lost_focus() {
            cqrs.execute(&RenameBezierInt {
                id: bezier.id,
                name: bezier.name.clone(),
            });
        }

        Resize::default()
            .id_source("resize_bezier_int")
            .default_height(320.0)
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .id_source("scroll_bezier_int")
                    .show(ui, |ui| {
                        for point in bezier.points.iter_mut() {
                            if ui
                                .selectable_label(point.is_selected, &point.name)
                                .clicked()
                            {
                                point.is_selected = !point.is_selected;
                            }
                        }
                    })
            });

        ui.horizontal(|ui| {
            ComboBox::from_id_source("Bezier int select point")
                .selected_text(if let Some(p) = &bezier.selected_point {
                    &p.1
                } else {
                    ""
                })
                .show_ui(ui, |ui| {
                    for point in points
                        .iter()
                        .filter(|p| !bezier.points.iter().any(|bp| bp.id == p.id))
                    {
                        if ui.selectable_label(false, &point.name).clicked() {
                            bezier.selected_point = Some((point.id, point.name.clone()));
                        }
                    }
                });
            if ui.button("Add Point").clicked() {
                if let Some((id, _name)) = &bezier.selected_point {
                    cqrs.execute(&AddPointToBezierInt {
                        id: bezier.id,
                        point_id: *id,
                    });
                }
                bezier.selected_point = None;
            }
        });

        if ui.button("Delete Points").clicked() {
            cqrs.execute(&DeleteBezierIntPoints {
                id: bezier.id,
                points: bezier
                    .points
                    .iter()
                    .filter(|p| p.is_selected)
                    .map(|p| p.id)
                    .collect(),
            });
        }
    }

    fn build_surface_c0_transformation_panel(ui: &mut egui::Ui, cqrs: &mut CQRS, surface: &mut SurfaceC0DTO) {
        if ui.text_edit_singleline(&mut surface.name).lost_focus() {
            cqrs.execute(&RenameSurfaceC0 {
                id: surface.id,
                name: surface.name.clone(),
            });
        }
        
        if ui.button("Select points").clicked() { 
            cqrs.execute(&SelectSurfaceC0Points { surface_id: surface.id, });
        }
        
        ui.horizontal(|ui| {
            ui.label("Tessellation level");
            if DragValue::new(&mut surface.tess_level)
                .clamp_range(2..=64)
                .ui(ui).changed() {
                cqrs.execute(&UpdateSurfaceC0 {
                    id: surface.id,
                    tess_level: surface.tess_level,
                    draw_polygon: surface.draw_polygon,
                });
            }
        });
        
        if ui.checkbox(&mut surface.draw_polygon, "Draw Polygon").changed() {
            cqrs.execute(&UpdateSurfaceC0 {
                id: surface.id,
                tess_level: surface.tess_level,
                draw_polygon: surface.draw_polygon,
            });
        }
    }

    fn build_surface_c2_transformation_panel(ui: &mut egui::Ui, cqrs: &mut CQRS, surface: &mut SurfaceC2DTO) {
        if ui.text_edit_singleline(&mut surface.name).lost_focus() {
            cqrs.execute(&RenameSurfaceC2 {
                id: surface.id,
                name: surface.name.clone(),
            });
        }

        if ui.button("Select points").clicked() {
            cqrs.execute(&SelectSurfaceC2Points { surface_id: surface.id, });
        }

        ui.horizontal(|ui| {
            ui.label("Tessellation level");
            if DragValue::new(&mut surface.tess_level)
                .clamp_range(2..=64)
                .ui(ui).changed() {
                cqrs.execute(&UpdateSurfaceC2 {
                    id: surface.id,
                    tess_level: surface.tess_level,
                    draw_polygon: surface.draw_polygon,
                });
            }
        });

        if ui.checkbox(&mut surface.draw_polygon, "Draw Polygon").changed() {
            cqrs.execute(&UpdateSurfaceC2 {
                id: surface.id,
                tess_level: surface.tess_level,
                draw_polygon: surface.draw_polygon,
            });
        }
    }
    
    fn build_stereoscopy_settings_panel(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.stereoscopy, "Stereoscopy");
        Slider::new(&mut self.stereoscopy_eye_distance, 0.01..=10.0)
            .step_by(0.01)
            .text("Eye distance")
            .ui(ui);
    }
}
