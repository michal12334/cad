use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use math::matrix4::Matrix4;
use math::operations::multiply_quaternions;
use math::vector4::Vector4;

use crate::backend::Backend;
use crate::cqrs::common::selected_objects_center::SelectedObjectsCenter;
use crate::cqrs::cqrs::{Command, CQRS};
use crate::cqrs::toruses::torus_details::TransformerDTO;
use crate::domain::events::points::point_moved::PointMoved;
use crate::domain::events::toruses::torus_transformed::TorusTransformed;

pub struct TransformSelectedObjects {
    pub transformer: TransformerDTO,
}

impl Command<TransformSelectedObjects> for TransformSelectedObjects {
    fn execute(command: &TransformSelectedObjects, app_state: Rc<RefCell<Backend>>) {
        let cqrs = CQRS::new(app_state.clone());
        let center_point = cqrs.get(&SelectedObjectsCenter).unwrap();
        let mut binding = app_state.borrow_mut();
        let backend = binding.deref_mut();
        let mut point_moved_events = vec![];
        let mut torus_transformed_events = vec![];
        for object in backend.storage.selected_objects.iter() {
            if let Some(torus_id) = object.torus_id {
                let torus = backend.storage.toruses.get_mut(&torus_id).unwrap();
                let position = (
                    torus.transformer.position.0 + command.transformer.position.0,
                    torus.transformer.position.1 + command.transformer.position.1,
                    torus.transformer.position.2 + command.transformer.position.2,
                );
                let scale = (
                    torus.transformer.scale.0 * command.transformer.scale.0,
                    torus.transformer.scale.1 * command.transformer.scale.1,
                    torus.transformer.scale.2 * command.transformer.scale.2,
                );
                let delta_position = (
                    position.0 - center_point.position.0,
                    position.1 - center_point.position.1,
                    position.2 - center_point.position.2,
                );
                let rotation_matrix = Matrix4::rotation_quaternion(Vector4::new(
                    command.transformer.rotation.0 as f32,
                    command.transformer.rotation.1 as f32,
                    command.transformer.rotation.2 as f32,
                    command.transformer.rotation.3 as f32,
                ));
                let delta_position = rotation_matrix
                    * Vector4::new(
                        delta_position.0 as f32,
                        delta_position.1 as f32,
                        delta_position.2 as f32,
                        0.0,
                    );
                let delta_position = (
                    delta_position.x as f64,
                    delta_position.y as f64,
                    delta_position.z as f64,
                );
                let position = (
                    center_point.position.0 + delta_position.0 * command.transformer.scale.0,
                    center_point.position.1 + delta_position.1 * command.transformer.scale.1,
                    center_point.position.2 + delta_position.2 * command.transformer.scale.2,
                );
                let rotation =
                    multiply_quaternions(command.transformer.rotation, torus.transformer.rotation);
                torus.transform(position, rotation, scale);
                torus_transformed_events.push(Rc::new(TorusTransformed::new(
                    torus.id,
                    torus.transformer.position,
                    torus.transformer.rotation,
                    torus.transformer.scale,
                )));
            }
            if let Some(point_id) = object.point_id {
                let point = backend.storage.points.get_mut(&point_id).unwrap();
                let position = (
                    point.transformer.position.0 + command.transformer.position.0,
                    point.transformer.position.1 + command.transformer.position.1,
                    point.transformer.position.2 + command.transformer.position.2,
                );
                let delta_position = (
                    position.0 - center_point.position.0,
                    position.1 - center_point.position.1,
                    position.2 - center_point.position.2,
                );
                let rotation_matrix = Matrix4::rotation_quaternion(Vector4::new(
                    command.transformer.rotation.0 as f32,
                    command.transformer.rotation.1 as f32,
                    command.transformer.rotation.2 as f32,
                    command.transformer.rotation.3 as f32,
                ));
                let delta_position = rotation_matrix
                    * Vector4::new(
                        delta_position.0 as f32,
                        delta_position.1 as f32,
                        delta_position.2 as f32,
                        0.0,
                    );
                let delta_position = (
                    delta_position.x as f64,
                    delta_position.y as f64,
                    delta_position.z as f64,
                );
                let position = (
                    center_point.position.0 + delta_position.0 * command.transformer.scale.0,
                    center_point.position.1 + delta_position.1 * command.transformer.scale.1,
                    center_point.position.2 + delta_position.2 * command.transformer.scale.2,
                );
                point.transform(position);
                point_moved_events.push(Rc::new(PointMoved::new(
                    point.id,
                    point.transformer.position,
                )));
            }
        }
        drop(binding);
        let backend = app_state.borrow();
        for e in point_moved_events {
            backend.services.event_publisher.publish(e);
        }
        for e in torus_transformed_events {
            backend.services.event_publisher.publish(e);
        }
    }
}
