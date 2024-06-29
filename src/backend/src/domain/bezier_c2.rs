use crate::domain::point::Point;
use crate::domain::transformer::LittleTransformer;

pub struct BezierC2 {
    pub id: u64,
    pub name: String,
    pub draw_b_spline_polygon: bool,
    pub draw_bernstein_polygon: bool,
    pub draw_bernstein_points: bool,
    pub b_spline_points: Vec<BezierC2BSplinePoint>,
    pub bernstein_points: Vec<BezierC2BernsteinPoint>,
    pub selected_bernstein_point: Option<usize>,
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
        let bernstein_points = Self::get_bernstein_points(&b_spline_points);

        let b_spline_points = b_spline_points
            .iter()
            .map(|point| BezierC2BSplinePoint { id: point.id })
            .collect();
        
        Self {
            id,
            name: format!("BezierC2 {}", id),
            draw_b_spline_polygon: false,
            draw_bernstein_polygon: false,
            draw_bernstein_points: false,
            b_spline_points,
            bernstein_points,
            selected_bernstein_point: None,
        }
    }
    
    pub fn update_points(&mut self, b_spline_points: Vec<Point>) {
        self.b_spline_points = b_spline_points
            .iter()
            .map(|point| BezierC2BSplinePoint { id: point.id })
            .collect();
        self.bernstein_points = Self::get_bernstein_points(&b_spline_points);
    }

    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
    
    fn get_bernstein_points(b_spline_points: &[Point]) -> Vec<BezierC2BernsteinPoint> {
        if b_spline_points.len() < 4 {
            vec![]
        } else {
            let mut g = 1.0/3.0 * b_spline_points[0].transformer.to_vec3() + 2.0/3.0 * b_spline_points[1].transformer.to_vec3();
            let mut f;
            let mut points = vec![];
            for i in 1..(b_spline_points.len() - 2) {
                f = 2.0/3.0 * b_spline_points[i].transformer.to_vec3() + 1.0/3.0 * b_spline_points[i+1].transformer.to_vec3();
                let e = (f + g) * 0.5;
                g = 1.0/3.0 * b_spline_points[i].transformer.to_vec3() + 2.0/3.0 * b_spline_points[i+1].transformer.to_vec3();
                points.push(BezierC2BernsteinPoint { transformer: LittleTransformer::from_vec3(e) });
                points.push(BezierC2BernsteinPoint { transformer: LittleTransformer::from_vec3(f) });
                points.push(BezierC2BernsteinPoint { transformer: LittleTransformer::from_vec3(g) });
            }
            f = 2.0/3.0 * b_spline_points[b_spline_points.len() - 2].transformer.to_vec3() + 1.0/3.0 * b_spline_points[b_spline_points.len() - 1].transformer.to_vec3();
            let e = (f + g) * 0.5;
            points.push(BezierC2BernsteinPoint { transformer: LittleTransformer::from_vec3(e) });
            points
        }
    }
    
    pub fn set_draw_b_spline_polygon(&mut self, draw_b_spline_polygon: bool) {
        self.draw_b_spline_polygon = draw_b_spline_polygon;
    }
    
    pub fn set_draw_bernstein_polygon(&mut self, draw_bernstein_polygon: bool) {
        self.draw_bernstein_polygon = draw_bernstein_polygon;
    }
    
    pub fn set_draw_bernstein_points(&mut self, draw_bernstein_points: bool) {
        self.draw_bernstein_points = draw_bernstein_points;
    }
    
    pub fn set_selected_bernstein_point(&mut self, selected_bernstein_point: Option<usize>) {
        self.selected_bernstein_point = selected_bernstein_point;
    }
}
