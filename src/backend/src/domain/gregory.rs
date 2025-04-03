use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use derive_new::new;
use itertools::Itertools;
use math::vector3::Vector3;

use super::point::Point;

#[derive(Debug, Clone)]
pub struct Gregory {
    pub id: u64,
    pub name: String,
    pub patches: Vec<GregoryPatch>,
    pub triangle: Triangle,
    pub tess_level: u8,
}

#[derive(Debug, Clone, new)]
pub struct GregoryPatch {
    pub top: [Vector3; 4],
    pub top_sides: [Vector3; 2],
    pub bottom_sides: [Vector3; 2],
    pub bottom: [Vector3; 4],
    pub u_inner: [Vector3; 4],
    pub v_inner: [Vector3; 4],
}

impl Gregory {
    pub fn new(id: u64, triangle: Triangle, points: &HashMap<u64, Point>) -> Self {
        let t = GregoryTriangle::from_triangle(&triangle, points);

        Self {
            id,
            name: format!("Gregory {}", id),
            patches: t.patches.iter().cloned().collect(),
            triangle,
            tess_level: 4,
        }
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn recalculate_mesh(&mut self, points: &HashMap<u64, Point>) {
        let t = GregoryTriangle::from_triangle(&self.triangle, points);
        self.patches = t.patches.iter().cloned().collect();
    }

    pub fn related_points(&self) -> HashSet<u64> {
        self.triangle
            .edges
            .iter()
            .flat_map(|e| e.patch_points.iter().flat_map(|p| p))
            .copied()
            .collect()
    }

    pub fn update_settings(&mut self, tess_level: u8) {
        self.tess_level = tess_level;
    }
}

#[derive(Debug, Clone, new)]
pub struct Edge {
    pub edge_points: [u64; 4],
    pub patch_points: [[u64; 4]; 4],
}

impl Edge {
    pub fn inverse(&self) -> Self {
        Self::new(
            [
                self.edge_points[3],
                self.edge_points[2],
                self.edge_points[1],
                self.edge_points[0],
            ],
            [
                [
                    self.patch_points[0][3],
                    self.patch_points[0][2],
                    self.patch_points[0][1],
                    self.patch_points[0][0],
                ],
                [
                    self.patch_points[1][3],
                    self.patch_points[1][2],
                    self.patch_points[1][1],
                    self.patch_points[1][0],
                ],
                [
                    self.patch_points[2][3],
                    self.patch_points[2][2],
                    self.patch_points[2][1],
                    self.patch_points[2][0],
                ],
                [
                    self.patch_points[3][3],
                    self.patch_points[3][2],
                    self.patch_points[3][1],
                    self.patch_points[3][0],
                ],
            ],
        )
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        let s1: HashSet<u64> = HashSet::from_iter(self.edge_points.iter().map(|x| *x));
        let s2: HashSet<u64> = HashSet::from_iter(other.edge_points.iter().map(|x| *x));

        s1 == s2
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edge_points.iter().sum::<u64>().hash(state);
    }
}

#[derive(Debug, Clone, new)]
pub struct Triangle {
    pub edges: [Edge; 3],
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        let s1: HashSet<Edge> = HashSet::from_iter(self.edges.iter().map(|x| x.clone()));
        let s2: HashSet<Edge> = HashSet::from_iter(other.edges.iter().map(|x| x.clone()));

        s1 == s2
    }
}

impl Eq for Triangle {}

impl Hash for Triangle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edges
            .iter()
            .flat_map(|e| e.edge_points)
            .sum::<u64>()
            .hash(state);
    }
}

#[derive(Debug, Clone, new)]
pub struct BorderPatch {
    base_points: [[Vector3; 4]; 4],
}

impl BorderPatch {
    fn points(&self) -> [[Vector3; 4]; 2] {
        let bezier = &self.base_points[0];

        let divided = Self::divide_bezier(bezier, 0.5);

        [
            [divided.0[0], divided.0[1], divided.0[2], divided.0[3]],
            [divided.1[0], divided.1[1], divided.1[2], divided.1[3]],
        ]
    }

