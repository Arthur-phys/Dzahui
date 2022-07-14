use cgmath::{Vector3, Vector4, Point3, Point2, Transform, Matrix4};
use crate::{Camera, DzahuiWindow, drawable::sphere::Sphere};

#[derive(Debug)]
pub struct Cone {
    anchorage_point: Point3<f32>,
    angle: f32
}

impl Cone {
    pub fn new(anchorage_point: Point3<f32>, angle: f32) -> Cone {
        Cone { anchorage_point, angle}
    }

    pub fn from_mouse_position(angle: f32, mouse_coordinates: Point2<f64>, camera: &Camera, window: &DzahuiWindow) -> Cone {
        // Create cone from position of mouse
        let ndc_coordinates = Vector4::new(
            (mouse_coordinates.x - (window.width as f64)/2.0)/((window.width as f64)/2.0), // map between -1 and 1
            -(mouse_coordinates.y - (window.height as f64)/2.0)/((window.height as f64)/2.0),
            -1.0, // we start at near plane
            1.0
        );
        // cast becomes unnecesary if all values are f64, but camera is not working with f64 and opengl
        let inverse_matrix: Matrix4<f64> = camera.projection_matrix.inverse_transform().expect("No inverse transform exists for this matrix").cast().unwrap();
        let world_coordinates = inverse_matrix * ndc_coordinates;
        let anchorage_point: Point3<f32> = Point3::new(world_coordinates.x,world_coordinates.y, world_coordinates.z).cast().unwrap();
        
        Cone { anchorage_point, angle }
    }

    pub fn obtain_nearest_intersection(&self, spheres: &Vec<Sphere>) -> (f32,usize) {
        // Filter objects to only those that are partially or completelly inside cone
        let filtered_objects: Vec<&Sphere> = spheres.iter().filter(|sphere| {
            let z = sphere.center.z;
            let x = sphere.center.x;
            let y = sphere.center.y;

            // filters
            let mut is_z_in_range = z + sphere.radius < self.anchorage_point.z;
            is_z_in_range = is_z_in_range && z > self.anchorage_point.z - 100.0;
            let is_x_y_smaller = (x.powf(2.0) + y.powf(2.0))*self.angle.cos().powf(2.0) <= z.powf(2.0)*self.angle.sin().powf(2.0);
            is_z_in_range && is_x_y_smaller
        }).collect();

        // Obtain sphere closest to anchorage point
        filtered_objects.iter().map(|sphere| {((sphere.center.z - self.anchorage_point.z).abs(),sphere.id)}).reduce(|(past_distance,past_id), (new_distance, new_id)| {
            if new_distance < past_distance {
                (new_distance,new_id)
            } else {
                (past_distance,past_id)
            }
        }).unwrap()
    }
}