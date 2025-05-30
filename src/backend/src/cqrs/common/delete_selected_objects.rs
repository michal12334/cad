use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::backend::Backend;
use crate::cqrs::cqrs::Command;
use crate::domain::events::beziers_c0::bezier_c0_deleted::BezierC0Deleted;
use crate::domain::events::beziers_c2::bezier_c2_deleted::BezierC2Deleted;
use crate::domain::events::beziers_int::bezier_int_deleted::BezierIntDeleted;
use crate::domain::events::gregories::gregory_deleted::GregoryDeleted;
use crate::domain::events::intersections::intersection_deleted::IntersectionDeleted;
use crate::domain::events::surfaces_c0::surface_c0_deleted::SurfaceC0Deleted;
use crate::domain::events::surfaces_c2::surface_c2_deleted::SurfaceC2Deleted;
use crate::domain::events::toruses::torus_deleted::TorusDeleted;
use crate::domain::intersection::IntersectionObjectId;

pub struct DeleteSelectedObjects;

impl Command<DeleteSelectedObjects> for DeleteSelectedObjects {
    fn execute(_command: &DeleteSelectedObjects, app_state: Rc<RefCell<Backend>>) {
        let mut binding = app_state.borrow_mut();
        let backend = binding.deref_mut();
        backend.storage.intersections.retain(|_, intersection| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.intersection_id == Some(intersection.id))
        });
        backend.storage.toruses.retain(|_, torus| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.torus_id == Some(torus.id))
                || backend.storage.intersections.values().any(|i| {
                    i.object1_id == IntersectionObjectId::Torus(torus.id)
                        || i.object2_id == IntersectionObjectId::Torus(torus.id)
                })
        });
        backend.storage.beziers_c0.retain(|_, bezier| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.bezier_c0_id == Some(bezier.id))
        });
        backend.storage.beziers_c2.retain(|_, bezier| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.bezier_c2_id == Some(bezier.id))
        });
        backend.storage.beziers_int.retain(|_, bezier| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.bezier_int_id == Some(bezier.id))
        });
        backend.storage.surfaces_c0.retain(|_, surface| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.surface_c0_id == Some(surface.id))
                || backend.storage.intersections.values().any(|i| {
                    i.object1_id == IntersectionObjectId::SurfaceC0(surface.id)
                        || i.object2_id == IntersectionObjectId::SurfaceC0(surface.id)
                })
        });
        backend.storage.surfaces_c2.retain(|_, surface| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.surface_c2_id == Some(surface.id))
                || backend.storage.intersections.values().any(|i| {
                    i.object1_id == IntersectionObjectId::SurfaceC2(surface.id)
                        || i.object2_id == IntersectionObjectId::SurfaceC2(surface.id)
                })
        });
        backend.storage.gregories.retain(|_, gregory| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.gregory_id == Some(gregory.id))
        });
        backend.storage.points.retain(|_, point| {
            !backend
                .storage
                .selected_objects
                .iter()
                .any(|object| object.point_id == Some(point.id))
                || backend
                    .storage
                    .beziers_c0
                    .values()
                    .any(|b| b.points.iter().any(|p| p.id == point.id))
                || backend
                    .storage
                    .beziers_c2
                    .values()
                    .any(|b| b.b_spline_points.iter().any(|p| p.id == point.id))
                || backend
                    .storage
                    .beziers_int
                    .values()
                    .any(|b| b.points.iter().any(|p| p.id == point.id))
                || backend
                    .storage
                    .surfaces_c0
                    .values()
                    .any(|s| s.points.iter().any(|p| p.id == point.id))
                || backend
                    .storage
                    .surfaces_c2
                    .values()
                    .any(|s| s.points.iter().any(|p| p.id == point.id))
                || backend
                    .storage
                    .gregories
                    .values()
                    .any(|g| g.related_points().contains(&point.id))
        });

        let deleted_intersections = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.intersection_id)
            .collect::<Vec<_>>();

        let deleted_toruses = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.torus_id)
            .filter(|id| !backend.storage.toruses.contains_key(&id))
            .collect::<Vec<_>>();

        let deleted_beziers_c0 = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.bezier_c0_id)
            .collect::<Vec<_>>();

        let deleted_beziers_c2 = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.bezier_c2_id)
            .collect::<Vec<_>>();

        let deleted_beziers_int = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.bezier_int_id)
            .collect::<Vec<_>>();

        let deleted_surfaces_c0 = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.surface_c0_id)
            .filter(|id| !backend.storage.surfaces_c0.contains_key(&id))
            .collect::<Vec<_>>();

        let deleted_surfaces_c2 = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.surface_c2_id)
            .filter(|id| !backend.storage.surfaces_c2.contains_key(&id))
            .collect::<Vec<_>>();

        let deleted_gregories = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|object| object.gregory_id)
            .collect::<Vec<_>>();

        backend.storage.selected_objects.clear();

        drop(binding);

        let backend = app_state.borrow();
        deleted_intersections.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(IntersectionDeleted::new(*id)));
        });
        deleted_toruses.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(TorusDeleted::new(*id)));
        });
        deleted_beziers_c0.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(BezierC0Deleted::new(*id)));
        });
        deleted_beziers_c2.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(BezierC2Deleted::new(*id)));
        });
        deleted_beziers_int.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(BezierIntDeleted::new(*id)));
        });
        deleted_surfaces_c0.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(SurfaceC0Deleted::new(*id)));
        });
        deleted_surfaces_c2.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(SurfaceC2Deleted::new(*id)));
        });
        deleted_gregories.iter().for_each(|id| {
            backend
                .services
                .event_publisher
                .publish(Rc::new(GregoryDeleted::new(*id)));
        });
    }
}
