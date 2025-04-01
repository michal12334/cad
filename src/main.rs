#[macro_use]
extern crate glium;
extern crate user_interface;

use std::cell::RefCell;
use std::rc::Rc;

use backend::processes::gregories::publishers::{GregoryCreatedPublisher, GregoryRenamedPublisher};
use backend::processes::gregories::recalculate_gregories_on_point_moved::RecalculateGregoriesOnPointMoved;
use drawing::drawers::gregory_drawer::GregoryDrawer;
use drawing::processes::common::rebuild_storage_on_selected_points_merged::RebuildStorageOnSelectedPointsMerged;
use egui::Color32;
use glium::{Blend, BlendingFunction, LinearBlendingFactor, PolygonMode, Surface};
use user_interface::processes::fetch_objects_on_selected_points_merged::FetchObjectsOnSelectedPointsMerged;
use user_interface::processes::sync_greogry_with_backend::{SyncGregoryCreation, SyncGregoryName};
use winit::event::ElementState::Pressed;
use winit::event::MouseButton;
use winit::{event, event_loop};

use backend::backend::Backend;
use backend::cqrs::common::selected_objects_center::SelectedObjectsCenter;
use backend::cqrs::cqrs::CQRS;
use backend::cqrs::cursors::transform_cursor::TransformCursor;
use backend::cqrs::points::all_points::AllPoints;
use backend::cqrs::points::point_details::LittleTransformerDTO;
use backend::domain::point::Point;
use backend::domain::transformer::LittleTransformer;
use backend::processes::beziers_c0::add_point_to_selected_beziers_c0_on_point_created::AddPointToSelectedBeziersC0OnPointCreated;
use backend::processes::beziers_c0::move_bezier_c0_points_on_point_moved::MoveBezierC0PointsOnPointMoved;
use backend::processes::beziers_c0::publishers::{
    BezierC0CreatedPublisher, BezierC0DeletedPublisher, BezierC0DrawPolygonSetPublisher,
    BezierC0PointMovedPublisher, BezierC0PointsDeletedPublisher, BezierC0RenamedPublisher,
    PointAddedToBezierC0Publisher,
};
use backend::processes::beziers_c2::add_point_to_selected_beziers_c2_on_point_created::AddPointToSelectedBeziersC2OnPointCreated;
use backend::processes::beziers_c2::move_bezier_c2_points_on_point_moved::MoveBezierC2PointsOnPointMoved;
use backend::processes::beziers_c2::publishers::{
    BezierC2CreatedPublisher, BezierC2DeletedPublisher, BezierC2DrawBSplinePolygonSetPublisher,
    BezierC2DrawBernsteinPointsSetPublisher, BezierC2DrawBernsteinPolygonSetPublisher,
    BezierC2PointMovedPublisher, BezierC2PointsDeletedPublisher,
    BezierC2SelectedBernsteinPointSetPublisher, PointAddedToBezierC2Publisher,
};
use backend::processes::beziers_int::add_point_to_selected_bezier_int_on_point_created::AddPointToSelectedBezierIntOnPointCreated;
use backend::processes::beziers_int::publishers::{
    BezierIntBernsteinPointMovedPublisher, BezierIntCreatedPublisher, BezierIntDeletedPublisher,
    BezierIntPointsDeletedPublisher, PointAddedToBezierIntPublisher,
};
use backend::processes::beziers_int::update_bezier_int_points_on_point_moved::UpdateBezierIntPointsOnPointMoved;
use backend::processes::common::publishers::SceneLoadedPublisher;
use backend::processes::points::publishers::PointMovedPublisher;
use backend::processes::surfaces_c0::move_surface_c0_point_on_point_moved::MoveSurfaceC0PointOnPointMoved;
use backend::processes::surfaces_c0::publishers::{
    SurfaceC0CreatedPublisher, SurfaceC0DeletedPublisher, SurfaceC0PointsSelectedPublisher,
    SurfaceC0UpdatedPublisher,
};
use backend::processes::surfaces_c2::move_surface_c2_point_on_point_moved::MoveSurfaceC2PointOnPointMoved;
use backend::processes::surfaces_c2::publishers::{
    SurfaceC2CreatedPublisher, SurfaceC2DeletedPublisher, SurfaceC2PointsSelectedPublisher,
    SurfaceC2UpdatedPublisher,
};
use infrastructure::event_bus::EventBus;
use math::vector4::Vector4;
use user_interface::processes::fetch_objects_on_scene_loaded::FetchObjectsOnSceneLoaded;
use user_interface::processes::selected_surface_c0_points_on_surface_c0_points_selected::SelectedSurfaceC0PointsOnSurfaceC0PointsSelected;
use user_interface::processes::selected_surface_c2_points_on_surface_c2_points_selected::SelectedSurfaceC2PointsOnSurfaceC2PointsSelected;
use user_interface::processes::sync_bezier_c0_with_backend::{
    SyncBezierC0AddedPointsWithBackend, SyncBezierC0DeletedPointsWithBackend,
    SyncBezierC0NameWithBackend,
};
use user_interface::processes::sync_bezier_c2_with_backend::{
    SyncBezierC2AddedPointsWithBackend, SyncBezierC2DeletedPointsWithBackend,
    SyncBezierC2PointPositionsWithBackend,
};
use user_interface::processes::sync_bezier_int_with_backend::{
    SyncBezierIntAddedPointWithBackend, SyncBezierIntPointsDeletedWithBackend,
};
use user_interface::processes::sync_point_with_backend::SyncPointPositionWithBackend;
use user_interface::ui::Ui;

