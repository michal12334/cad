use math::vector3::Vector3;
use crate::domain::point::Point;
use crate::domain::transformer::LittleTransformer;

pub struct BezierC2 {
    pub id: u64,
    pub name: String,
    pub draw_polygon: bool,
    pub b_spline_points: Vec<BezierC2BSplinePoint>,
    pub bernstein_points: Vec<BezierC2BernsteinPoint>,
}

pub struct BezierC2BSplinePoint {
    pub id: u64,
}

pub struct BezierC2BernsteinPoint {
    pub transformer: LittleTransformer,
}

impl BezierC2 {
    pub fn new(
        id: u64,
        b_spline_points: Vec<Point>,
    ) -> Self {
        let bernstein_points = 
            if b_spline_points.len() < 4 {
                vec![]
            } else { 
                let mut g = 1.0/3.0 * b_spline_points[0].transformer.to_vec3() + 2.0/3.0 * b_spline_points[1].transformer.to_vec3();
                let mut f;
                let mut points = vec![];
                for i in 1..(b_spline_points.len() - 1) {
                    f = 2.0/3.0 * b_spline_points[i].transformer.to_vec3() + 1.0/3.0 * b_spline_points[i+1].transformer.to_vec3();
                    let e = (f + g) * 0.5;
                    g = 1.0/3.0 * b_spline_points[i].transformer.to_vec3() + 2.0/3.0 * b_spline_points[i+1].transformer.to_vec3();
                    points.push(BezierC2BernsteinPoint { transformer: LittleTransformer::from_vec3(e) });
                    points.push(BezierC2BernsteinPoint { transformer: LittleTransformer::from_vec3(f) });
                    points.push(BezierC2BernsteinPoint { transformer: LittleTransformer::from_vec3(g) });
                }
                f = 2.0/3.0 * b_spline_points[points.len() - 2].transformer.to_vec3() + 1.0/3.0 * b_spline_points[points.len() - 1].transformer.to_vec3();
                let e = (f + g) * 0.5;
                points.push(BezierC2BernsteinPoint { transformer: LittleTransformer::from_vec3(e) });
                points
            };

        let b_spline_points = b_spline_points
            .into_iter()
            .map(|point| BezierC2BSplinePoint { id: point.id })
            .collect();        
        
        Self {
            id,
            name: format!("BezierC2 {}", id),
            draw_polygon: false,
            b_spline_points,
            bernstein_points,
        }
    }
}
