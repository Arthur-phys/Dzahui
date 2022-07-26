use cgmath::{self, Matrix4, Deg, Vector3, Point3};
use crate::{drawable::Drawable, DzahuiWindow, HighlightableVertices};

#[derive(Debug)]
pub struct Camera {

    pub camera_direction: Vector3<f32>, // Change direction via euler angles without changing target.
    pub camera_position: Point3<f32>, // Camera position from original coordinate system (world coordinate system).
    pub camera_target:  Point3<f32>, // Camera target. Normally set to (0,0,0) but could change.
    pub view_matrix: Matrix4<f32>, // View matrix to change how camera sees objects.
    pub active_view_change: bool, // Yo know wether we can change view matrix.
    pub projection_matrix: Matrix4<f32>, // Matrix to see final results in screeen (it's normally a perspective matrix)
    pub up_vector: Vector3<f32>, // Vector to create a coordinate system for camera relative to it's position (position ends up in (0,0,0))
    pub camera_sensitivity: f32, // How much should camera get close when zooming
    pub theta: f32, // y axis - position angle to move camera
    pub phi: f32, // xz plane- position angle to move camera
    pub radius: f32, // length of camera position vector
    camera_speed: f32, // How fast should camera move
    aspect_ratio: f32, // Screen information to create projection matrix
    pitch: f32, // Euler angle: To look up and down
    pub near: f32, // How close should first plane be to camera for projection matrix
    pub far: f32, // How close should second plane be to camera for projection matrix
    fov: f32, // Angle of view
    yaw: f32, // Euler angle: to look left or right
}

impl Camera {

    pub fn new(mesh: &Box<dyn HighlightableVertices>, height: f32, width: f32) -> Camera {
        // The easier values to obtain from mesh
        // near is obtained from max_length
        let mut near: f32 = (mesh.get_max_length() * 2.0 - 50.0 ) as f32;
        if near <= 0.0 {
            near = 0.1;
        }
        // far is always 100 away from near
        let far: f32 = near + 100.0;
        // Predetermied values
        // fov is 45 degrees initially (so that tan(45)=1, therefore near = height of plane 1)
        let fov: f32 = 45.0;
        // Yaw starts camera looking straight (not left nor right)
        let yaw: f32 = -90.0;
        // Pitch starts starts camera looking straight (not up nor down)
        let pitch: f32 = 0.0;
        
        // sphere parametrization for camera position
        // raidus to make camera move around objective
        let radius = (mesh.get_max_length() * 2.0) as f32;
        // angles
        // y axis - position angle
        let theta: f32 = 90.0;
        // zx plane - position angle
        let phi: f32 = 0.0;

        // Camera speed starts at 0.5
        let camera_speed: f32 = 0.5;
        // Camera aspect_ratio
        let aspect_ratio: f32 = width/height;
        // Camera sensitivity for zoom and change direction starts at 0.5
        let camera_sensitivity: f32 = 0.5;
        // Up vector is always (0,0,1)
        let up_vector = Vector3::new(0.0,1.0,0.0);
        // Direction is obtained from yaw and pitch
        let camera_direction = Vector3::new(yaw.to_radians().cos()*pitch.to_radians().cos(),
        pitch.to_radians().sin(),yaw.to_radians().sin()*pitch.to_radians().cos());
        // Starts at near from first proyection plane
        let camera_position: Point3<f32> = Point3::new(theta.to_radians().sin()*phi.to_radians().sin(),
            theta.to_radians().cos(),theta.to_radians().sin()*phi.to_radians().cos()) * radius;
        // Starts looking at origin of coordinates
        let camera_target: Point3<f32> = Point3::new(0.0,0.0,0.0);
        // View and projection matrix
        // They are closely correlated, that's why they're both in the same structure.
        let view_matrix = Matrix4::look_at_rh(camera_position, camera_target, up_vector);
        let projection_matrix = cgmath::perspective(Deg(fov), aspect_ratio, near, far);
        // change view matrix defaults to false
        let active_view_change = false;

        Camera { camera_direction, camera_position, camera_target, up_vector, projection_matrix, theta, phi, radius,
            view_matrix, active_view_change, camera_sensitivity, camera_speed, aspect_ratio, pitch, near, far, fov, yaw}
    }

    pub fn modify_view_matrix(&mut self) {
        self.view_matrix = Matrix4::look_at_rh(self.camera_position, self.camera_target, self.up_vector);
    }

    pub fn modfy_projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(Vector3::new(0.0,0.0,0.0))
    }

    pub fn position_camera(&self, window: &DzahuiWindow) {
        // send new view matrix
        window.geometry_shader.set_mat4("view", &self.view_matrix);
    }


}