use std::cell::RefCell;
use std::rc::Rc;

use math::operations::quaternion_to_euler;

use crate::data_access::storage::Storage;
use crate::services::file_helpers::bezier_c0::{BezierC0, BezierC0Point};
use crate::services::file_helpers::bezier_c2::{BezierC2, BezierC2Point};
use crate::services::file_helpers::bezier_int::{BezierInt, BezierIntPoint};
use crate::services::file_helpers::geometry_obj::GeometryObj;
use crate::services::file_helpers::point::Point;
use crate::services::file_helpers::scene::Scene;
use crate::services::file_helpers::surface_c0::{SurfaceC0, SurfaceC0ControlPoint, SurfaceC0Patch};
use crate::services::file_helpers::surface_c2::{SurfaceC2, SurfaceC2ControlPoint, SurfaceC2Patch};
use crate::services::file_helpers::torus::Torus;
use crate::services::file_helpers::xyz::{Xyu32, Xyz};
use crate::services::id_generator::IdGenerator;

pub fn save_scene(storage: &Storage, file_path: &str, id_generator: &mut IdGenerator) {
    let id_generator = Rc::new(RefCell::new(id_generator));
    let scene = Scene {
        points: storage
            .points
            .values()
            .map(|p| Point {
                id: p.id,
                name: p.name.clone(),
                position: Xyz {
                    x: p.transformer.position.0,
                    y: p.transformer.position.1,
                    z: -p.transformer.position.2,
                },
            })
            .collect(),
        geometry: storage
            .toruses
            .values()
            .map(|t| {
                GeometryObj::Torus(Torus {
                    id: t.id,
                    name: t.name.clone(),
                    position: Xyz {
                        x: t.transformer.position.0,
                        y: t.transformer.position.1,
                        z: -t.transformer.position.2,
                    },
                    rotation: Xyz::from_tuple(quaternion_to_euler(t.transformer.rotation)),
                    scale: Xyz {
                        x: t.transformer.scale.0,
                        y: t.transformer.scale.1,
                        z: t.transformer.scale.2,
                    },
                    small_radius: t.minor_radius,
                    large_radius: t.major_radius,
                    samples: Xyu32 {
                        x: t.minor_segments,
                        y: t.major_segments,
                    },
                })
            })
            .chain(storage.beziers_c0.values().map(|b| {
                GeometryObj::BezierC0(BezierC0 {
                    id: b.id,
                    name: b.name.clone(),
                    control_points: b
                        .points
                        .iter()
                        .map(|p| BezierC0Point { id: p.id })
                        .collect(),
                })
            }))
            .chain(storage.beziers_c2.values().map(|b| {
                GeometryObj::BezierC2(BezierC2 {
                    id: b.id,
                    name: b.name.clone(),
                    de_boor_points: b
                        .b_spline_points
                        .iter()
                        .map(|p| BezierC2Point { id: p.id })
                        .collect(),
                })
            }))
            .chain(storage.beziers_int.values().map(|b| {
                GeometryObj::InterpolatedC2(BezierInt {
                    id: b.id,
                    name: b.name.clone(),
                    control_points: b
                        .points
                        .iter()
                        .map(|p| BezierIntPoint { id: p.id })
                        .collect(),
                })
            }))
            .chain(storage.surfaces_c0.values().map(|s| {
                GeometryObj::BezierSurfaceC0(SurfaceC0 {
                    id: s.id,
                    name: s.name.clone(),
                    size: Xyu32 {
                        x: s.size.0,
                        y: s.size.1,
                    },
                    patches: (0..s.size.1)
                        .flat_map(|y| (0..s.size.0).map(move |x| (x, y)))
                        .map(|(x, y)| {
                            [
                                (3 * x, 3 * y),
                                (3 * x + 1, 3 * y),
                                (3 * x + 2, 3 * y),
                                (3 * x + 3, 3 * y),
                                (3 * x, 3 * y + 1),
                                (3 * x + 1, 3 * y + 1),
                                (3 * x + 2, 3 * y + 1),
                                (3 * x + 3, 3 * y + 1),
                                (3 * x, 3 * y + 2),
                                (3 * x + 1, 3 * y + 2),
                                (3 * x + 2, 3 * y + 2),
                                (3 * x + 3, 3 * y + 2),
                                (3 * x, 3 * y + 3),
                                (3 * x + 1, 3 * y + 3),
                                (3 * x + 2, 3 * y + 3),
                                (3 * x + 3, 3 * y + 3),
                            ]
                        })
                        .map(|p| SurfaceC0Patch {
                            id: id_generator.borrow_mut().next(),
                            name: "".to_string(),
                            object_type: "bezierPatchC0".to_string(),
                            control_points: p
                                .iter()
                                .map(|(x, y)| SurfaceC0ControlPoint {
                                    id: s.points[(x * (s.size.1 * 3 + 1) + y) as usize].id,
                                })
                                .collect(),
                            samples: Xyu32 { x: 4, y: 4 },
                        })
                        .collect::<Vec<_>>(),
                })
            }))
            .chain(storage.surfaces_c2.values().map(|s| {
                GeometryObj::BezierSurfaceC2(SurfaceC2 {
                    id: s.id,
                    name: s.name.clone(),
                    size: Xyu32 {
                        x: s.size.0,
                        y: s.size.1,
                    },
                    patches: (0..s.size.1)
                        .flat_map(|y| (0..s.size.0).map(move |x| (x, y)))
                        .map(|(x, y)| {
                            [
                                (x, y),
                                (x + 1, y),
                                (x + 2, y),
                                (x + 3, y),
                                (x, y + 1),
                                (x + 1, y + 1),
                                (x + 2, y + 1),
                                (x + 3, y + 1),
                                (x, y + 2),
                                (x + 1, y + 2),
                                (x + 2, y + 2),
                                (x + 3, y + 2),
                                (x, y + 3),
                                (x + 1, y + 3),
                                (x + 2, y + 3),
                                (x + 3, y + 3),
                            ]
                        })
                        .map(|p| SurfaceC2Patch {
                            id: id_generator.borrow_mut().next(),
                            name: "".to_string(),
                            object_type: "bezierPatchC2".to_string(),
                            control_points: p
                                .iter()
                                .map(|(x, y)| SurfaceC2ControlPoint {
                                    id: s.points[(x * (s.size.1 + 3) + y) as usize].id,
                                })
                                .collect(),
                            samples: Xyu32 { x: 4, y: 4 },
                        })
                        .collect::<Vec<_>>(),
                })
            }))
            .collect(),
    };
    let serialized = serde_json::to_string_pretty(&scene).unwrap();
    std::fs::write(file_path, serialized).unwrap();
}
