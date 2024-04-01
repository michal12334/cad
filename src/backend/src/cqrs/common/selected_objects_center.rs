use crate::app_state::AppState;
use crate::cqrs::cqrs::Query;
use crate::cqrs::points::point_details::LittleTransformerDTO;

pub struct SelectedObjectsCenter;

impl Query<SelectedObjectsCenter, Option<LittleTransformerDTO>> for SelectedObjectsCenter {
    fn get(_query: &SelectedObjectsCenter, app_state: &AppState) -> Option<LittleTransformerDTO> {
        if app_state.storage.selected_objects.len() >= 2 {
            app_state
                .storage
                .selected_objects
                .iter()
                .flat_map(|object| {
                    app_state
                        .storage
                        .toruses
                        .values()
                        .filter(|torus| object.torus_id == Some(torus.id))
                        .map(|torus| LittleTransformerDTO {
                            position: torus.transformer.position,
                        })
                        .chain(
                            app_state
                                .storage
                                .points
                                .values()
                                .filter(|point| object.point_id == Some(point.id))
                                .map(|point| LittleTransformerDTO {
                                    position: point.transformer.position,
                                }),
                        )
                })
                .map(|x| (1, x))
                .reduce(|x, y| {
                    (
                        x.0 + y.0,
                        LittleTransformerDTO {
                            position: (
                                x.1.position.0 + y.1.position.0,
                                x.1.position.1 + y.1.position.1,
                                x.1.position.2 + y.1.position.2,
                            ),
                        },
                    )
                })
                .map(|x| LittleTransformerDTO {
                    position: (
                        x.1.position.0 / x.0 as f64,
                        x.1.position.1 / x.0 as f64,
                        x.1.position.2 / x.0 as f64,
                    ),
                })
        } else {
            None
        }
    }
}
