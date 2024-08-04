use crate::data_access::storage::Storage;
use crate::domain::point::Point;
use crate::domain::transformer::LittleTransformer;
use crate::services::file_helpers::scene::Scene;

pub fn load_scene(storage: &mut Storage, file_path: &str) {
    let serialized = std::fs::read_to_string(file_path).unwrap();
    let scene: Scene = serde_json::from_str(&serialized).unwrap();
    storage.points.clear();
    for point in scene.points {
        storage.points.insert(point.id, Point::new_with_name(
            point.id,
            point.name,
            LittleTransformer {
                position: (point.position.x, point.position.y, point.position.z),
            }
        ));
    }
}
