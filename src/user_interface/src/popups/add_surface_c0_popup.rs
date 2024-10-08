use egui::{Context, Widget};
use itertools::Itertools;

use backend::cqrs::common::new_id::NewId;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::surfaces_c0::create_surface_c0::{CreateSurfaceC0, CreateSurfaceInfoDTO};
use backend::cqrs::surfaces_c0::surface_c0_details::SurfaceC0Details;
use backend::cqrs::surfaces_c0::surface_c0_points::SurfaceC0Points;

use crate::object::Object;
use crate::popups::popup::Popup;

pub struct AddSurfaceC0Popup {
    is_closed: bool,
    pub is_cylinder: bool,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub radius: f64,
    pub size: (u32, u32),
}

impl AddSurfaceC0Popup {
    pub fn new() -> Self {
        Self {
            is_closed: false,
            is_cylinder: false,
            length: 1.0,
            width: 1.0,
            height: 1.0,
            radius: 1.0,
            size: (1, 1),
        }
    }
}

impl Popup for AddSurfaceC0Popup {
    fn build(&mut self, cqrs: &mut CQRS, context: &Context) -> Vec<Object> {
        let mut result = vec![];

        egui::Window::new("Add Surface C0").show(context, |ui| {
            ui.checkbox(&mut self.is_cylinder, "Cylinder");

            if self.is_cylinder {
                ui.horizontal(|ui| {
                    egui::DragValue::new(&mut self.radius)
                        .clamp_range(0.1..=100.0)
                        .speed(0.1)
                        .ui(ui);
                    ui.label("Radius");
                });
                ui.horizontal(|ui| {
                    egui::DragValue::new(&mut self.height)
                        .clamp_range(0.1..=100.0)
                        .speed(0.1)
                        .ui(ui);
                    ui.label("Height");
                });
            } else {
                ui.horizontal(|ui| {
                    egui::DragValue::new(&mut self.length)
                        .clamp_range(0.1..=100.0)
                        .speed(0.1)
                        .ui(ui);
                    ui.label("Length");
                });
                ui.horizontal(|ui| {
                    egui::DragValue::new(&mut self.width)
                        .clamp_range(0.1..=100.0)
                        .speed(0.1)
                        .ui(ui);
                    ui.label("Width");
                });
            }

            ui.horizontal(|ui| {
                egui::DragValue::new(&mut self.size.0)
                    .clamp_range(1..=100)
                    .ui(ui);
                ui.label("Size X");
            });

            ui.horizontal(|ui| {
                egui::DragValue::new(&mut self.size.1)
                    .clamp_range(1..=100)
                    .ui(ui);
                ui.label("Size Y");
            });

            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    let id = cqrs.handle(&NewId {});
                    cqrs.execute(&CreateSurfaceC0 {
                        id,
                        create_surface_info: CreateSurfaceInfoDTO {
                            is_cylinder: self.is_cylinder,
                            length: Some(self.length),
                            width: Some(self.width),
                            height: Some(self.height),
                            radius: Some(self.radius),
                            size: self.size,
                        },
                    });

                    let surface = cqrs.get(&SurfaceC0Details { id });
                    let points = cqrs.get(&SurfaceC0Points { id });

                    result.push(Object::SurfaceC0(surface));
                    for point in points.into_iter().unique_by(|p| p.id) {
                        result.push(Object::Point(point));
                    }

                    self.is_closed = true;
                }
                if ui.button("Close").clicked() {
                    self.is_closed = true;
                }
            });
        });

        return result;
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }
}
