use backend::cqrs::gregories::calculate_gregories::CalculateGregories;
use backend::cqrs::points::merge_selected_points::MergeSelectedPoints;
use rfd::FileDialog;

use backend::cqrs::beziers_c0::add_bezier_c0::AddBezierC0;
use backend::cqrs::beziers_c0::bezier_c0_details::BezierC0Details;
use backend::cqrs::beziers_c2::add_bezier_c2::AddBezierC2;
use backend::cqrs::beziers_c2::bezier_c2_details::BezierC2Details;
use backend::cqrs::beziers_int::add_bezier_int::AddBezierInt;
use backend::cqrs::common::load_scene::LoadScene;
use backend::cqrs::common::new_id::NewId;
use backend::cqrs::common::save_scene::SaveScene;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::points::add_point::AddPoint;
use backend::cqrs::toruses::add_torus::AddTorus;
use backend::cqrs::toruses::torus_details::TorusDetails;

use crate::object::Object;
use crate::object::Object::{BezierC0, Torus};
use crate::popups::add_surface_c0_popup::AddSurfaceC0Popup;
use crate::popups::add_surface_c2_popup::AddSurfaceC2Popup;
use crate::popups::find_intersection_popup::FindIntersectionPopup;
use crate::popups::find_self_intersection_popup::FindSelfIntersectionPopup;
use crate::ui::Ui;

type DomainBezierC0 = crate::domain::bezier_c0::BezierC0;
type DomainBezierC2 = crate::domain::bezier_c2::BezierC2;

impl Ui {
    pub fn build_object_addition_panel(&mut self, ui: &mut egui::Ui, cqrs: &mut CQRS) {
        ui.horizontal(|ui| {
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
            }
            if ui.button("Merge Points").clicked() {
                cqrs.execute(&MergeSelectedPoints);
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Add Bezier C0").clicked() {
                let id = cqrs.handle(&NewId {});
                cqrs.execute(&AddBezierC0 { id });
                self.objects.push(BezierC0(DomainBezierC0::from_dto(
                    &cqrs.get(&BezierC0Details { id }),
                )));
            }
            if ui.button("Add Bezier C2").clicked() {
                let id = cqrs.handle(&NewId {});
                cqrs.execute(&AddBezierC2 { id });
                self.objects.push(Object::BezierC2(DomainBezierC2::from_dto(
                    &cqrs.get(&BezierC2Details { id }),
                )));
            }
            if ui.button("Add Bezier Int").clicked() {
                let id = cqrs.handle(&NewId {});
                cqrs.execute(&AddBezierInt { id });
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Add Surface C0").clicked() {
                self.popup = Some(Box::new(AddSurfaceC0Popup::new()));
            }
            if ui.button("Add Surface C2").clicked() {
                self.popup = Some(Box::new(AddSurfaceC2Popup::new()));
            }
            if ui.button("Add Gregory").clicked() {
                cqrs.execute(&CalculateGregories);
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                let path = FileDialog::new().save_file();
                if let Some(path) = path {
                    cqrs.execute(&SaveScene {
                        file_path: path.to_str().unwrap().to_string(),
                    });
                }
            }
            if ui.button("Load").clicked() {
                let path = FileDialog::new().pick_file();
                if let Some(path) = path {
                    cqrs.execute(&LoadScene {
                        file_path: path.to_str().unwrap().to_string(),
                    });
                }
            }
            if ui.button("Find Intesection").clicked() {
                let ids = self
                    .selected_objects
                    .iter()
                    .filter_map(|x| x.get_intersection_object_id())
                    .take(2)
                    .collect::<Vec<_>>();
                if ids.len() == 2 {
                    self.popup = Some(Box::new(FindIntersectionPopup::new([
                        ids[0].clone(),
                        ids[1].clone(),
                    ])));
                } else if ids.len() == 1 {
                    self.popup = Some(Box::new(FindSelfIntersectionPopup::new(ids[0].clone())));
                }
            }
        });
    }
}