use crate::drawing::drawers::bezier_c0_drawer::BezierC0Drawer;
use crate::drawing::drawers::bezier_c2_drawer::BezierC2Drawer;
use crate::drawing::drawers::bezier_int_drawer::BezierIntDrawer;
use crate::drawing::drawers::cursor_drawer::CursorDrawer;
use crate::drawing::drawers::infinite_grid_drawer::InfiniteGridDrawer;
use crate::drawing::drawers::point_drawer::PointDrawer;
use crate::drawing::drawers::points_drawer::PointsDrawer;
use crate::drawing::drawers::polygon_drawer::PolygonDrawer;
use crate::drawing::drawers::surface_c0_drawer::SurfaceC0Drawer;
use crate::drawing::drawers::surface_c2_drawer::SurfaceC2Drawer;
use crate::drawing::drawers::torus_drawer::TorusDrawer;
use crate::drawing::drawing_storage::DrawingStorage;
use crate::drawing::processes::beziers_c0::add_bezier_c0_on_bezier_c0_created::AddBezierC0OnBezierC0Created;
use crate::drawing::processes::beziers_c0::add_point_to_bezier_c0_on_point_added_to_bezier_c0::AddPointToBezierC0OnPointAddedToBezierC0;
use crate::drawing::processes::beziers_c0::delete_bezier_c0_on_bezier_c0_deleted::DeleteBezierC0OnBezierC0Deleted;
use crate::drawing::processes::beziers_c0::delete_bezier_c0_points_on_bezier_c0_points_deleted::DeleteBezierC0PointsOnBezierC0PointsDeleted;
use crate::drawing::processes::beziers_c0::set_draw_polygon_on_bezier_c0_draw_polygon_set::SetDrawPolygonOnBezierC0DrawPolygonSet;
use crate::drawing::processes::beziers_c0::update_bezier_c0_points_on_bezier_c0_point_moved::UpdateBezierC0PointsOnBezierC0PointMoved;
use crate::drawing::processes::beziers_c2::add_bezier_c2_on_bezier_c2_created::AddBezierC2OnBezierC2Created;
use crate::drawing::processes::beziers_c2::add_point_to_bezier_c2_on_point_added_to_bezier_c2::AddPointToBezierC2OnPointAddedToBezierC2;
use crate::drawing::processes::beziers_c2::delete_bezier_c2_on_bezier_c2_deleted::DeleteBezierC2OnBezierC2Deleted;
use crate::drawing::processes::beziers_c2::delete_bezier_c2_points_on_bezier_c2_points_deleted::DeleteBezierC2PointsOnBezierC2PointsDeleted;
use crate::drawing::processes::beziers_c2::set_draw_b_spline_polygon_on_bezier_c2_draw_b_spline_polygon_set::SetDrawBSplinePolygonOnBezierC2DrawBSplinePolygonSet;
use crate::drawing::processes::beziers_c2::set_draw_bernstein_points_on_bezier_c2_draw_bernstein_points_set::SetDrawBernsteinPointsOnBezierC2DrawBernsteinPointsSet;
use crate::drawing::processes::beziers_c2::set_draw_bernstein_polygon_on_bezier_c2_draw_bernstein_polygon_set::SetDrawBernsteinPolygonOnBezierC2DrawBernsteinPolygonSet;
use crate::drawing::processes::beziers_c2::set_selected_bernstein_point_on_bezier_c2_selected_bernstein_point_set::SetSelectedBernsteinPointOnBezierC2SelectedBernsteinPointSet;
use crate::drawing::processes::beziers_c2::update_bezier_c2_points_on_bezier_c2_point_moved::UpdateBezierC2PointsOnBezierC2PointMoved;
use crate::drawing::processes::beziers_int::add_bezier_int_on_bezier_int_created::AddBezierIntOnBezierIntCreated;
use crate::drawing::processes::beziers_int::add_point_to_bezier_int_on_point_added_to_bezier_int::AddPointToBezierIntOnPointAddedToBezierInt;
use crate::drawing::processes::beziers_int::delete_bezier_int_on_bezier_int_deleted::DeleteBezierIntOnBezierIntDeleted;
use crate::drawing::processes::beziers_int::delete_bezier_int_points_on_bezier_int_points_deleted::DeleteBezierIntPointsOnBezierIntPointsDeleted;
use crate::drawing::processes::beziers_int::update_bezier_int_points_on_bezier_int_bernstein_point_moved::UpdateBezierIntPointsOnBezierIntBernsteinPointMoved;
use crate::drawing::processes::common::rebuild_storage_on_scene_loaded::RebuildStorageOnSceneLoaded;
use crate::drawing::processes::surfaces_c0::add_surface_c0_on_surface_c0_created::AddSurfaceC0OnSurfaceC0Created;
use crate::drawing::processes::surfaces_c0::delete_surface_c0_on_surface_c0_deleted::DeleteSurfaceC0OnSurfaceC0Deleted;
use crate::drawing::processes::surfaces_c0::update_surface_c0_on_surface_c0_updated::UpdateSurfaceC0OnSurfaceC0Updated;
use crate::drawing::processes::surfaces_c0::update_surface_c0_points_on_surface_c0_point_moved::UpdateSurfaceC0PointsOnSurfaceC0PointMoved;
use crate::drawing::processes::surfaces_c2::add_surface_c2_on_surface_c2_created::AddSurfaceC2OnSurfaceC2Created;
use crate::drawing::processes::surfaces_c2::delete_surface_c2_on_surface_c2_deleted::DeleteSurfaceC2OnSurfaceC2Deleted;
use crate::drawing::processes::surfaces_c2::update_surface_c2_on_surface_c2_updated::UpdateSurfaceC2OnSurfaceC2Updated;
use crate::drawing::processes::surfaces_c2::update_surface_c2_points_on_surface_c2_point_moved::UpdateSurfaceC2PointsOnSurfaceC2PointMoved;

