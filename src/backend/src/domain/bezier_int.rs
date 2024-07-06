use itertools::Itertools;
use crate::domain::point::Point;
use crate::domain::transformer::LittleTransformer;

pub struct BezierInt {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub draw_bernstein_polygon: bool,
    pub draw_bernstein_points: bool,
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
            draw_polygon: false,
            draw_bernstein_polygon: false,
            draw_bernstein_points: false,
            points,
            bernstein_points,
        }
    }
    
    fn get_bernstein_points(points: &[Point]) -> Vec<BezierIntBernsteinPoint> {
        if points.len() < 2 {
            vec![]
        } else {
            points
                .iter()
                .tuple_windows()
                .flat_map(|(a, b)| {
                    let p1 = BezierIntBernsteinPoint {
                        transformer: a.transformer.clone(),
                    };
                    let p2 = BezierIntBernsteinPoint {
                        transformer: LittleTransformer {
                            position: (
                                a.transformer.position.0 + (b.transformer.position.0 - a.transformer.position.0) / 3.0,
                                a.transformer.position.1 + (b.transformer.position.1 - a.transformer.position.1) / 3.0 + 2.0,
                                a.transformer.position.2 + (b.transformer.position.2 - a.transformer.position.2) / 3.0 + 3.0,
                            ),
                        }
                    };
                    let p3 = BezierIntBernsteinPoint {
                        transformer: LittleTransformer {
                            position: (
                                a.transformer.position.0 + (b.transformer.position.0 - a.transformer.position.0) * 2.0 / 3.0,
                                a.transformer.position.1 + (b.transformer.position.1 - a.transformer.position.1) * 2.0 / 3.0 + 1.0,
                                a.transformer.position.2 + (b.transformer.position.2 - a.transformer.position.2) * 2.0 / 3.0 + 2.0,
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
