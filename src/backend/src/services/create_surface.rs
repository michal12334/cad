use crate::cqrs::surfaces_c0::create_surface_c0::CreateSurfaceInfoDTO;
use crate::domain::point::Point;
use crate::domain::surface_c0::{SurfaceC0, SurfaceC0Point};
use crate::domain::surface_c2::{SurfaceC2, SurfaceC2Point};
use crate::domain::transformer::LittleTransformer;
use crate::services::id_generator::IdGenerator;

pub fn create_surface_c0(
    id: u64,
    create_surface_info: &CreateSurfaceInfoDTO,
    id_generator: &mut IdGenerator,
    cursor_position: &LittleTransformer,
) -> (SurfaceC0, Vec<Point>) {
    if create_surface_info.is_cylinder {
        let radius = create_surface_info.radius.unwrap();
        let height = create_surface_info.height.unwrap();
        let size = create_surface_info.size;
        let size_x = size.0 * 3;
        let size_y = size.1 * 3 + 1;
        let points = (0..size_x)
            .flat_map(|x| (0..size_y).map(move |y| (x, y)))
            .map(|(x, y)| {
                let x = x as f64;
                let y = y as f64;
                let angle = 2.0 * std::f64::consts::PI * x / size_x as f64;
                let height = height * y / (size_y - 1) as f64;
                let position = (radius * angle.cos(), radius * angle.sin(), height);
                let position = LittleTransformer {
                    position: (
                        position.0 + cursor_position.position.0,
                        position.1 + cursor_position.position.1,
                        position.2 + cursor_position.position.2,
                    ),
                };
                Point::new(id_generator.next(), position)
            })
            .collect::<Vec<_>>();
        let surface_points = points
            .iter()
            .chain(points.iter().take(size_y as usize))
            .map(|point| SurfaceC0Point { id: point.id })
            .collect();
        let surface = SurfaceC0::new(id, surface_points, create_surface_info.size, true);
        (surface, points)
    } else {
        let width = create_surface_info.width.unwrap();
        let length = create_surface_info.length.unwrap();
        let size = create_surface_info.size;
        let size_x = size.0 * 3 + 1;
        let size_y = size.1 * 3 + 1;
        let points = (0..size_x)
            .flat_map(|x| (0..size_y).map(move |y| (x, y)))
            .map(|(x, y)| {
                let x = x as f64;
                let y = y as f64;
                let position = (
                    width * x / (size_x - 1) as f64,
                    0.0,
                    length * y / (size_y - 1) as f64,
                );
                let position = LittleTransformer {
                    position: (
                        position.0 + cursor_position.position.0,
                        position.1 + cursor_position.position.1,
                        position.2 + cursor_position.position.2,
                    ),
                };
                Point::new(id_generator.next(), position)
            })
            .collect::<Vec<_>>();
        let surface_points = points
            .iter()
            .map(|point| SurfaceC0Point { id: point.id })
            .collect();
        let surface = SurfaceC0::new(id, surface_points, create_surface_info.size, false);
        (surface, points)
    }
}

pub fn create_surface_c2(
    id: u64,
    create_surface_info: &CreateSurfaceInfoDTO,
    id_generator: &mut IdGenerator,
    cursor_position: &LittleTransformer,
) -> (SurfaceC2, Vec<Point>) {
    if create_surface_info.is_cylinder {
        let radius = create_surface_info.radius.unwrap();
        let height = create_surface_info.height.unwrap();
        let size = create_surface_info.size;
        let size_x = size.0;
        let size_y = size.1 + 3;
        let points = (0..size_x)
            .flat_map(|x| (0..size_y).map(move |y| (x, y)))
            .map(|(x, y)| {
                let x = x as f64;
                let y = y as f64;
                let angle = 2.0 * std::f64::consts::PI * x / size_x as f64;
                let height = height * y / (size_y - 1) as f64;
                let position = (radius * angle.cos(), radius * angle.sin(), height);
                let position = LittleTransformer {
                    position: (
                        position.0 + cursor_position.position.0,
                        position.1 + cursor_position.position.1,
                        position.2 + cursor_position.position.2,
                    ),
                };
                Point::new(id_generator.next(), position)
            })
            .collect::<Vec<_>>();
        let surface_points = points
            .iter()
            .chain(points.iter().take(3 * size_y as usize))
            .map(|point| SurfaceC2Point { id: point.id })
            .collect();
        let surface = SurfaceC2::new(id, surface_points, create_surface_info.size, true);
        (surface, points)
    } else {
        let width = create_surface_info.width.unwrap();
        let length = create_surface_info.length.unwrap();
        let size = create_surface_info.size;
        let size_x = size.0 + 3;
        let size_y = size.1 + 3;
        let points = (0..size_x)
            .flat_map(|x| (0..size_y).map(move |y| (x, y)))
            .map(|(x, y)| {
                let x = x as f64;
                let y = y as f64;
                let position = (
                    width * x / (size_x - 1) as f64,
                    0.0,
                    length * y / (size_y - 1) as f64,
                );
                let position = LittleTransformer {
                    position: (
                        position.0 + cursor_position.position.0,
                        position.1 + cursor_position.position.1,
                        position.2 + cursor_position.position.2,
                    ),
                };
                Point::new(id_generator.next(), position)
            })
            .collect::<Vec<_>>();
        let surface_points = points
            .iter()
            .map(|point| SurfaceC2Point { id: point.id })
            .collect();
        let surface = SurfaceC2::new(id, surface_points, create_surface_info.size, false);
        (surface, points)
    }
}
