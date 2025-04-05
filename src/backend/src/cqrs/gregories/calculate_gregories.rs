use std::{cell::RefCell, rc::Rc};

use itertools::Itertools;

use crate::{
    backend::Backend,
    cqrs::cqrs::Command,
    domain::{
        events::gregories::gregory_created::GregoryCreated,
        gregory::{Edge, Gregory, Triangle},
    },
};

pub struct CalculateGregories;

impl Command<CalculateGregories> for CalculateGregories {
    fn execute(_: &CalculateGregories, app_state: Rc<RefCell<Backend>>) {
        let mut backend = app_state.borrow_mut();

        let selected_surfaces_c0 = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|o| o.surface_c0_id)
            .map(|id| backend.storage.surfaces_c0.get(&id).unwrap());

        let edges = selected_surfaces_c0
            .flat_map(|s| {
                let s0 = s.size.0 as usize * 3 + 1;
                let s1 = s.size.1 as usize * 3 + 1;
                (0..(s.size.0 as usize))
                    .flat_map(move |x| {
                        let bx = x * 3;
                        [
                            Edge::new(
                                [
                                    s.points[bx * s1].id,
                                    s.points[(bx + 1) * s1].id,
                                    s.points[(bx + 2) * s1].id,
                                    s.points[(bx + 3) * s1].id,
                                ],
                                [
                                    [
                                        s.points[bx * s1].id,
                                        s.points[(bx + 1) * s1].id,
                                        s.points[(bx + 2) * s1].id,
                                        s.points[(bx + 3) * s1].id,
                                    ],
                                    [
                                        s.points[bx * s1 + 1].id,
                                        s.points[(bx + 1) * s1 + 1].id,
                                        s.points[(bx + 2) * s1 + 1].id,
                                        s.points[(bx + 3) * s1 + 1].id,
                                    ],
                                    [
                                        s.points[bx * s1 + 2].id,
                                        s.points[(bx + 1) * s1 + 2].id,
                                        s.points[(bx + 2) * s1 + 2].id,
                                        s.points[(bx + 3) * s1 + 2].id,
                                    ],
                                    [
                                        s.points[bx * s1 + 3].id,
                                        s.points[(bx + 1) * s1 + 3].id,
                                        s.points[(bx + 2) * s1 + 3].id,
                                        s.points[(bx + 3) * s1 + 3].id,
                                    ],
                                ],
                            ),
                            Edge::new(
                                [
                                    s.points[bx * s1 + s1 - 1].id,
                                    s.points[(bx + 1) * s1 + s1 - 1].id,
                                    s.points[(bx + 2) * s1 + s1 - 1].id,
                                    s.points[(bx + 3) * s1 + s1 - 1].id,
                                ],
                                [
                                    [
                                        s.points[bx * s1 + s1 - 1].id,
                                        s.points[(bx + 1) * s1 + s1 - 1].id,
                                        s.points[(bx + 2) * s1 + s1 - 1].id,
                                        s.points[(bx + 3) * s1 + s1 - 1].id,
                                    ],
                                    [
                                        s.points[bx * s1 + s1 - 2].id,
                                        s.points[(bx + 1) * s1 + s1 - 2].id,
                                        s.points[(bx + 2) * s1 + s1 - 2].id,
                                        s.points[(bx + 3) * s1 + s1 - 2].id,
                                    ],
                                    [
                                        s.points[bx * s1 + s1 - 3].id,
                                        s.points[(bx + 1) * s1 + s1 - 3].id,
                                        s.points[(bx + 2) * s1 + s1 - 3].id,
                                        s.points[(bx + 3) * s1 + s1 - 3].id,
                                    ],
                                    [
                                        s.points[bx * s1 + s1 - 4].id,
                                        s.points[(bx + 1) * s1 + s1 - 4].id,
                                        s.points[(bx + 2) * s1 + s1 - 4].id,
                                        s.points[(bx + 3) * s1 + s1 - 4].id,
                                    ],
                                ],
                            ),
                        ]
                    })
                    .chain((0..(s.size.1 as usize)).flat_map(move |y| {
                        let by = y * 3;
                        [
                            Edge::new(
                                [
                                    s.points[by].id,
                                    s.points[by + 1].id,
                                    s.points[by + 2].id,
                                    s.points[by + 3].id,
                                ],
                                [
                                    [
                                        s.points[by].id,
                                        s.points[by + 1].id,
                                        s.points[by + 2].id,
                                        s.points[by + 3].id,
                                    ],
                                    [
                                        s.points[s1 + by].id,
                                        s.points[s1 + by + 1].id,
                                        s.points[s1 + by + 2].id,
                                        s.points[s1 + by + 3].id,
                                    ],
                                    [
                                        s.points[2 * s1 + by].id,
                                        s.points[2 * s1 + by + 1].id,
                                        s.points[2 * s1 + by + 2].id,
                                        s.points[2 * s1 + by + 3].id,
                                    ],
                                    [
                                        s.points[3 * s1 + by].id,
                                        s.points[3 * s1 + by + 1].id,
                                        s.points[3 * s1 + by + 2].id,
                                        s.points[3 * s1 + by + 3].id,
                                    ],
                                ],
                            ),
                            Edge::new(
                                [
                                    s.points[(s0 as usize - 1) * s1 as usize + by].id,
                                    s.points[(s0 as usize - 1) * s1 as usize + by + 1].id,
                                    s.points[(s0 as usize - 1) * s1 as usize + by + 2].id,
                                    s.points[(s0 as usize - 1) * s1 as usize + by + 3].id,
                                ],
                                [
                                    [
                                        s.points[(s0 as usize - 1) * s1 as usize + by].id,
                                        s.points[(s0 as usize - 1) * s1 as usize + by + 1].id,
                                        s.points[(s0 as usize - 1) * s1 as usize + by + 2].id,
                                        s.points[(s0 as usize - 1) * s1 as usize + by + 3].id,
                                    ],
                                    [
                                        s.points[(s0 as usize - 2) * s1 as usize + by].id,
                                        s.points[(s0 as usize - 2) * s1 as usize + by + 1].id,
                                        s.points[(s0 as usize - 2) * s1 as usize + by + 2].id,
                                        s.points[(s0 as usize - 2) * s1 as usize + by + 3].id,
                                    ],
                                    [
                                        s.points[(s0 as usize - 3) * s1 as usize + by].id,
                                        s.points[(s0 as usize - 3) * s1 as usize + by + 1].id,
                                        s.points[(s0 as usize - 3) * s1 as usize + by + 2].id,
                                        s.points[(s0 as usize - 3) * s1 as usize + by + 3].id,
                                    ],
                                    [
                                        s.points[(s0 as usize - 4) * s1 as usize + by].id,
                                        s.points[(s0 as usize - 4) * s1 as usize + by + 1].id,
                                        s.points[(s0 as usize - 4) * s1 as usize + by + 2].id,
                                        s.points[(s0 as usize - 4) * s1 as usize + by + 3].id,
                                    ],
                                ],
                            ),
                        ]
                    }))
            })
            .collect::<Vec<_>>();

