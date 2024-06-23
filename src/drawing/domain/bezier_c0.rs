use glium::{Display, IndexBuffer, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use backend::cqrs::points::point_details::PointDTO;
use backend::domain::vertex::Vertex;

pub struct BezierC0 {
    pub id: u64,
    pub points: Vec<Vertex>,
    pub vertex_buffer: Option<VertexBuffer<Vertex>>,
    pub curve_index_buffer: Option<IndexBuffer<u16>>,
    pub polygon_index_buffer: Option<IndexBuffer<u16>>,
    pub draw_polygon: bool,
}

impl BezierC0 {
    pub fn new(id: u64, points: &[PointDTO], display: &Display<WindowSurface>) -> Self {
        let points = points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();
        
        let (vertex_buffer, curve_index_buffer, polygon_index_buffer) = Self::get_buffers(&points, &display);
        
        Self {
            id,
            points,
            vertex_buffer,
            curve_index_buffer,
            polygon_index_buffer,
            draw_polygon: false,
        }
    }
    
    pub fn add_point(&mut self, point: PointDTO, display: &Display<WindowSurface>) {
        self.points.push(Vertex {
            position: [
                point.transformer.position.0 as f32,
                point.transformer.position.1 as f32,
                point.transformer.position.2 as f32,
            ],
        });

        (self.vertex_buffer, self.curve_index_buffer, self.polygon_index_buffer) = Self::get_buffers(&self.points, &display);
    }
    
    pub fn update_points(&mut self, points: &[PointDTO], display: &Display<WindowSurface>) {
        self.points = points
            .iter()
            .map(|p| Vertex {
                position: [
                    p.transformer.position.0 as f32,
                    p.transformer.position.1 as f32,
                    p.transformer.position.2 as f32,
                ],
            })
            .collect::<Vec<Vertex>>();
        
        (self.vertex_buffer, self.curve_index_buffer, self.polygon_index_buffer) = Self::get_buffers(&self.points, &display);
    }
    
    fn get_buffers(points: &Vec<Vertex>, display: &Display<WindowSurface>) -> (Option<VertexBuffer<Vertex>>, Option<IndexBuffer<u16>>, Option<IndexBuffer<u16>>) {
        if points.len() < 2 {
            return (None, None, None);
        }
        
        let mut points = points.clone();
        while points.len() % 3 != 1 {
            points.push(Vertex {
                position: [0.0, 0.0, 0.0],
            });
        }
        (
            Some(VertexBuffer::new(display, &points).unwrap()),
            Some(IndexBuffer::new(
                display,
                PrimitiveType::LinesListAdjacency,
                &(0..(points.len() as u16 - 3))
                    .step_by(3)
                    .flat_map(|f| [f, f + 1, f + 2, f + 3])
                    .collect::<Vec<u16>>(),
            )
                .unwrap()),
            Some(IndexBuffer::new(
                display,
                PrimitiveType::LineStrip,
                &(0..points.len() as u16)
                    .collect::<Vec<u16>>(),
            )
                .unwrap()),
        )
    }
}