mod drawing;

fn main() {
    let mut width = 1200;
    let mut height = 900;

    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("CAD")
        .with_inner_size(width, height)
        .build(&event_loop);

    let display = Rc::new(display);

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &window, &event_loop);

    let event_bus = EventBus::new();
    let event_bus = Rc::new(RefCell::new(event_bus));

    let app_state = Rc::new(RefCell::new(Backend::new(event_bus.clone())));
    let ui = Rc::new(RefCell::new(Ui::new()));
    let drawing_storage = Rc::new(RefCell::new(DrawingStorage::new()));

    event_bus
        .borrow_mut()
        .add_consumer(BezierC0RenamedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC0PointsDeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(PointAddedToBezierC0Publisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC0CreatedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC0PointMovedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC0DeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC0DrawPolygonSetPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC2CreatedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC2DrawBernsteinPointsSetPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC2DrawBernsteinPolygonSetPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC2DrawBSplinePolygonSetPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(PointAddedToBezierC2Publisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC2PointsDeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC2PointMovedPublisher {
            backend: app_state.clone(),
        });
    event_bus.borrow_mut().add_consumer(PointMovedPublisher {
        backend: app_state.clone(),
    });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC2SelectedBernsteinPointSetPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddPointToSelectedBeziersC0OnPointCreated {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(MoveBezierC0PointsOnPointMoved {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddPointToSelectedBeziersC2OnPointCreated {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(MoveBezierC2PointsOnPointMoved {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierC2DeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierIntCreatedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(PointAddedToBezierIntPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddPointToSelectedBezierIntOnPointCreated {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierIntPointsDeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierIntBernsteinPointMovedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(UpdateBezierIntPointsOnPointMoved {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(BezierIntDeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SurfaceC0CreatedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(MoveSurfaceC0PointOnPointMoved {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SurfaceC0PointsSelectedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SurfaceC0UpdatedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SurfaceC0DeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SurfaceC2CreatedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(MoveSurfaceC2PointOnPointMoved {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SurfaceC2PointsSelectedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SurfaceC2UpdatedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SurfaceC2DeletedPublisher {
            backend: app_state.clone(),
        });
    event_bus.borrow_mut().add_consumer(SceneLoadedPublisher {
        backend: app_state.clone(),
    });
    event_bus
        .borrow_mut()
        .add_consumer(GregoryCreatedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(GregoryRenamedPublisher {
            backend: app_state.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(RecalculateGregoriesOnPointMoved {
            backend: app_state.clone(),
        });

    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC0NameWithBackend { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC0DeletedPointsWithBackend { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC0AddedPointsWithBackend { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC2AddedPointsWithBackend {
            ui: ui.clone(),
            cqrs: CQRS::new(app_state.clone()),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC2DeletedPointsWithBackend {
            ui: ui.clone(),
            cqrs: CQRS::new(app_state.clone()),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierC2PointPositionsWithBackend {
            ui: ui.clone(),
            cqrs: CQRS::new(app_state.clone()),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SyncPointPositionWithBackend { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierIntAddedPointWithBackend { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncBezierIntPointsDeletedWithBackend { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncGregoryCreation { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SyncGregoryName { ui: ui.clone() });
    event_bus
        .borrow_mut()
        .add_consumer(SelectedSurfaceC0PointsOnSurfaceC0PointsSelected {
            ui: ui.clone(),
            cqrs: CQRS::new(app_state.clone()),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SelectedSurfaceC2PointsOnSurfaceC2PointsSelected {
            ui: ui.clone(),
            cqrs: CQRS::new(app_state.clone()),
        });
    event_bus
        .borrow_mut()
        .add_consumer(FetchObjectsOnSceneLoaded {
            ui: ui.clone(),
            cqrs: CQRS::new(app_state.clone()),
        });
    event_bus
        .borrow_mut()
        .add_consumer(FetchObjectsOnSelectedPointsMerged {
            ui: ui.clone(),
            cqrs: CQRS::new(app_state.clone()),
        });

    event_bus
        .borrow_mut()
        .add_consumer(AddBezierC0OnBezierC0Created {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddPointToBezierC0OnPointAddedToBezierC0 {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(DeleteBezierC0PointsOnBezierC0PointsDeleted {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(UpdateBezierC0PointsOnBezierC0PointMoved {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(DeleteBezierC0OnBezierC0Deleted {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SetDrawPolygonOnBezierC0DrawPolygonSet {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddBezierC2OnBezierC2Created {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddPointToBezierC2OnPointAddedToBezierC2 {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SetDrawBernsteinPointsOnBezierC2DrawBernsteinPointsSet {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SetDrawBernsteinPolygonOnBezierC2DrawBernsteinPolygonSet {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(SetDrawBSplinePolygonOnBezierC2DrawBSplinePolygonSet {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(DeleteBezierC2PointsOnBezierC2PointsDeleted {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(UpdateBezierC2PointsOnBezierC2PointMoved {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus.borrow_mut().add_consumer(
        SetSelectedBernsteinPointOnBezierC2SelectedBernsteinPointSet {
            drawing_storage: drawing_storage.clone(),
        },
    );
    event_bus
        .borrow_mut()
        .add_consumer(DeleteBezierC2OnBezierC2Deleted {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddBezierIntOnBezierIntCreated {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddPointToBezierIntOnPointAddedToBezierInt {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(DeleteBezierIntPointsOnBezierIntPointsDeleted {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(UpdateBezierIntPointsOnBezierIntBernsteinPointMoved {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(DeleteBezierIntOnBezierIntDeleted {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddSurfaceC0OnSurfaceC0Created {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(UpdateSurfaceC0PointsOnSurfaceC0PointMoved {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(UpdateSurfaceC0OnSurfaceC0Updated {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(DeleteSurfaceC0OnSurfaceC0Deleted {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(AddSurfaceC2OnSurfaceC2Created {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(UpdateSurfaceC2PointsOnSurfaceC2PointMoved {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(UpdateSurfaceC2OnSurfaceC2Updated {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(DeleteSurfaceC2OnSurfaceC2Deleted {
            drawing_storage: drawing_storage.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(RebuildStorageOnSceneLoaded {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });
    event_bus
        .borrow_mut()
        .add_consumer(RebuildStorageOnSelectedPointsMerged {
            drawing_storage: drawing_storage.clone(),
            cqrs: CQRS::new(app_state.clone()),
            display: display.clone(),
        });

    let torus_drawer = TorusDrawer::new(&display);
    let point_drawer = PointDrawer::new(&display);
    let cursor_drawer = CursorDrawer::new(&display);
    let infinite_grid_drawer = InfiniteGridDrawer::new(&display);
    let bezier_c0_drawer = BezierC0Drawer::new(&display);
    let bezier_c2_drawer = BezierC2Drawer::new(&display);
    let bezier_int_drawer = BezierIntDrawer::new(&display);
    let polygon_drawer = PolygonDrawer::new(&display);
    let points_drawer = PointsDrawer::new(&display);
    let surface_c0_drawer = SurfaceC0Drawer::new(&display);
    let surface_c2_drawer = SurfaceC2Drawer::new(&display);
    let gregory_drawer = GregoryDrawer::new(&display);

    let mut mouse_position = (0.0, 0.0);
    let mut camera_direction = math::vector3::Vector3::new(0.0f32, 0.0, 1.0);
    let mut camera_angle = math::vector3::Vector3::new(0.0f32, 0.0, 0.0);
    let mut camera_up = math::vector3::Vector3::new(0.0f32, 1.0, 0.0);
    let mut camera_distant = 4.0f32;
    let mut view_matrix = math::matrix4::Matrix4::view(
        camera_direction * camera_distant * (-1.0),
        camera_direction,
        camera_up,
    );
    let mut mouse_middle_button_pressed = false;

    let color = Color32::WHITE.to_normalized_gamma_f32();
    let selected_color = Color32::YELLOW.to_normalized_gamma_f32();
    let bernstein_color = Color32::DARK_RED.to_normalized_gamma_f32();
    let selected_bernstein_color = Color32::LIGHT_GREEN.to_normalized_gamma_f32();
    let right_eye_color = [1.0, 0.0, 0.0, 1.0];
    let left_eye_color = [0.0, 1.0, 1.0, 1.0];

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        line_width: Some(1.0),
        point_size: Some(8.0),
        polygon_mode: PolygonMode::Line,
        blend: Blend::alpha_blending(),
        ..Default::default()
    };
    let draw_params_stereo = {
        let mut draw_params = draw_params.clone();

        draw_params.blend = Blend {
            color: BlendingFunction::Addition {
                source: LinearBlendingFactor::SourceAlpha,
                destination: LinearBlendingFactor::DestinationAlpha,
            },
            alpha: BlendingFunction::Addition {
                source: LinearBlendingFactor::SourceAlpha,
                destination: LinearBlendingFactor::DestinationAlpha,
            },
            constant_value: (0.0, 0.0, 0.0, 0.0),
        };

        draw_params
    };

    event_loop.run(move |event, _window_target, control_flow| {
        let mut redraw = || {
            let mut cqrs = CQRS::new(app_state.clone());
            let repaint_after;
            unsafe {
                let ui = ui.as_ptr();
                repaint_after = egui_glium.run(&window, (*ui).build(&mut cqrs));
            }

            *control_flow = if repaint_after.is_zero() {
                window.request_redraw();
                event_loop::ControlFlow::Poll
            } else if let Some(repaint_after_instant) =
                std::time::Instant::now().checked_add(repaint_after)
            {
                event_loop::ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                event_loop::ControlFlow::Wait
            };

            {
                let mut target = display.draw();

                target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                let cqrs = CQRS::new(app_state.clone());

                let app_state = app_state.borrow();

                if ui.borrow().stereoscopy {
                    let eye_distance = ui.borrow().stereoscopy_eye_distance;
                    let fov = ui.borrow().stereoscopy_fov;
                    let perspective = math::matrix4::Matrix4::perspective_stereoscopy(fov, width as f32 / height as f32, 0.1, 1024.0, -100.0 - eye_distance / 2.0, 100.0 - eye_distance / 2.0);
                    let view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0) - (math::matrix4::Matrix4::rotation_y(camera_angle.y) * math::matrix4::Matrix4::rotation_x(camera_angle.x) * Vector4::new(1.0, 0.0, 0.0, 0.0)).xyz() * (eye_distance / 2.0), camera_direction, camera_up);

                    for torus in app_state.storage.toruses.iter() {
                        torus_drawer.draw(&mut target, &display, &torus.1, &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    }

                    for point in app_state.storage.points.iter() {
                        point_drawer.draw(&mut target, &display, &point.1, &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    }

                    let center_point = cqrs.get(&SelectedObjectsCenter);
                    if let Some(center_point) = center_point {
                        let mut transformer = LittleTransformer::new();
                        transformer.position = center_point.position;
                        point_drawer.draw(&mut target, &display, &Point::new(0, transformer), &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c0.values() {
                        bezier_c0_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, right_eye_color, width, height, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values() {
                        bezier_c2_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, right_eye_color, width, height, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_int.values() {
                        bezier_int_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, right_eye_color, width, height, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c0.values().filter(|b| b.draw_polygon && b.polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.vertex_buffer.as_ref().unwrap(), &bezier.polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_b_spline_polygon && b.b_spline_polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.b_spline_vertex_buffer.as_ref().unwrap(), &bezier.b_spline_polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_bernstein_polygon && b.bernstein_polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.bernstein_vertex_buffer.as_ref().unwrap(), &bezier.bernstein_polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_bernstein_points && b.bernstein_points_index_buffer.is_some()) {
                        points_drawer.draw(&mut target, &bezier.bernstein_vertex_buffer.as_ref().unwrap(), &bezier.bernstein_points_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, right_eye_color, selected_bernstein_color, bezier.selected_bernstein_point, &draw_params_stereo);
                    }

                    for surface in drawing_storage.borrow().surfaces_c0.values() {
                        surface_c0_drawer.draw(&mut target, &surface, &perspective, &view_matrix, right_eye_color, surface.tess_level, &draw_params_stereo);
                    }

                    for surface in drawing_storage.borrow().surfaces_c0.values().filter(|s| s.draw_polygon) {
                        polygon_drawer.draw(&mut target, &surface.vertex_buffer, &surface.polygon_index_buffer, &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    }

                    for surface in drawing_storage.borrow().surfaces_c2.values() {
                        surface_c2_drawer.draw(&mut target, &surface, &perspective, &view_matrix, right_eye_color, surface.tess_level, &draw_params_stereo);
                    }

                    for surface in drawing_storage.borrow().surfaces_c2.values().filter(|s| s.draw_polygon) {
                        polygon_drawer.draw(&mut target, &surface.vertex_buffer, &surface.polygon_index_buffer, &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    }

                    cursor_drawer.draw(&mut target, &display, &app_state.storage.cursor, &perspective, &view_matrix, right_eye_color, &draw_params_stereo);
                    infinite_grid_drawer.draw(&mut target, &perspective.data, &view_matrix.data, right_eye_color, &draw_params_stereo);

                    let perspective = math::matrix4::Matrix4::perspective_stereoscopy(fov, width as f32 / height as f32, 0.1, 1024.0, -100.0 + eye_distance / 2.0, 100.0 + eye_distance / 2.0);
                    let view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0) + (math::matrix4::Matrix4::rotation_y(camera_angle.y) * math::matrix4::Matrix4::rotation_x(camera_angle.x) * Vector4::new(1.0, 0.0, 0.0, 0.0)).xyz() * (eye_distance / 2.0), camera_direction, camera_up);

                    target.clear_depth(1.0);

                    for torus in app_state.storage.toruses.iter() {
                        torus_drawer.draw(&mut target, &display, &torus.1, &perspective, &view_matrix, left_eye_color, &draw_params_stereo);
                    }

                    for point in app_state.storage.points.iter() {
                        point_drawer.draw(&mut target, &display, &point.1, &perspective, &view_matrix, left_eye_color, &draw_params_stereo);
                    }

                    let center_point = cqrs.get(&SelectedObjectsCenter);
                    if let Some(center_point) = center_point {
                        let mut transformer = LittleTransformer::new();
                        transformer.position = center_point.position;
                        point_drawer.draw(&mut target, &display, &Point::new(0, transformer), &perspective, &view_matrix, left_eye_color, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c0.values() {
                        bezier_c0_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, left_eye_color, width, height, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values() {
                        bezier_c2_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, left_eye_color, width, height, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_int.values() {
                        bezier_int_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, left_eye_color, width, height, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c0.values().filter(|b| b.draw_polygon && b.polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.vertex_buffer.as_ref().unwrap(), &bezier.polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, left_eye_color, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_b_spline_polygon && b.b_spline_polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.b_spline_vertex_buffer.as_ref().unwrap(), &bezier.b_spline_polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, left_eye_color, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_bernstein_polygon && b.bernstein_polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.bernstein_vertex_buffer.as_ref().unwrap(), &bezier.bernstein_polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, left_eye_color, &draw_params_stereo);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_bernstein_points && b.bernstein_points_index_buffer.is_some()) {
                        points_drawer.draw(&mut target, &bezier.bernstein_vertex_buffer.as_ref().unwrap(), &bezier.bernstein_points_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, left_eye_color, selected_bernstein_color, bezier.selected_bernstein_point, &draw_params_stereo);
                    }

                    for surface in drawing_storage.borrow().surfaces_c0.values() {
                        surface_c0_drawer.draw(&mut target, &surface, &perspective, &view_matrix, left_eye_color, surface.tess_level, &draw_params_stereo);
                    }

                    for surface in drawing_storage.borrow().surfaces_c0.values().filter(|s| s.draw_polygon) {
                        polygon_drawer.draw(&mut target, &surface.vertex_buffer, &surface.polygon_index_buffer, &perspective, &view_matrix, left_eye_color, &draw_params_stereo);
                    }

                    for surface in drawing_storage.borrow().surfaces_c2.values() {
                        surface_c2_drawer.draw(&mut target, &surface, &perspective, &view_matrix, left_eye_color, surface.tess_level, &draw_params_stereo);
                    }

                    for surface in drawing_storage.borrow().surfaces_c2.values().filter(|s| s.draw_polygon) {
                        polygon_drawer.draw(&mut target, &surface.vertex_buffer, &surface.polygon_index_buffer, &perspective, &view_matrix, left_eye_color, &draw_params_stereo);
                    }

                    cursor_drawer.draw(&mut target, &display, &app_state.storage.cursor, &perspective, &view_matrix, left_eye_color, &draw_params_stereo);

                    infinite_grid_drawer.draw(&mut target, &perspective.data, &view_matrix.data, left_eye_color, &draw_params_stereo);
                } else {
                    let perspective = math::matrix4::Matrix4::perspective(std::f32::consts::PI / 3.0, width as f32 / height as f32, 0.1, 1024.0);

                    for torus in app_state.storage.toruses.iter() {
                        let color = if app_state.storage.selected_objects.iter().any(|so| so.torus_id == Some(*torus.0)) { selected_color } else { color };
                        torus_drawer.draw(&mut target, &display, &torus.1, &perspective, &view_matrix, color, &draw_params);
                    }

                    for point in app_state.storage.points.iter() {
                        let color = if app_state.storage.selected_objects.iter().any(|so| so.point_id == Some(*point.0)) { selected_color } else { color };
                        point_drawer.draw(&mut target, &display, &point.1, &perspective, &view_matrix, color, &draw_params);
                    }

                    for gregory in app_state.storage.gregories.iter() {
                        gregory_drawer.draw(&mut target, gregory.1, &perspective, &view_matrix, color, 20, &draw_params, &display);
                    }

                    let center_point = cqrs.get(&SelectedObjectsCenter);
                    if let Some(center_point) = center_point {
                        let mut transformer = LittleTransformer::new();
                        transformer.position = center_point.position;
                        point_drawer.draw(&mut target, &display, &Point::new(0, transformer), &perspective, &view_matrix, Color32::BROWN.to_normalized_gamma_f32(), &draw_params);
                    }

                    for bezier in drawing_storage.borrow().beziers_c0.values() {
                        bezier_c0_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, color, width, height, &draw_params);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values() {
                        bezier_c2_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, color, width, height, &draw_params);
                    }

                    for bezier in drawing_storage.borrow().beziers_int.values() {
                        bezier_int_drawer.draw(&mut target, &bezier, &perspective, &view_matrix, color, width, height, &draw_params);
                    }

                    for bezier in drawing_storage.borrow().beziers_c0.values().filter(|b| b.draw_polygon && b.polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.vertex_buffer.as_ref().unwrap(), &bezier.polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, color, &draw_params);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_b_spline_polygon && b.b_spline_polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.b_spline_vertex_buffer.as_ref().unwrap(), &bezier.b_spline_polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, color, &draw_params);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_bernstein_polygon && b.bernstein_polygon_index_buffer.is_some()) {
                        polygon_drawer.draw(&mut target, &bezier.bernstein_vertex_buffer.as_ref().unwrap(), &bezier.bernstein_polygon_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, color, &draw_params);
                    }

                    for bezier in drawing_storage.borrow().beziers_c2.values().filter(|b| b.draw_bernstein_points && b.bernstein_points_index_buffer.is_some()) {
                        points_drawer.draw(&mut target, &bezier.bernstein_vertex_buffer.as_ref().unwrap(), &bezier.bernstein_points_index_buffer.as_ref().unwrap(), &perspective, &view_matrix, bernstein_color, selected_bernstein_color, bezier.selected_bernstein_point, &draw_params);
                    }

                    for surface in drawing_storage.borrow().surfaces_c0.values() {
                        surface_c0_drawer.draw(&mut target, &surface, &perspective, &view_matrix, color, surface.tess_level, &draw_params);
                    }

                    for surface in drawing_storage.borrow().surfaces_c0.values().filter(|s| s.draw_polygon) {
                        polygon_drawer.draw(&mut target, &surface.vertex_buffer, &surface.polygon_index_buffer, &perspective, &view_matrix, color, &draw_params);
                    }

                    for surface in drawing_storage.borrow().surfaces_c2.values() {
                        surface_c2_drawer.draw(&mut target, &surface, &perspective, &view_matrix, color, surface.tess_level, &draw_params);
                    }

                    for surface in drawing_storage.borrow().surfaces_c2.values().filter(|s| s.draw_polygon) {
                        polygon_drawer.draw(&mut target, &surface.vertex_buffer, &surface.polygon_index_buffer, &perspective, &view_matrix, color, &draw_params);
                    }

                    cursor_drawer.draw(&mut target, &display, &app_state.storage.cursor, &perspective, &view_matrix, [0.0, 1.0, 0.0, 1.0], &draw_params);

                    infinite_grid_drawer.draw(&mut target, &perspective.data, &view_matrix.data, color, &draw_params);
                }

                egui_glium.paint(&display, &mut target);

                target.finish().unwrap();
            }
        };
        match event {
            event::Event::RedrawRequested(_) => redraw(),

            event::Event::WindowEvent { event, .. } => {
                use event::WindowEvent;
                match &event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = event_loop::ControlFlow::Exit;
                    }
                    WindowEvent::Resized(new_size) => {
                        display.resize((*new_size).into());
                        width = new_size.width;
                        height = new_size.height;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let delta = (position.x - mouse_position.0, position.y - mouse_position.1);
                        mouse_position = (position.x, position.y);
                        if mouse_middle_button_pressed {
                            camera_angle.x += delta.1 as f32 * 0.01;
                            camera_angle.y += delta.0 as f32 * 0.01 * if camera_angle.x.cos() < 0.0 { -1.0 } else { 1.0 };
                            camera_direction = (math::matrix4::Matrix4::rotation_y(camera_angle.y) * math::matrix4::Matrix4::rotation_x(camera_angle.x) * Vector4::new(0.0, 0.0, 1.0, 0.0)).xyz();
                            camera_up = (math::matrix4::Matrix4::rotation_y(camera_angle.y) * math::matrix4::Matrix4::rotation_x(camera_angle.x) * Vector4::new(0.0, 1.0, 0.0, 0.0)).xyz();
                            view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0), camera_direction, camera_up);
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if *button == MouseButton::Middle {
                            mouse_middle_button_pressed = *state == Pressed;
                        } else if *button == MouseButton::Left && !ui.borrow().is_pointer_over_area() && *state == Pressed {
                            let x = mouse_position.0 / width as f64 * 2.0 - 1.0;
                            let y = 1.0 - mouse_position.1 / height as f64 * 2.0;
                            let point = Vector4::new(x as f32, y as f32, 0.95, 1.0);
                            let inversed_view_matrix = view_matrix.get_inversed();
                            let inversed_perspective_matrix = math::matrix4::Matrix4::perspective(std::f32::consts::PI / 3.0, width as f32 / height as f32, 0.1, 1024.0).get_inversed();
                            let point = (inversed_view_matrix * inversed_perspective_matrix * point).to_vector3();
                            let mut cqrs = CQRS::new(app_state.clone());
                            cqrs.execute(&TransformCursor { transformer: LittleTransformerDTO { position: (point.x as f64, point.y as f64, point.z as f64) } });
                        } else if *button == MouseButton::Right && !ui.borrow().is_pointer_over_area() && *state == Pressed {
                            let mut cqrs = CQRS::new(app_state.clone());
                            let points = cqrs.get(&AllPoints);
                            for point in points {
                                let position = Vector4::new(point.transformer.position.0 as f32, point.transformer.position.1 as f32, point.transformer.position.2 as f32, 1.0);
                                let position = (math::matrix4::Matrix4::perspective(std::f32::consts::PI / 3.0, width as f32 / height as f32, 0.1, 1024.0) * view_matrix * position).to_vector3();
                                let x = mouse_position.0 as f32 / width as f32 * 2.0 - 1.0;
                                let y = 1.0 - mouse_position.1 as f32 / height as f32 * 2.0;
                                if (position.x - x) * (position.x - x) + (position.y - y) * (position.y - y) <= 0.0001 {
                                    ui.borrow_mut().change_point_selection(point.id, &mut cqrs);
                                }
                            }
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        if !ui.borrow().is_pointer_over_area() {
                            match delta {
                                event::MouseScrollDelta::LineDelta(_x, y) => {
                                    camera_distant += -y * 0.1;
                                    view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0), camera_direction, camera_up);
                                }
                                _ => {}
                            }
                        }
                    }
                    WindowEvent::TouchpadMagnify { delta, .. } => {
                        if !ui.borrow().is_pointer_over_area() {
                            camera_distant += -delta as f32;
                                    view_matrix = math::matrix4::Matrix4::view(camera_direction * camera_distant * (-1.0), camera_direction, camera_up);
                        }
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.virtual_keycode == Some(event::VirtualKeyCode::LControl) {
                            ui.borrow_mut().set_control_pressed(input.state == Pressed);
                        } else if input.virtual_keycode == Some(event::VirtualKeyCode::Delete) && input.state == Pressed {
                            let mut cqrs = CQRS::new(app_state.clone());
                            cqrs.execute(&backend::cqrs::common::delete_selected_objects::DeleteSelectedObjects);
                            ui.borrow_mut().fetch_objects(&cqrs);
                        } else if input.virtual_keycode == Some(event::VirtualKeyCode::C) && input.state == Pressed {
                            mouse_middle_button_pressed = !mouse_middle_button_pressed;
                        }
                    }
                    _ => {}
                }

                let event_response = egui_glium.on_event(&event);

                if event_response.repaint {
                    window.request_redraw();
                }
            }
            event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }) => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
