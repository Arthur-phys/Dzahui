use cgmath::{self, Matrix4, Deg, Vector3, Point3, Matrix, InnerSpace};

pub struct Camera {
    pub camera_direction: Vector3<f64>,
    pub camera_position: Vector3<f64>,
    pub camera_target:  Vector3<f64>,
    pub up_vector: Vector3<f64>,
    camera_sensitivity: f64,
    camera_speed: f64,
    pitch: f64,
    near: f64,
    far: f64,
    yaw: f64,
    fov: f64,
}

impl Camera {

}