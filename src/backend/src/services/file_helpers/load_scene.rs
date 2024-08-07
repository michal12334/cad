use crate::data_access::storage::Storage;
use crate::domain::point::Point;
use crate::domain::torus::Torus;
use crate::domain::transformer::{LittleTransformer, Transformer};
use crate::services::file_helpers::geometry_obj::GeometryObj;
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
    for torus in scene.geometry.iter().filter_map(|g| if let GeometryObj::Torus(torus) = g { Some(torus) } else { None } ) {
        storage.toruses.insert(torus.id, Torus::new_with_name(
            torus.id,
            torus.name.clone(),
            torus.large_radius,
            torus.small_radius,
            torus.samples.y,
            torus.samples.x,
            Transformer {
                position: (torus.position.x, torus.position.y, torus.position.z),
                rotation: (torus.rotation.x, torus.rotation.y, torus.rotation.z, 1f64),
                scale: (torus.scale.x, torus.scale.y, torus.scale.z),
            },
        ));
    }
}