    fn divide_bezier(bezier: &[Vector3; 4], t: f32) -> ([Vector3; 4], [Vector3; 4]) {
        let mut p0 = [bezier[0]; 4];
        let mut p1 = [bezier[3]; 4];

        let t1 = 1.0 - t;

        let mut values = bezier.clone();
        let mut values_swap = [Vector3::zero(); 4];

        for i in (1..=3).rev() {
            for j in 0..i {
                values_swap[j] = t1 * values[j] + t * values[j + 1];
            }

            std::mem::swap(&mut values, &mut values_swap);

            p0[4 - i] = values[0];
            p1[i - 1] = values[i - 1];
        }

        (p0, p1)
    }

    fn diff_u(&self) -> [Vector3; 3] {
        let bezier0 = [
            self.base_points[0][0],
            self.base_points[1][0],
            self.base_points[2][0],
            self.base_points[3][0],
        ];

        let bezier1 = [
            self.base_points[0][3],
            self.base_points[1][3],
            self.base_points[2][3],
            self.base_points[3][3],
        ];

        let bezier_front = [
            self.base_points[0][0],
            self.base_points[0][1],
            self.base_points[0][2],
            self.base_points[0][3],
        ];

        let bezier_back = [
            self.base_points[1][0],
            self.base_points[1][1],
            self.base_points[1][2],
            self.base_points[1][3],
        ];

        let front_val = Self::bezier_value(&bezier_front, 0.5);
        let back_val = Self::bezier_value(&bezier_back, 0.5);

        [
            Self::bezier_derivative(&bezier0, 0.0),
            3.0 * (back_val - front_val),
            Self::bezier_derivative(&bezier1, 0.0),
        ]
    }

    fn twist(&self) -> [Vector3; 3] {
        let p = &self.base_points;
        let mut twist_bezier = Vec::new();
        for j in 0..3 {
            twist_bezier.push(9.0 * (p[1][j + 1] - p[0][j + 1] - p[1][j] + p[0][j]));
        }

        let w0 = Self::bezier_value(&twist_bezier, 0.0);
        let w1 = Self::bezier_value(&twist_bezier, 0.5);
        let w2 = Self::bezier_value(&twist_bezier, 1.0);

        [w0, w1, w2]
    }

    fn subdivide(&self) -> ([Vector3; 4], [Vector3; 4]) {
        let points = self.points();

        let bezier0 = [points[0][0], points[0][1], points[0][2], points[0][3]];

        let bezier1 = [points[1][0], points[1][1], points[1][2], points[1][3]];

        (bezier0, bezier1)
    }

    fn diff_v(&self) -> [[Vector3; 4]; 2] {
        let bezier = self.subdivide();

        [
            [
                Self::bezier_derivative(&bezier.0, 0.0),
                Self::bezier_derivative(&bezier.0, 1.0 / 3.0),
                Self::bezier_derivative(&bezier.0, 2.0 / 3.0),
                Self::bezier_derivative(&bezier.0, 1.0),
            ],
            [
                Self::bezier_derivative(&bezier.1, 0.0),
                Self::bezier_derivative(&bezier.1, 1.0 / 3.0),
                Self::bezier_derivative(&bezier.1, 2.0 / 3.0),
                Self::bezier_derivative(&bezier.1, 1.0),
            ],
        ]
    }

    fn points_v(&self) -> [[Vector3; 4]; 2] {
        let bezier = self.subdivide();

        [
            [
                Self::bezier_value(&bezier.0, 0.0),
                Self::bezier_value(&bezier.0, 1.0 / 3.0),
                Self::bezier_value(&bezier.0, 2.0 / 3.0),
                Self::bezier_value(&bezier.0, 1.0),
            ],
            [
                Self::bezier_value(&bezier.1, 0.0),
                Self::bezier_value(&bezier.1, 1.0 / 3.0),
                Self::bezier_value(&bezier.1, 2.0 / 3.0),
                Self::bezier_value(&bezier.1, 1.0),
            ],
        ]
    }

