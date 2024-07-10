use itertools::{Itertools, multizip};
use crate::domain::point::Point;
use crate::domain::transformer::LittleTransformer;

pub struct BezierInt {
    pub id: u64,
    pub name: String,
    pub points: Vec<BezierIntPoint>,
    pub bernstein_points: Vec<BezierIntBernsteinPoint>,
}

pub struct BezierIntPoint {
    pub id: u64,
}

pub struct BezierIntBernsteinPoint {
    pub transformer: LittleTransformer,
}


impl BezierInt {
    pub fn new(
        id: u64,
        points: Vec<Point>,
    ) -> Self {
        let bernstein_points = Self::get_bernstein_points(&points);

        let points = points
            .iter()
            .map(|point| BezierIntPoint { id: point.id })
            .collect();

        Self {
            id,
            name: format!("Bezier Int {}", id),
            points,
            bernstein_points,
        }
    }
    
    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
    
    pub fn update_points(&mut self, points: Vec<Point>) {
        self.points = points
            .iter()
            .map(|point| BezierIntPoint { id: point.id })
            .collect();
        self.bernstein_points = Self::get_bernstein_points(&points);
    }

    fn get_bernstein_points(points: &[Point]) -> Vec<BezierIntBernsteinPoint> {
        if points.len() < 2 {
            vec![]
        } else if points.len() == 2 {
            vec![
                BezierIntBernsteinPoint {
                    transformer: LittleTransformer {
                        position: points[0].transformer.position,
                    },
                },
                 BezierIntBernsteinPoint {
                     transformer: LittleTransformer {
                         position: points[0].transformer.position,
                     },
                 },
                 BezierIntBernsteinPoint {
                     transformer: LittleTransformer {
                         position: points[1].transformer.position,
                     },
                 },
                 BezierIntBernsteinPoint {
                     transformer: LittleTransformer {
                         position: points[1].transformer.position,
                     },
                 }
            ]
        } else {
            let n = points.len() - 1;

            let lengths = points
                .iter()
                .tuple_windows()
                .map(|(a, b)| {
                    ((a.transformer.position.0 - b.transformer.position.0).powi(2)+
                    (a.transformer.position.1 - b.transformer.position.1).powi(2)+
                    (a.transformer.position.2 - b.transformer.position.2).powi(2)).sqrt()
                })
                .collect::<Vec<_>>();

            let lower_diag = lengths
                .iter()
                .tuple_windows()
                .map(|(a, b)| {
                    a / (a + b)
                })
                .skip(1)
                .collect::<Vec<_>>();

            let upper_diag = lengths
                .iter()
                .tuple_windows()
                .map(|(a, b)| {
                    b / (a + b)
                })
                .take(n - 2)
                .collect::<Vec<_>>();
            
            let mut free_term = points
                .iter()
                .tuple_windows()
                .zip(lengths.iter().tuple_windows())
                .map(|((p1, p2, p3), (d1, d2))| {
                    let lhs = (
                        (p3.transformer.position.0 - p2.transformer.position.0) / d2,
                        (p3.transformer.position.1 - p2.transformer.position.1) / d2,
                        (p3.transformer.position.2 - p2.transformer.position.2) / d2,
                    );
                    let rhs = (
                        (p2.transformer.position.0 - p1.transformer.position.0) / d1,
                        (p2.transformer.position.1 - p1.transformer.position.1) / d1,
                        (p2.transformer.position.2 - p1.transformer.position.2) / d1,
                    );
                    let divisor = d1 + d2;
                    (
                        (lhs.0 - rhs.0) / divisor * 3.0,
                        (lhs.1 - rhs.1) / divisor * 3.0,
                        (lhs.2 - rhs.2) / divisor * 3.0,
                    )
                })
                .collect::<Vec<_>>();
            
            let k = n - 1;
            let mut diag = vec![2.0; k];

            for i in 1..k {
                let multiplier = lower_diag[i - 1] / diag[i - 1];
                diag[i] -= multiplier * upper_diag[i - 1];

                let term = free_term[i - 1];
                free_term[i].0 -= term.0 * multiplier;
                free_term[i].1 -= term.1 * multiplier;
                free_term[i].2 -= term.2 * multiplier;
            }
            
            for i in (1..k).rev() {
                let multiplier = upper_diag[i - 1] / diag[i];
                let term = free_term[i];
                free_term[i - 1].0 -= term.0 * multiplier;
                free_term[i - 1].1 -= term.1 * multiplier;
                free_term[i - 1].2 -= term.2 * multiplier;
                free_term[i].0 /= diag[i];
                free_term[i].1 /= diag[i];
                free_term[i].2 /= diag[i];
            }

            free_term[0].0 /= diag[0];
            free_term[0].1 /= diag[0];
            free_term[0].2 /= diag[0];

            let c: Vec<_> = [&[(0.0, 0.0, 0.0)], free_term.as_slice(), &[(0.0, 0.0, 0.0)]].concat();
            
            let d = c
                .iter()
                .tuple_windows()
                .zip(lengths.iter())
                .map(|((c1, c2), len)| {
                    (
                        (c2.0 - c1.0) / len / 3.0,
                        (c2.1 - c1.1) / len / 3.0,
                        (c2.2 - c1.2) / len / 3.0,
                    )
                })
                .collect::<Vec<_>>();
            
            let b: Vec<_> = multizip((points.iter().tuple_windows(), c.iter(), d.iter(), lengths.iter()))
                .map(|((p1, p2), c, d, len)| {
                    (
                        (p2.transformer.position.0 - p1.transformer.position.0) / len - c.0 * len - d.0 * len * len,
                        (p2.transformer.position.1 - p1.transformer.position.1) / len - c.1 * len - d.1 * len * len,
                        (p2.transformer.position.2 - p1.transformer.position.2) / len - c.2 * len - d.2 * len * len,
                    )
                })
            .collect();
            
            multizip((points, b, c, d, lengths))
                .map(|(a, b, c, d, len)| {
                    (a, (b.0 * len, b.1 * len, b.2 * len), (c.0 * len * len, c.1 * len * len, c.2 * len * len), (d.0 * len * len * len, d.1 * len * len * len, d.2 * len * len * len))
                })
                .flat_map(|(a, b, c, d)| {
                    let p1 = BezierIntBernsteinPoint {
                        transformer: a.transformer.clone(),
                    };
                    let p2 = BezierIntBernsteinPoint {
                        transformer: LittleTransformer {
                            position: (
                                a.transformer.position.0 + b.0 / 3.0,
                                a.transformer.position.1 + b.1 / 3.0,
                                a.transformer.position.2 + b.2 / 3.0,
                            ),
                        }
                    };
                    let p3 = BezierIntBernsteinPoint {
                        transformer: LittleTransformer {
                            position: (
                                a.transformer.position.0 + b.0 * 2.0 / 3.0 + c.0 / 3.0,
                                a.transformer.position.1 + b.1 * 2.0 / 3.0 + c.1 / 3.0,
                                a.transformer.position.2 + b.2 * 2.0 / 3.0 + c.2 / 3.0,
                            ),
                        }
                    };
                    [p1, p2, p3]
                })
                .chain([BezierIntBernsteinPoint {
                    transformer: points.last().unwrap().transformer.clone(),
                }])
                .collect()
        }
    }
}
