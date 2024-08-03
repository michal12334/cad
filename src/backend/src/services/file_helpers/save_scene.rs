use crate::data_access::storage::Storage;
use crate::services::file_helpers::point::Point;
use crate::services::file_helpers::scene::Scene;
use crate::services::file_helpers::xyz::Xyz;

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
    };
    let serialized = serde_json::to_string_pretty(&scene).unwrap();
    std::fs::write(file_path, serialized).unwrap();
}
