use backend::cqrs::beziers_c0::all_beziers_c0::AllBeziersC0;
use backend::cqrs::beziers_c2::all_beziers_c2::AllBeziersC2;
use backend::cqrs::beziers_int::all_beziers_int::AllBeziersInt;
use chrono::{DateTime, Local};
use egui::ScrollArea;
use itertools::Itertools;

use backend::cqrs::common::select_objects::{SelectObjects, SelectionObjectDTO};
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::cursors::cursor_details::CursorDTO;
use backend::cqrs::cursors::cursor_details::CursorDetails;
use backend::cqrs::points::all_points::AllPoints;
use backend::cqrs::toruses::all_toruses::AllToruses;
use backend::cqrs::toruses::torus_details::TransformerDTO;

type DomainBezierC0 = crate::domain::bezier_c0::BezierC0;
type DomainBezierC2 = crate::domain::bezier_c2::BezierC2;
type DomainBezierInt = crate::domain::bezier_int::BezierInt;

use crate::object::Object;
use crate::object::Object::{BezierC0, BezierC2, BezierInt, Point, Torus};
use crate::object_id::ObjectId;
use crate::popups::popup::Popup;

pub struct Ui {
    pub objects: Vec<Object>,
    pub selected_objects: Vec<ObjectId>,
    pub cursor: Option<CursorDTO>,
    pub cursor_selected: bool,
    pub pointer_is_over_area: bool,
    pub control_pressed: bool,
    pub group_transformation: Option<TransformerDTO>,
    pub previous_group_transformation: Option<TransformerDTO>,
    pub previous_time: DateTime<Local>,
    pub popup: Option<Box<dyn Popup>>,
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
            previous_time: Local::now(),
            popup: None,
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
            .chain(
                cqrs.get(&AllBeziersC0)
                    .iter()
                    .map(|bezier| BezierC0(DomainBezierC0::from_dto(bezier))),
            )
            .chain(
                cqrs.get(&AllBeziersC2)
                    .iter()
                    .map(|bezier| BezierC2(DomainBezierC2::from_dto(bezier))),
            )
            .chain(
                cqrs.get(&AllBeziersInt)
                    .iter()
                    .map(|bezier| BezierInt(DomainBezierInt::from_dto(bezier))),
            )
            .sorted_by_key(|object| object.get_id())
            .collect();
        self.selected_objects.clear();
    }

    pub fn change_point_selection(&mut self, id: u64, cqrs: &mut CQRS) {
        let is_selected = self.selected_objects.iter().any(|so| so.get_id() == id);
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

    pub fn build<'a>(&'a mut self, cqrs: &'a mut CQRS) -> impl FnMut(&egui::Context) + '_ {
        self.cursor = Some(cqrs.get(&CursorDetails {}));
        move |egui_ctx| {
            egui::Window::new("panel")
                .default_height(850.0)
                .show(egui_ctx, |ui| {
                    if egui_ctx.is_pointer_over_area() {
                        self.pointer_is_over_area = true;
                    } else {
                        self.pointer_is_over_area = false;
                    }

                    ScrollArea::vertical().id_source("a3").show(ui, |ui| {
                        self.build_object_addition_panel(ui, cqrs);
                        self.build_object_selection_panel(ui, cqrs);
                        self.build_selected_object_transformation_panel(ui, cqrs);
                        self.build_fps_counter(ui);
                    });
                });
            
            if let Some(popup) = &mut self.popup {
                popup.build(cqrs, egui_ctx);
                if popup.is_closed() {
                    self.popup = None;
                }
            }
        }
    }

    fn build_fps_counter(&mut self, ui: &mut egui::Ui) {
        let current_time = Local::now();
        let duration = current_time - self.previous_time;
        let fps = 1000.0 / duration.num_milliseconds() as f64;
        self.previous_time = current_time;
        ui.label(format!("FPS: {:.1}", fps,));
    }
}
