pub struct SelectedObject {
    pub torus_id: Option<u64>,
    pub point_id: Option<u64>,
    pub bezier_c0_id: Option<u64>,
    pub bezier_c2_id: Option<u64>,
    pub bezier_int_id: Option<u64>,
    pub surface_c0_id: Option<u64>,
    pub surface_c2_id: Option<u64>,
}

impl SelectedObject {
    pub fn new_torus(torus_id: u64) -> Self {
        Self {
            torus_id: Some(torus_id),
            point_id: None,
            bezier_c0_id: None,
            bezier_c2_id: None,
            bezier_int_id: None,
            surface_c0_id: None,
            surface_c2_id: None,
        }
    }

    pub fn new_point(point_id: u64) -> Self {
        Self {
            torus_id: None,
            point_id: Some(point_id),
            bezier_c0_id: None,
            bezier_c2_id: None,
            bezier_int_id: None,
            surface_c0_id: None,
            surface_c2_id: None,
        }
    }

    pub fn new_bezier_c0(bezier_c0_id: u64) -> Self {
        Self {
            torus_id: None,
            point_id: None,
            bezier_c0_id: Some(bezier_c0_id),
            bezier_c2_id: None,
            bezier_int_id: None,
            surface_c0_id: None,
            surface_c2_id: None,
        }
    }

    pub fn new_bezier_c2(bezier_c2_id: u64) -> Self {
        Self {
            torus_id: None,
            point_id: None,
            bezier_c0_id: None,
            bezier_c2_id: Some(bezier_c2_id),
            bezier_int_id: None,
            surface_c0_id: None,
            surface_c2_id: None,
        }
    }

    pub fn new_bezier_int(bezier_int_id: u64) -> Self {
        Self {
            torus_id: None,
            point_id: None,
            bezier_c0_id: None,
            bezier_c2_id: None,
            bezier_int_id: Some(bezier_int_id),
            surface_c0_id: None,
            surface_c2_id: None,
        }
    }
    
    pub fn new_surface_c0(surface_c0_id: u64) -> Self {
        Self {
            torus_id: None,
            point_id: None,
            bezier_c0_id: None,
            bezier_c2_id: None,
            bezier_int_id: None,
            surface_c0_id: Some(surface_c0_id),
            surface_c2_id: None,
        }
    }
    
    pub fn new_surface_c2(surface_c2_id: u64) -> Self {
        Self {
            torus_id: None,
            point_id: None,
            bezier_c0_id: None,
            bezier_c2_id: None,
            bezier_int_id: None,
            surface_c0_id: None,
            surface_c2_id: Some(surface_c2_id),
        }
    }
}
