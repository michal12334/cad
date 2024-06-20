use backend::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPointDTO;
use backend::cqrs::beziers_c2::bezier_c2_details::BezierC2DTO;

pub struct BezierC2 {
    pub id: u64,
    pub name: String,
    pub b_spline_points: Vec<BezierC2BSplinePoint>,
    pub bernstein_points: Vec<BezierC2BernsteinPoint>,
    pub selected_point: Option<(u64, String)>,
    pub draw_b_spline_polygon: bool,
    pub draw_bernstein_polygon: bool,
    pub draw_bernstein_points: bool,
}

pub struct BezierC2BSplinePoint {
    pub id: u64,
    pub name: String,
    pub is_selected: bool,
}

pub struct BezierC2BernsteinPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl BezierC2 {
    pub fn from_dto(dto: &BezierC2DTO) -> Self {
        BezierC2 {
            id: dto.id,
            name: dto.name.clone(),
            selected_point: None,
            draw_b_spline_polygon: false,
            draw_bernstein_polygon: false,
            draw_bernstein_points: false,
            b_spline_points: dto
                .b_spline_points
                .iter()
                .map(|bp| BezierC2BSplinePoint {
                    id: bp.id,
                    name: bp.name.clone(),
                    is_selected: false,
                })
                .collect(),
            bernstein_points: Self::bernstein_points_from_dto(&dto.bernstein_points),
        }
    }
    
    pub fn set_bernstein_points(&mut self, bernstein_points: &[BezierC2BernsteinPointDTO]) {
        self.bernstein_points = Self::bernstein_points_from_dto(bernstein_points);
    }
    
    fn bernstein_points_from_dto(bernstein_points: &[BezierC2BernsteinPointDTO]) -> Vec<BezierC2BernsteinPoint> {
        bernstein_points
            .iter()
            .map(|bp| BezierC2BernsteinPoint {
                x: bp.transformer.position.0,
                y: bp.transformer.position.1,
                z: bp.transformer.position.2,
            })
            .collect()
    }
}

impl BezierC2BSplinePoint {
    pub fn new(id: u64, name: String) -> Self {
        BezierC2BSplinePoint {
            id,
            name,
            is_selected: false,
        }
    }
}
