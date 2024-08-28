use math::operations::euler_to_quaternion;

use crate::data_access::storage::Storage;
use crate::domain::bezier_c0::{BezierC0, BezierC0Point};
use crate::domain::bezier_c2::BezierC2;
use crate::domain::bezier_int::BezierInt;
use crate::domain::point::Point;
use crate::domain::surface_c0::{SurfaceC0, SurfaceC0Point};
use crate::domain::surface_c2::{SurfaceC2, SurfaceC2Point};
use crate::domain::torus::Torus;
use crate::domain::transformer::{LittleTransformer, Transformer};
use crate::services::file_helpers::geometry_obj::GeometryObj;
use crate::services::file_helpers::scene::Scene;

pub fn load_scene(storage: &mut Storage, file_path: &str) {
    let serialized = std::fs::read_to_string(file_path).unwrap();
    let scene: Scene = serde_json::from_str(&serialized).unwrap();
    storage.points.clear();
    storage.toruses.clear();
    storage.beziers_c0.clear();
    storage.beziers_c2.clear();
    storage.beziers_int.clear();
    storage.surfaces_c0.clear();
    storage.surfaces_c2.clear();
    storage.selected_objects.clear();
    for point in scene.points {
        storage.points.insert(
            point.id,
            Point::new_with_name(
                point.id,
                point.name,
                LittleTransformer {
                    position: (point.position.x, point.position.y, -point.position.z),
                },
            ),
        );
    }
    for torus in scene.geometry.iter().filter_map(|g| {
        if let GeometryObj::Torus(torus) = g {
            Some(torus)
        } else {
            None
        }
    }) {
        storage.toruses.insert(
            torus.id,
            Torus::new_with_name(
                torus.id,
                torus.name.clone(),
                torus.large_radius,
                torus.small_radius,
                torus.samples.y,
                torus.samples.x,
                Transformer {
                    position: (torus.position.x, torus.position.y, -torus.position.z),
                    rotation: euler_to_quaternion(
                        torus.rotation.x,
                        torus.rotation.y,
                        torus.rotation.z,
                    ),
                    scale: (torus.scale.x, torus.scale.y, torus.scale.z),
                },
            ),
        );
    }
    for bezier_c0 in scene.geometry.iter().filter_map(|g| {
        if let GeometryObj::BezierC0(bezier) = g {
            Some(bezier)
        } else {
            None
        }
    }) {
        let points = bezier_c0
            .control_points
            .iter()
            .map(|p| BezierC0Point { id: p.id })
            .collect();
        storage.beziers_c0.insert(
            bezier_c0.id,
            BezierC0::new_with_name(bezier_c0.id, bezier_c0.name.clone(), points),
        );
    }
    for bezier_c2 in scene.geometry.iter().filter_map(|g| {
        if let GeometryObj::BezierC2(bezier) = g {
            Some(bezier)
        } else {
            None
        }
    }) {
        let points = bezier_c2
            .de_boor_points
            .iter()
            .map(|p| storage.points.get(&p.id).unwrap().clone())
            .collect();
        storage.beziers_c2.insert(
            bezier_c2.id,
            BezierC2::new_with_name(bezier_c2.id, bezier_c2.name.clone(), points),
        );
    }
    for bezier_int in scene.geometry.iter().filter_map(|g| {
        if let GeometryObj::InterpolatedC2(bezier) = g {
            Some(bezier)
        } else {
            None
        }
    }) {
        let points = bezier_int
            .control_points
            .iter()
            .map(|p| storage.points.get(&p.id).unwrap().clone())
            .collect();
        storage.beziers_int.insert(
            bezier_int.id,
            BezierInt::new_with_name(bezier_int.id, bezier_int.name.clone(), points),
        );
    }
    for surface_c0 in scene.geometry.iter().filter_map(|g| {
        if let GeometryObj::BezierSurfaceC0(surface) = g {
            Some(surface)
        } else {
            None
        }
    }) {
        let mut points = vec![];

        for i in 0..surface_c0.size.x {
            for j in if i == surface_c0.size.x - 1 {
                0..4
            } else {
                0..3
            } {
                for k in 0..surface_c0.size.y {
                    for g in if k == surface_c0.size.y - 1 {
                        0..4
                    } else {
                        0..3
                    } {
                        let patch = &surface_c0.patches[(i + k * surface_c0.size.x) as usize];
                        let control_point = &patch.control_points[(j + g * 4) as usize];
                        points.push(SurfaceC0Point {
                            id: control_point.id,
                        });
                    }
                }
            }
        }

        storage.surfaces_c0.insert(
            surface_c0.id,
            SurfaceC0::new_with_name(
                surface_c0.id,
                surface_c0.name.clone(),
                points,
                (surface_c0.size.x, surface_c0.size.y),
            ),
        );
    }
    for surface_c2 in scene.geometry.iter().filter_map(|g| {
        if let GeometryObj::BezierSurfaceC2(surface) = g {
            Some(surface)
        } else {
            None
        }
    }) {
        let mut points = vec![];

        for i in 0..surface_c2.size.x {
            for j in if i == surface_c2.size.x - 1 {
                0..4
            } else {
                0..1
            } {
                for k in 0..surface_c2.size.y {
                    for g in if k == surface_c2.size.y - 1 {
                        0..4
                    } else {
                        0..1
                    } {
                        let patch = &surface_c2.patches[(i + k * surface_c2.size.x) as usize];
                        let control_point = &patch.control_points[(j + g * 4) as usize];
                        points.push(SurfaceC2Point {
                            id: control_point.id,
                        });
                    }
                }
            }
        }

        storage.surfaces_c2.insert(
            surface_c2.id,
            SurfaceC2::new_with_name(
                surface_c2.id,
                surface_c2.name.clone(),
                points,
                (surface_c2.size.x, surface_c2.size.y),
            ),
        );
    }
}
