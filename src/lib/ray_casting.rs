use cgmath::{Vector3, Vector4, Point3, Point2, Transform, Matrix4};
use crate::{Camera, DzahuiWindow};

pub struct Cone {
    direction: Vector3<f64>,
    anchorage_point: Point3<f64>,
    angle: f64
}

impl Cone {
    pub fn new(direction: Vector3<f64>, anchorage_point: Point3<f64>, angle: f64) -> Cone {
        Cone { direction, anchorage_point, angle}
    }

    pub fn from_mouse_position(direction: Vector3<f64>, angle: f64, mouse_coordinates: Point2<f64>, camera: &Camera, window: &DzahuiWindow) -> Cone {
        let ndc_coordinates = Vector4::new(
            (mouse_coordinates.x - (window.width as f64)/2.0)/((window.width as f64)/2.0), // map between -1 and 1
            (mouse_coordinates.y - (window.height as f64)/2.0)/((window.height as f64)/2.0),
            -1.0, // we start at near plane
            1.0
        );
        let combined_matrix = camera.projection_matrix * camera.view_matrix;
        // cast becomes unnecesary if all values are f64, but camera is not working with f64 and opengl
        let inverse_matrix: Matrix4<f64> = combined_matrix.inverse_transform().expect("No inverse transform exists for this matrix").cast().unwrap();
        let world_coordinates = inverse_matrix * ndc_coordinates;
        let anchorage_point = Point3::new(world_coordinates.x,world_coordinates.y,world_coordinates.z);
        
        Cone { direction, anchorage_point, angle }
    }

    pub fn obtain_nearest_intersection() {}
}