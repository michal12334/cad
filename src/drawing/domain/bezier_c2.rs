use glium::{Display, IndexBuffer, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use backend::cqrs::beziers_c2::bezier_c2_bernstein_points::BezierC2BernsteinPointDTO;
use backend::cqrs::points::point_details::PointDTO;
use backend::domain::vertex::Vertex;

pub struct BezierC2 {
    pub id: u64,
    pub bernstein_points: Vec<Vertex>,
    pub b_spline_points: Vec<Vertex>,
    pub bernstein_vertex_buffer: Option<VertexBuffer<Vertex>>,
    pub b_spline_vertex_buffer: Option<VertexBuffer<Vertex>>,
    pub curve_index_buffer: Option<IndexBuffer<u16>>,
    pub bernstein_points_index_buffer: Option<IndexBuffer<u16>>,
    pub bernstein_polygon_index_buffer: Option<IndexBuffer<u16>>,
    pub b_spline_polygon_index_buffer: Option<IndexBuffer<u16>>,
    pub draw_bernstein_points: bool,
    pub draw_bernstein_polygon: bool,
    pub draw_b_spline_polygon: bool,
    pub selected_bernstein_point: Option<usize>,
}

impl BezierC2 {
    pub fn new(id: u64, bernstein_points: &[BezierC2BernsteinPointDTO], b_spline_points: &[PointDTO], display: &Display<WindowSurface>) -> Self {
        let bernstein_points = bernstein_points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();

        let b_spline_points = b_spline_points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();
        
        let (bernstein_vertex_buffer, b_spline_vertex_buffer, curve_index_buffer, bernstein_points_index_buffer, bernstein_polygon_index_buffer, b_spline_polygon_index_buffer) = Self::get_buffers(&bernstein_points, &b_spline_points, &display);
        
        Self {
            id,
            bernstein_points,
            b_spline_points,
            bernstein_vertex_buffer,
            b_spline_vertex_buffer,
            curve_index_buffer,
            bernstein_points_index_buffer,
            bernstein_polygon_index_buffer,
            b_spline_polygon_index_buffer,
            draw_bernstein_points: false,
            draw_bernstein_polygon: false,
            draw_b_spline_polygon: false,
            selected_bernstein_point: None,
        }
    }

    pub fn update_points(&mut self, points: &[BezierC2BernsteinPointDTO], b_spline_points: &[PointDTO], display: &Display<WindowSurface>) {
        self.bernstein_points = points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();
        
        self.b_spline_points = b_spline_points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();
        
        (self.bernstein_vertex_buffer, self.b_spline_vertex_buffer, self.curve_index_buffer, self.bernstein_points_index_buffer, self.bernstein_polygon_index_buffer, self.b_spline_polygon_index_buffer) = Self::get_buffers(&self.bernstein_points, &self.b_spline_points, &display);
    }
    
    fn get_buffers(bernstein_points: &Vec<Vertex>, b_spline_points: &Vec<Vertex>, display: &Display<WindowSurface>) -> (Option<VertexBuffer<Vertex>>, Option<VertexBuffer<Vertex>>, Option<IndexBuffer<u16>>, Option<IndexBuffer<u16>>, Option<IndexBuffer<u16>>, Option<IndexBuffer<u16>>) {
        let (bernstein_vertex_buffer, curve_index_buffer, bernstein_points_index_buffer, bernstein_polygon_index_buffer) =
            if bernstein_points.len() < 4 {
                (None, None, None, None)
            } else {
                (
                    Some(VertexBuffer::new(display, &bernstein_points).unwrap()),
                    Some(IndexBuffer::new(
                        display,
                        PrimitiveType::LinesListAdjacency,
                        &(0..(bernstein_points.len() as u16 - 3))
                            .step_by(3)
                            .flat_map(|f| [f, f + 1, f + 2, f + 3])
                            .collect::<Vec<u16>>(),
                    )
                        .unwrap()),
                    Some(IndexBuffer::new(
                        display,
                        PrimitiveType::Points,
                        &(0..bernstein_points.len() as u16)
                            .collect::<Vec<u16>>(),
                    )
                        .unwrap()),
                    Some(IndexBuffer::new(
                        display,
                        PrimitiveType::LineStrip,
                        &(0..bernstein_points.len() as u16)
                            .collect::<Vec<u16>>(),
                    )
                        .unwrap()),
                )
            };
        
        let (b_spline_vertex_buffer, b_spline_points_index_buffer) = if b_spline_points.is_empty() { 
            (None, None) 
        } else {
            (
                Some(VertexBuffer::new(display, &b_spline_points).unwrap()),
                Some(IndexBuffer::new(
                    display,
                    PrimitiveType::LineStrip,
                    &(0..b_spline_points.len() as u16)
                        .collect::<Vec<u16>>(),
                )
                    .unwrap()),
            )
        };
        
        (bernstein_vertex_buffer, b_spline_vertex_buffer, curve_index_buffer, bernstein_points_index_buffer, bernstein_polygon_index_buffer, b_spline_points_index_buffer)
    }
}
