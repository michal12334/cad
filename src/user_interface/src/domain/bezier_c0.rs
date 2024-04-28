use backend::cqrs::beziers_c0::bezier_c0_details::BezierC0DTO;

pub struct BezierC0 {
    pub id: u64,
    pub name: String,
    pub points: Vec<BezierC0Point>,
}

pub struct BezierC0Point {
    pub id: u64,
    pub name: String,
    pub is_selected: bool,
}

impl BezierC0 {
    pub fn from_dto(dto: &BezierC0DTO) -> Self {
        BezierC0 {
            id: dto.id,
            name: dto.name.clone(),
            points: dto
                .points
                .iter()
                .map(|bp| BezierC0Point {
                    id: bp.id,
                    name: bp.name.clone(),
                    is_selected: false,
                })
                .collect(),
        }
    }
}
