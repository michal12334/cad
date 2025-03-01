use std::{cell::RefCell, rc::Rc};

use backend_events::points::selected_points_merged::SelectedPointsMerged;

use crate::{
    backend::Backend,
    cqrs::{
        common::selected_objects_center::SelectedObjectsCenter,
        cqrs::{Command, CQRS},
    },
    domain::{point::Point, transformer::LittleTransformer},
};

pub struct MergeSelectedPoints;

impl Command<MergeSelectedPoints> for MergeSelectedPoints {
    fn execute(_: &MergeSelectedPoints, app_state: Rc<RefCell<Backend>>) {
        let backend = app_state.borrow();

        if backend.storage.selected_objects.len() < 2
            || backend
                .storage
                .selected_objects
                .iter()
                .any(|o| o.point_id.is_none())
        {
            return;
        }

        let cqrs = CQRS::new(app_state.clone());
        let center_point = cqrs.get(&SelectedObjectsCenter).unwrap();

        drop(cqrs);
        drop(backend);

        let mut backend = app_state.borrow_mut();

        let point = Point::new(
            backend.services.id_generator.next(),
            LittleTransformer {
                position: center_point.position,
            },
        );

        let point_id = point.id;

        backend.storage.points.insert(point.id, point);

        let points = backend.storage.points.clone();

        for p_id in backend
            .storage
            .selected_objects
            .iter()
            .map(|o| o.point_id.unwrap())
            .collect::<Vec<_>>()
        {
            backend.storage.points.remove(&p_id);

            backend
                .storage
                .beziers_c0
                .iter_mut()
                .filter(|(_, b)| b.points.iter().any(|p| p.id == p_id))
                .for_each(|(_, b)| {
                    b.replace_point(p_id, point_id);
                });

            backend
                .storage
                .beziers_c2
                .iter_mut()
                .filter(|(_, b)| b.b_spline_points.iter().any(|p| p.id == p_id))
                .for_each(|(_, b)| {
                    b.replace_point(p_id, point_id, &points);
                });

            backend
                .storage
                .beziers_int
                .iter_mut()
                .filter(|(_, b)| b.points.iter().any(|p| p.id == p_id))
                .for_each(|(_, b)| {
                    b.replace_point(p_id, point_id, &points);
                });

            backend
                .storage
                .surfaces_c0
                .iter_mut()
                .filter(|(_, s)| s.points.iter().any(|p| p.id == p_id))
                .for_each(|(_, s)| {
                    s.replace_point(p_id, point_id);
                });

            backend
                .storage
                .surfaces_c2
                .iter_mut()
                .filter(|(_, s)| s.points.iter().any(|p| p.id == p_id))
                .for_each(|(_, s)| {
                    s.replace_point(p_id, point_id);
                });
        }

        backend.storage.selected_objects.clear();

        drop(backend);

        app_state
            .borrow()
            .services
            .event_publisher
            .publish(Rc::new(SelectedPointsMerged));
    }
}
