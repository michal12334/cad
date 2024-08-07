use math::operations::quaternion_to_euler;
use crate::data_access::storage::Storage;
use crate::services::file_helpers::geometry_obj::GeometryObj;
use crate::services::file_helpers::point::Point;
use crate::services::file_helpers::scene::Scene;
use crate::services::file_helpers::torus::Torus;
use crate::services::file_helpers::xyz::{Xyu32, Xyz};

pub fn save_scene(storage: &Storage, file_path: &str) {
    let scene = Scene {
        points: storage.points
            .values()
            .map(|p| Point {
                id: p.id,
                name: p.name.clone(),
                position: Xyz {
                    x: p.transformer.position.0,
                    y: p.transformer.position.1,
                    z: p.transformer.position.2,
                }
            })
            .collect(),
        geometry: storage.toruses
            .values()
            .map(|t| GeometryObj::Torus(Torus {
                id: t.id,
                name: t.name.clone(),
                position: Xyz {
                    x: t.transformer.position.0,
                    y: t.transformer.position.1,
                    z: t.transformer.position.2,
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
            }))
            .collect(),
    };
    let serialized = serde_json::to_string_pretty(&scene).unwrap();
    std::fs::write(file_path, serialized).unwrap();
}
