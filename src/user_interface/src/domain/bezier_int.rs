use backend::cqrs::beziers_int::bezier_int_details::BezierIntDTO;

pub struct BezierInt {
    pub id: u64,
    pub name: String,
    pub points: Vec<BezierIntPoint>,
    pub selected_point: Option<(u64, String)>,
}

pub struct BezierIntPoint {
    pub id: u64,
    pub name: String,
    pub is_selected: bool,
}

impl BezierInt {
    pub fn from_dto(dto: &BezierIntDTO) -> Self {
        BezierInt {
            id: dto.id,
            name: dto.name.clone(),
            points: dto
                .points
                .iter()
                .map(|bp| BezierIntPoint {
                    id: bp.id,
                    name: bp.name.clone(),
                    is_selected: false,
                })
                .collect(),
            selected_point: None,
        }
    }
}