        let triangles = edges
            .iter()
            .flat_map(|e| [e.clone(), e.inverse()])
            .cartesian_product(edges.iter().flat_map(|e| [e.clone(), e.inverse()]))
            .cartesian_product(edges.iter().flat_map(|e| [e.clone(), e.inverse()]))
            .filter_map(|((e1, e2), e3)| {
                if e1 == e2 || e1 == e3 || e2 == e3 {
                    None
                } else if e1.edge_points[0] == e3.edge_points[3]
                    && e1.edge_points[3] == e2.edge_points[0]
                    && e2.edge_points[3] == e3.edge_points[0]
                {
                    Some(Triangle::new([e1.clone(), e2.clone(), e3.clone()]))
                } else {
                    None
                }
            })
            .unique()
            .collect::<Vec<_>>();

        let points = backend.storage.points.clone();

        let mut events = vec![];

        for t in triangles {
            let gregory = Gregory::new(backend.services.id_generator.next(), t, &points);

            events.push(GregoryCreated::new(
                gregory.id,
                gregory.name.clone(),
                4,
                gregory.draw_vectors,
            ));

            backend.storage.gregories.insert(gregory.id, gregory);
        }

        drop(backend);

        let backend = app_state.borrow();

        for e in events {
            backend.services.event_publisher.publish(Rc::new(e));
        }
    }
}
