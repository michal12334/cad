use backend::cqrs::{
    common::new_id::NewId,
    cqrs::CQRS,
    intersections::{
        find_intersection::IntersectionObjectIdDTO, find_self_intersection::FindSelfIntersection,
    },
};
use egui::{Context, Widget};

use crate::object::Object;

use super::popup::Popup;

pub struct FindSelfIntersectionPopup {
    is_closed: bool,
    id: IntersectionObjectIdDTO,
    texture_size: usize,
    newton_factor: f32,
    rough: bool,
    max_distance: f32,
}

impl FindSelfIntersectionPopup {
    pub fn new(id: IntersectionObjectIdDTO) -> Self {
        Self {
            is_closed: false,
            id,
            texture_size: 200,
            newton_factor: 0.2,
            rough: false,
            max_distance: 0.0000001,
        }
    }
}

impl Popup for FindSelfIntersectionPopup {
    fn build(&mut self, cqrs: &mut CQRS, context: &Context) -> Vec<Object> {
        egui::Window::new("Find Intersection").show(context, |ui| {
            ui.horizontal(|ui| {
                egui::DragValue::new(&mut self.texture_size)
                    .clamp_range(100..=10000)
                    .ui(ui);
                ui.label("texture size");
            });

            ui.horizontal(|ui| {
                egui::DragValue::new(&mut self.newton_factor)
                    .clamp_range(0.01..=1.0)
                    .speed(0.01)
                    .ui(ui);
                ui.label("newton factor");
            });

            ui.horizontal(|ui| {
                egui::DragValue::new(&mut self.max_distance)
                    .clamp_range(0.0000001..=0.1)
                    .speed(0.0000001)
                    .ui(ui);
                ui.label("max distance");
            });

            ui.checkbox(&mut self.rough, "Rough");

            ui.horizontal(|ui| {
                if ui.button("Find").clicked() {
                    let id = cqrs.handle(&NewId {});
                    cqrs.execute(&FindSelfIntersection {
                        id: self.id,
                        intersection_id: id,
                        texture_size: self.texture_size,
                        newton_factor: self.newton_factor,
                        rough: self.rough,
                        max_distance: self.max_distance,
                    });

                    self.is_closed = true;
                }
                if ui.button("Close").clicked() {
                    self.is_closed = true;
                }
            });
        });

        vec![]
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }
}
