use cgmath::{self, Matrix4, Deg, Vector3, Point3, Matrix};
use crate::mesh::Mesh;

// Wrapper to convert vector to point when needed
// Use with caution
#[derive(Clone)]
#[derive(Debug)]
pub struct Vector3D<S> {
    vector : Vector3<S>
}

impl<S> Vector3D<S> {
    pub fn new(x:S,y:S,z:S) -> Self {
        let vector = Vector3::new(x,y,z);
        Vector3D { vector }
    }
}

impl<S> Into<Point3<S>> for Vector3D<S> {
    fn into(self) -> Point3<S> {
        let vec = self.vector;
        Point3 { x: vec.x, y: vec.y, z: vec.z }
    }
}
#[derive(Debug)]
pub struct Camera {

    pub camera_direction: Vector3D<f32>,
    pub camera_position: Vector3D<f32>,
    pub camera_target:  Vector3D<f32>,
    up_vector: Vector3<f32>,
    camera_sensitivity: f32,
    camera_speed: f32,
    pitch: f32,
    near: f32,
    far: f32,
    fov: f32,
    yaw: f32,
}

impl Camera {

    pub fn new(mesh: &Mesh) -> Camera {
        // The easier values to obtain from mesh
        // near is obtained from max_length
        let near: f32 = (mesh.max_length/2.0 - 50.0) as f32;
        // far is always 100 away from near
        let far: f32 = near + 100.0;
        // Predetermied values
        // fov is 45 degrees initially (so that tan(45)=1, therefore near = height of plane 1)
        let fov: f32 = 45.0;
        // Yaw starts camera looking straight (not left nor right)
        let yaw: f32 = -90.0;
        // Pitch starts starts camera looking straight (not up nor down)
        let pitch: f32 = 0.0;
        // Camera speed starts at 0.5
        let camera_speed: f32 = 0.5;
        // Camera sensitivity for zoom and change direction starts at 0.1
        let camera_sensitivity: f32 = 0.1;
        // Up vector is always (0,0,1)
        let up_vector = Vector3::new(0.0,0.0,1.0);
        // Direction is obtained from yaw and pitch
        let camera_direction = Vector3D::new(yaw.to_radians().cos()*pitch.to_radians().cos(),
        pitch.to_radians().sin(),yaw.to_radians().sin()*pitch.to_radians().cos());
        // Starts at near from first proyection plane
        let camera_position = Vector3D::new(0.0,0.0,near);
        // Starts looking at origin of coordinates
        let camera_target = Vector3D::new(0.0,0.0,0.0);
        Camera { camera_direction, camera_position, camera_target, up_vector, camera_sensitivity, camera_speed, pitch, near, far, fov, yaw}
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_lh(Point3::new(0.0,0.0,3.0), Point3::new(0.0,0.0,2.0), Vector3::new(0.0,0.0,1.0))
    }


}