    fn bezier_value(points: &[Vector3], t: f32) -> Vector3 {
        let t1 = 1.0 - t;

        let degree = points.len() - 1;

        let mut values = points.iter().copied().collect::<Vec<_>>();

        for i in (1..=degree).rev() {
            for j in 0..i {
                values[j] = t1 * values[j] + t * values[j + 1];
            }
        }

        values[0]
    }

    fn bezier_derivative(points: &[Vector3], t: f32) -> Vector3 {
        let degree = (points.len() - 1) as f32;

        let derivative: Vec<_> = points
            .iter()
            .tuple_windows()
            .map(|(&a0, &a1)| degree * (-a0 + a1))
            .collect();

        Self::bezier_value(&derivative, t)
    }
}

pub struct GregoryTriangle {
    patches: [GregoryPatch; 3],
    // indexed as [patch][subpatch][point]
    v_diff: [[[Vector3; 4]; 2]; 3],
    v_diff_p: [[[Vector3; 4]; 2]; 3],
    // indexed as [patch][point]
    u_diff: [[Vector3; 3]; 3],
    twist: [[Vector3; 3]; 3],
    twist_u_p: [[Vector3; 3]; 3],
}

impl GregoryTriangle {
    fn from_triangle(triangle: &Triangle, points: &HashMap<u64, Point>) -> Self {
        let p0 = BorderPatch::new([
            [
                points[&triangle.edges[0].patch_points[0][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[0][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[0][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[0][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[0].patch_points[1][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[1][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[1][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[1][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[0].patch_points[2][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[2][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[2][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[2][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[0].patch_points[3][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[3][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[3][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[0].patch_points[3][3]]
                    .transformer
                    .to_vec3(),
            ],
        ]);

        let p1 = BorderPatch::new([
            [
                points[&triangle.edges[1].patch_points[0][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[0][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[0][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[0][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[1].patch_points[1][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[1][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[1][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[1][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[1].patch_points[2][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[2][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[2][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[2][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[1].patch_points[3][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[3][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[3][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[1].patch_points[3][3]]
                    .transformer
                    .to_vec3(),
            ],
        ]);

        let p2 = BorderPatch::new([
            [
                points[&triangle.edges[2].patch_points[0][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[0][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[0][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[0][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[2].patch_points[1][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[1][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[1][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[1][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[2].patch_points[2][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[2][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[2][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[2][3]]
                    .transformer
                    .to_vec3(),
            ],
            [
                points[&triangle.edges[2].patch_points[3][0]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[3][1]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[3][2]]
                    .transformer
                    .to_vec3(),
                points[&triangle.edges[2].patch_points[3][3]]
                    .transformer
                    .to_vec3(),
            ],
        ]);

        Self::new([p0, p1, p2])
    }

    fn new(border_patches: [BorderPatch; 3]) -> Self {
        let border_points: Vec<_> = border_patches.iter().map(|p| p.points()).collect();
        let border_tangents: Vec<_> = border_patches.iter().map(|p| p.diff_u()).collect();

        let p30 = border_points[0][1][0];
        let p31 = border_points[1][1][0];
        let p32 = border_points[2][1][0];

        let p20 = p30 - border_tangents[0][1] / 3.0;
        let p21 = p31 - border_tangents[1][1] / 3.0;
        let p22 = p32 - border_tangents[2][1] / 3.0;

        let q0 = (3.0 * p20 - p30) / 2.0;
        let q1 = (3.0 * p21 - p31) / 2.0;
        let q2 = (3.0 * p22 - p32) / 2.0;

        let p = Vector3::from((q0 + q1 + q2) / 3.0);

        let p10 = (p + 2.0 * q0) / 3.0;
        let p11 = (p + 2.0 * q1) / 3.0;
        let p12 = (p + 2.0 * q2) / 3.0;

        let [points00, points10] = border_patches[0].points();
        let [points01, points11] = border_patches[1].points();
        let [points02, points12] = border_patches[2].points();

        let u0 = border_patches[0].diff_u();
        let u1 = border_patches[1].diff_u();
        let u2 = border_patches[2].diff_u();

        let [v00, v10] = border_patches[0].diff_v();
        let [v01, v11] = border_patches[1].diff_v();
        let [v02, v12] = border_patches[2].diff_v();

        let w0 = border_patches[0].twist();
        let w1 = border_patches[1].twist();
        let w2 = border_patches[2].twist();

        Self {
            twist: [w0, w1, w2],
            twist_u_p: [
                [points00[0], p30, points10[3]],
                [points01[0], p31, points11[3]],
                [points02[0], p32, points12[3]],
            ],
            u_diff: [u0, u1, u2],
            v_diff: [[v00, v10], [v01, v11], [v02, v12]],
            v_diff_p: [
                border_patches[0].points_v(),
                border_patches[1].points_v(),
                border_patches[2].points_v(),
            ],
            patches: [
                GregoryPatch {
                    top: [p, p10, p20, p30],
                    top_sides: [p12, points00[2]],
                    bottom_sides: [p22, points00[1]],
                    bottom: points12,
                    u_inner: [
                        p + (p12 - p) + (p10 - p),
                        p30 - u0[1] / 3.0 - v00[3] / 3.0 + w0[1] / 9.0,
                        points12[0] - u2[1] / 3.0 + v12[0] / 3.0 + w2[1] / 9.0,
                        points12[3] - u2[2] / 3.0 - v12[3] / 3.0 + w2[2] / 9.0,
                    ],
                    v_inner: [
                        p + (p12 - p) + (p10 - p),
                        p30 - u0[1] / 3.0 - v00[3] / 3.0 + w0[1] / 9.0,
                        points12[0] - u2[1] / 3.0 + v12[0] / 3.0 + w2[1] / 9.0,
                        points00[0] - u0[0] / 3.0 + v00[0] / 3.0 + w0[0] / 9.0,
                    ],
                },
                GregoryPatch {
                    top: [p, p11, p21, p31],
                    top_sides: [p10, points01[2]],
                    bottom_sides: [p20, points01[1]],
                    bottom: points10,
                    u_inner: [
                        p + (p10 - p) + (p11 - p),
                        p31 - u1[1] / 3.0 - v01[3] / 3.0 + w1[1] / 9.0,
                        points10[0] - u0[1] / 3.0 + v10[0] / 3.0 + w0[1] / 9.0,
                        points10[3] - u0[2] / 3.0 - v10[3] / 3.0 + w0[2] / 9.0,
                    ],
                    v_inner: [
                        p + (p10 - p) + (p11 - p),
                        p31 - u1[1] / 3.0 - v01[3] / 3.0 + w1[1] / 9.0,
                        points10[0] - u0[1] / 3.0 + v10[0] / 3.0 + w0[1] / 9.0,
                        points01[0] - u1[0] / 3.0 + v01[0] / 3.0 + w1[0] / 9.0,
                    ],
                },
                GregoryPatch {
                    top: [p, p12, p22, p32],
                    top_sides: [p11, points02[2]],
                    bottom_sides: [p21, points02[1]],
                    bottom: points11,
                    u_inner: [
                        p + (p11 - p) + (p12 - p),
                        p32 - u2[1] / 3.0 - v02[3] / 3.0 + w2[1] / 9.0,
                        points11[0] - u1[1] / 3.0 + v11[0] / 3.0 + w1[1] / 9.0,
                        points11[3] - u1[2] / 3.0 - v11[3] / 3.0 + w1[2] / 9.0,
                    ],
                    v_inner: [
                        p + (p11 - p) + (p12 - p),
                        p32 - u2[1] / 3.0 - v02[3] / 3.0 + w2[1] / 9.0,
                        points11[0] - u1[1] / 3.0 + v11[0] / 3.0 + w1[1] / 9.0,
                        points02[0] - u2[0] / 3.0 + v02[0] / 3.0 + w2[0] / 9.0,
                    ],
                },
            ],
        }
    }
}
