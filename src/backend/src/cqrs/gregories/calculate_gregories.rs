use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

use derive_new::new;
use itertools::Itertools;

use crate::{backend::Backend, cqrs::cqrs::Command};

pub struct CalculateGregories;

impl Command<CalculateGregories> for CalculateGregories {
    fn execute(_: &CalculateGregories, app_state: Rc<RefCell<Backend>>) {
        let backend = app_state.borrow_mut();

        let selected_surfaces_c0 = backend
            .storage
            .selected_objects
            .iter()
            .filter_map(|o| o.surface_c0_id)
            .map(|id| backend.storage.surfaces_c0.get(&id).unwrap());

        let edges = selected_surfaces_c0.flat_map(|s| {
            let s0 = s.size.0 as usize * 3 + 1;
            let s1 = s.size.1 as usize * 3 + 1;
            (0..(s.size.0 as usize)).flat_map(move |x| {
                let bx = x * 3;
                [
                Edge::new([
                    s.points[bx * s1].id,
                    s.points[(bx + 1) * s1].id,
                    s.points[(bx + 2) * s1].id,
                    s.points[(bx + 3) * s1].id,
                ]),
                Edge::new([
                    s.points[bx * s1 + s1 - 1].id,
                    s.points[(bx + 1) * s1 + s1 - 1].id,
                    s.points[(bx + 2) * s1 + s1 - 1].id,
                    s.points[(bx + 3) * s1 + s1 - 1].id,
                ]),
                ]
            }).chain((0..(s.size.1 as usize)).flat_map(move |y| {
                let by = y * 3;
                [
                    Edge::new([
                        s.points[by].id,
                        s.points[by + 1].id,
                        s.points[by + 2].id,
                        s.points[by + 3].id,
                    ]),
                    Edge::new([
                        s.points[(s0 as usize - 1) * s1 as usize + by].id,
                        s.points[(s0 as usize - 1) * s1 as usize + by + 1].id,
                        s.points[(s0 as usize - 1) * s1 as usize + by + 2].id,
                        s.points[(s0 as usize - 1) * s1 as usize + by + 3].id,
                    ]),
                ]
            }))
        })
        .collect::<Vec<_>>();

        let triangles = edges.iter().flat_map(|e| [e.clone(), e.inverse()])
            .cartesian_product(edges.iter().flat_map(|e| [e.clone(), e.inverse()]))
            .cartesian_product(edges.iter().flat_map(|e| [e.clone(), e.inverse()]))
            .filter_map(|((e1, e2), e3)| {
                if e1 == e2 || e1 == e3 || e2 == e3 {
                    None
                } else if e1.points[0] == e3.points[3] && e1.points[3] == e2.points[0] && e2.points[3] == e3.points[0] {
                    Some(Triangle::new([e1.clone(), e2.clone(), e3.clone()]))
                } else {
                    None
                }
            })
            .unique()
            .collect::<Vec<_>>();

        for t in triangles {
            println!("({}, {}, {}, {}), ({}, {}, {}, {}), ({}, {}, {}, {})", t.edges[0].points[0], t.edges[0].points[1], t.edges[0].points[2], t.edges[0].points[3], t.edges[1].points[0], t.edges[1].points[1], t.edges[1].points[2], t.edges[1].points[3], t.edges[2].points[0], t.edges[2].points[1], t.edges[2].points[2], t.edges[2].points[3])
        }
    }
}

#[derive(Debug, Clone, new)]
struct Edge {
    points: [u64; 4],
}

impl Edge {
    fn inverse(&self) -> Self {
        Self::new([self.points[3], self.points[2], self.points[1], self.points[0]])
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        let s1: HashSet<u64> = HashSet::from_iter(self.points.iter().map(|x| *x));
        let s2: HashSet<u64> = HashSet::from_iter(other.points.iter().map(|x| *x));

        s1 == s2
    }
}

impl Eq for Edge {
    
}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.points.iter().sum::<u64>().hash(state);
    }
}

#[derive(Debug, Clone, new)]
struct Triangle {
    edges: [Edge; 3],
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        let s1: HashSet<Edge> = HashSet::from_iter(self.edges.iter().map(|x| x.clone()));
        let s2: HashSet<Edge> = HashSet::from_iter(other.edges.iter().map(|x| x.clone()));

        s1 == s2
    }
}

impl Eq for Triangle {
    
}

impl Hash for Triangle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edges.iter().flat_map(|e| e.points).sum::<u64>().hash(state);
    }
}
