use math::matrix4::Matrix4;
use math::operations::multiply_quaternions;
use math::vector4::Vector4;
use crate::app_state::AppState;
use crate::cqrs::common::new_id::NewId;
use crate::cqrs::common::selected_objects_center::SelectedObjectsCenter;
use crate::cqrs::cqrs::{Command, CQRS};
use crate::cqrs::toruses::torus_details::TransformerDTO;

pub struct TransformSelectedObjects {
    pub transformer: TransformerDTO,
}

impl Command<TransformSelectedObjects> for TransformSelectedObjects {
    fn execute(command: &TransformSelectedObjects, app_state: &mut AppState) {
        let cqrs = CQRS::new(app_state);
        let center_point = cqrs.get(&SelectedObjectsCenter).unwrap();
        for object in app_state.storage.selected_objects.iter() {
            if let Some(torus_id) = object.torus_id {
                let torus = app_state.storage.toruses.get_mut(&torus_id).unwrap();
                let position = (torus.transformer.position.0 + command.transformer.position.0,
                                torus.transformer.position.1 + command.transformer.position.1,
                                torus.transformer.position.2 + command.transformer.position.2);
                let scale = (torus.transformer.scale.0 * command.transformer.scale.0,
                             torus.transformer.scale.1 * command.transformer.scale.1,
                             torus.transformer.scale.2 * command.transformer.scale.2);
                let delta_position = (position.0 - center_point.position.0,
                                      position.1 - center_point.position.1,
                                      position.2 - center_point.position.2);
                let rotation_matrix = Matrix4::rotation_quaternion(Vector4::new(command.transformer.rotation.0 as f32,
                                                                               command.transformer.rotation.1 as f32,
                                                                               command.transformer.rotation.2 as f32,
                                                                               command.transformer.rotation.3 as f32));
                let delta_position = Vector4::new(delta_position.0 as f32,
                                                                    delta_position.1 as f32,
                                                                    delta_position.2 as f32,
                                                                    0.0) * rotation_matrix;
                let delta_position = (delta_position.x as f64,
                                      delta_position.y as f64,
                                      delta_position.z as f64);
                let position = (center_point.position.0 + delta_position.0 * command.transformer.scale.0,
                                center_point.position.1 + delta_position.1 * command.transformer.scale.1,
                                center_point.position.2 + delta_position.2 * command.transformer.scale.2);
                let rotation = multiply_quaternions(torus.transformer.rotation, command.transformer.rotation);
                torus.transform(
                    position,
                    rotation,
                    scale,
                );
            }
            if let Some(point_id) = object.point_id {
                let point = app_state.storage.points.get_mut(&point_id).unwrap();
                let position = (point.transformer.position.0 + command.transformer.position.0,
                                point.transformer.position.1 + command.transformer.position.1,
                                point.transformer.position.2 + command.transformer.position.2);
                let delta_position = (position.0 - center_point.position.0,
                                      position.1 - center_point.position.1,
                                      position.2 - center_point.position.2);
                let rotation_matrix = Matrix4::rotation_quaternion(Vector4::new(command.transformer.rotation.0 as f32,
                                                                                command.transformer.rotation.1 as f32,
                                                                                command.transformer.rotation.2 as f32,
                                                                                command.transformer.rotation.3 as f32));
                let delta_position = Vector4::new(delta_position.0 as f32,
                                                  delta_position.1 as f32,
                                                  delta_position.2 as f32,
                                                  0.0) * rotation_matrix;
                let delta_position = (delta_position.x as f64,
                                      delta_position.y as f64,
                                      delta_position.z as f64);
                let position = (center_point.position.0 + delta_position.0 * command.transformer.scale.0,
                                center_point.position.1 + delta_position.1 * command.transformer.scale.1,
                                center_point.position.2 + delta_position.2 * command.transformer.scale.2);
                point.transform(
                    position,
                );
            }
        }
    }
}
