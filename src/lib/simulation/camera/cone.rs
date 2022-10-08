use cgmath::{Vector3, Vector4, Point3, Point2, Transform, Matrix4, InnerSpace};
use ndarray::{Array1,Axis, ArrayView1};

/// # General Information
/// 
/// A cone firstly serves as an object with 'ray casting'-like functionality to be able to click elements from screen.
/// It transforms viewport coordinates to object coordinates and projects a cone inside this viewport. Then, an intersectrion can ben done to obtain closest
/// object and return it to the user to do something.
/// 
/// # Fields
/// 
/// * `anchorage_point` - Point from where the cone starts. Normally coincides with near plane of camera.
/// * `direction` - Direction the cone will take. Given mouse position, a line is determined inside viewport. Near and far coordinates are determined and
/// the direction vector of the line generated (by near and far points) is used.
/// * `angle` - How much should the cone be open.
/// 
#[derive(Debug)]
pub(crate) struct Cone {
    anchorage_point: Point3<f32>,
    direction: Vector3<f32>,
    angle: f32
}

impl Cone {

    /// Function to create new instance. Normalizes direction vector. 
    pub(crate) fn new(anchorage_point: Point3<f32>, direction: Vector3<f32>, angle: f32) -> Cone {
        let direction = direction.normalize();
        Cone { anchorage_point, direction, angle }

    }
    
    /// # General Information
    /// 
    /// Change cone given mouse input.
    /// This function is normally used along cone and window. Steps reffered to in struct definition, are performed here.
    ///
    /// # Parameters
    /// 
    /// * `mouse_coordinates` - viewport coordinates to change anchorage point.
    /// * `projection_matrix` - camera projection matrix to find reverse transformation.
    /// * `window_width` - original window width needed to normalize viewport coordinates
    /// * `window_height` - original window height needed to normalize viewport coordinates
    /// 
    pub(crate) fn change_from_mouse_position(&mut self, mouse_coordinates: &Point2<f32>, projection_matrix: &Matrix4<f32>, window_width: u32, window_height: u32) {

        // Create cone from position of mouse
        let near_ndc_coordinates = Vector4::new(
            (mouse_coordinates.x - (window_width as f32)/2.0)/((window_width as f32)/2.0), // map between -1 and 1
            -(mouse_coordinates.y - (window_height as f32)/2.0)/((window_height as f32)/2.0),
            -1.0, // near plane
            1.0
        );
        let far_ndc_coordinates = Vector4::new(
            (mouse_coordinates.x - (window_width as f32)/2.0)/((window_width as f32)/2.0), // map between -1 and 1
            -(mouse_coordinates.y - (window_height as f32)/2.0)/((window_height as f32)/2.0),
            1.0, // far plane
            1.0
        );

        let inverse_projection_matrix: Matrix4<f32> = projection_matrix.inverse_transform().expect("No inverse transform exists for this matrix");
        let near_view_coordinates = inverse_projection_matrix * near_ndc_coordinates;
        let far_view_coordinates = inverse_projection_matrix * far_ndc_coordinates;
        
        // need to divide by w (god knows why)
        let near_view_coordinates = Vector3::new(near_view_coordinates.x,near_view_coordinates.y,near_view_coordinates.z) / near_view_coordinates.w;
        let far_view_coordinates = Vector3::new(far_view_coordinates.x,far_view_coordinates.y,far_view_coordinates.z) / far_view_coordinates.w;
        
        let anchorage_point: Point3<f32> = Point3::new(near_view_coordinates.x,near_view_coordinates.y, near_view_coordinates.z);
        let direction: Vector3<f32> = (far_view_coordinates - near_view_coordinates).normalize();

        self.anchorage_point = anchorage_point;
        self.direction = direction;
    }

    /// Matrix to translate vertex to a given location (normally determined by a mesh instance).
    fn get_translation_matrix(arr: &Array1<f32>) -> Matrix4<f32> {
        let vec_arr = Vector3::new(arr[0] as f32,arr[1] as f32,arr[2] as f32);
        Matrix4::from_translation(vec_arr)
    }

    /// Obtain center coordinates as viewed from camera
    fn get_view_center(arr: &ArrayView1<f64>, view_matrix: &Matrix4<f32>) -> Vector3<f32> {
        let vec_arr = Vector4::new(arr[0] as f32,arr[1] as f32,arr[2] as f32,1.0);
        let view_center = view_matrix * vec_arr;
        Vector3::new(view_center.x,view_center.y,view_center.z)
    }

    /// # General Information
    /// 
    /// Determine closest intersection given some vertices (sextuples of points) and current cone status. Only one vertex is returned with id.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - To determine cone location.
    /// * `vertices` - Vertices check wether they're inside or outside the cone
    /// * `view_matrix` - Camera view matrix needed to see where a vertex is (in view space).
    /// 
    pub(crate) fn obtain_nearest_intersection(&self, vertices: &Array1<f64>, view_matrix: &Matrix4<f32>) -> Option<(f32,usize)> {

        // Filter objects to only those that are partially or completelly inside cone
        let dim_1 = vertices.len() / 3;
        let reshaped_vertices = vertices.to_shared().reshape((dim_1,3));
        let filtered_objects: Vec<ArrayView1<f64>> = reshaped_vertices.axis_iter(Axis(0)).filter(|vertex| {
            let view_center = Cone::get_view_center(vertex,view_matrix);
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
            // then obtain x and y from t
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
        filtered_objects.iter().enumerate().map(|(id,vertex)| {
            let view_center_z = Cone::get_view_center(vertex,view_matrix).z;
            ((view_center_z - self.anchorage_point.z).abs(),id)
        
        }).reduce(|(past_distance,past_id), (new_distance, new_id)| {
        
            if new_distance < past_distance {
                (new_distance,new_id)
            } else {
                (past_distance,past_id)
            }
        })
    }
}