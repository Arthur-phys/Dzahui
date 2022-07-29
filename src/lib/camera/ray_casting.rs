use cgmath::{Vector3, Vector4, Point3, Point2, Transform, Matrix4, InnerSpace};
use crate::{Camera, DzahuiWindow, drawable::mesh::vertex::Vertex};

#[derive(Debug)]
pub struct Cone {
    anchorage_point: Point3<f32>,
    direction: Vector3<f32>,
    angle: f32
}

impl Cone {
    pub fn new(anchorage_point: Point3<f32>, direction: Vector3<f32>, angle: f32) -> Cone {
        let direction = direction.normalize();
        Cone { anchorage_point, direction, angle }
    }

    pub fn from_mouse_position(angle: f32, mouse_coordinates: Point2<f64>, camera: &Camera, window: &DzahuiWindow) -> Cone {
        // Create cone from position of mouse
        let near_ndc_coordinates = Vector4::new(
            (mouse_coordinates.x - (window.width as f64)/2.0)/((window.width as f64)/2.0), // map between -1 and 1
            -(mouse_coordinates.y - (window.height as f64)/2.0)/((window.height as f64)/2.0),
            -1.0, // near plane
            1.0
        );
        let far_ndc_coordinates = Vector4::new(
            (mouse_coordinates.x - (window.width as f64)/2.0)/((window.width as f64)/2.0), // map between -1 and 1
            -(mouse_coordinates.y - (window.height as f64)/2.0)/((window.height as f64)/2.0),
            1.0, // far plane
            1.0
        );

        let inverse_projection_matrix: Matrix4<f64> = camera.projection_matrix.inverse_transform().expect("No inverse transform exists for this matrix").cast().unwrap();
        let near_view_coordinates = inverse_projection_matrix * near_ndc_coordinates;
        let far_view_coordinates = inverse_projection_matrix * far_ndc_coordinates;
        
        // need to divide by w (god knows why)
        let near_view_coordinates = Vector3::new(near_view_coordinates.x,near_view_coordinates.y,near_view_coordinates.z) / near_view_coordinates.w;
        let far_view_coordinates = Vector3::new(far_view_coordinates.x,far_view_coordinates.y,far_view_coordinates.z) / far_view_coordinates.w;
        
        // cast becomes unnecesary if all values are f64, but camera is not working with f64 and opengl
        let anchorage_point: Point3<f32> = Point3::new(near_view_coordinates.x,near_view_coordinates.y, near_view_coordinates.z).cast().unwrap();
        let direction: Vector3<f32> = (far_view_coordinates - near_view_coordinates).cast().unwrap().normalize();

        Cone {anchorage_point, direction, angle}
    }

    pub fn obtain_nearest_intersection(&self, spheres: &Vec<Vertex>, camera: &Camera) -> Option<(f32,usize)> {
        // Filter objects to only those that are partially or completelly inside cone
        let filtered_objects: Vec<&Vertex> = spheres.iter().filter(|sphere| {
            let view_center = sphere.get_view_center(&camera);
            let x = view_center.x;
            let y = view_center.y;
            let z = view_center.z;
            // filters
            let mut is_z_in_range = z < self.anchorage_point.z;
            is_z_in_range = is_z_in_range && z > self.anchorage_point.z - 100.0;

            // obtaining values for circle center of cone
            // first obtain t from equation f(t) = p + tv
            // z direction can never be zero
            let curve_value_from_z = (z - self.anchorage_point.z)/self.direction.z;
            // then obtain x and y from such t
            // this generates circle center
            let c_x = self.anchorage_point.x + curve_value_from_z * self.direction.x;
            let c_y = self.anchorage_point.y + curve_value_from_z * self.direction.y;
            // obtain radius of circunference via angle and distance to anchorage point
            let c_r = ((c_x - self.anchorage_point.x).powf(2.0) + (c_y - self.anchorage_point.y).powf(2.0) + (z - self.anchorage_point.z).powf(2.0)).sqrt() * self.angle.to_radians().tan();

            // check inequalities for circle
            let circle_ineq = (c_x - x).powf(2.0) + (c_y - y).powf(2.0) <= c_r.powf(2.0);
            
            is_z_in_range && circle_ineq
        }).collect();

        // Obtain sphere closest to anchorage point
        filtered_objects.iter().map(|sphere| {
            let view_center_z =sphere.get_view_center(camera).z;
            ((view_center_z - self.anchorage_point.z).abs(),sphere.id)
        
        }).reduce(|(past_distance,past_id), (new_distance, new_id)| {
        
            if new_distance < past_distance {
                (new_distance,new_id)
            } else {
                (past_distance,past_id)
            }
        })
    }
}