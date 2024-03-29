// Module declaration
pub mod cone;

// External dependencies
use cgmath::{self, Deg, Matrix4, Point3, Vector3};


/// # General Information
///
/// Camera struct. Makes movement arround viewport possible. Always uses projection matrix and moves arround a sphere given a target.
///
/// # Fields
///
/// * `camera_position` - Camera position from original coordinate system (world coordinate system).
/// * `camera_target` - Normally set to (0,0,0) but can change. What camera points at.
/// * `view_matrix` - How camera ends up viewing object.
/// * `active_view_change` - Wether we can change view matrix. Normally used in callback functions inside loop.
/// * `projection_matrix` - Perspective matrix to see final results in screen.
/// * `up_vector` - Vector to create a coordinate system for camera relative to it's position (position ends up in (0,0,0) in default mode).
/// * `camera_sensitivity` -  How much should camera get close when zooming and moving arround objective.
/// * `theta` - y axis - position angle to move camera.
/// * `phi` - xz plane - position angle to move camera.
/// * `radius` - how far away camera is from object.
///
#[derive(Debug)]pub(crate) struct Camera {
    pub(crate) camera_position: Point3<f32>,
    camera_target: Point3<f32>,
    pub(crate) view_matrix: Matrix4<f32>,
    pub(crate) active_view_change: bool,
    pub(crate) projection_matrix: Matrix4<f32>,
    up_vector: Vector3<f32>,
    pub(crate) camera_sensitivity: f32,
    pub(crate) theta: f32,
    pub(crate) phi: f32,
    pub(crate) radius: f32,
}

/// # General Information
///
/// The camera builder. Gives some control to user, such as distance from target, initial position arround target, fov, speed, sensitivity and
/// object being pointed at.
///
/// # Fields
///
/// * `radius` - Distance to target.
/// * `theta` - One of two angles that dictates camera position arround target (in a sphere).
/// * `phi` - One of two angles that dictates camera position arround target (in a sphere).
/// * `fov` - Field of view of projection matrix.
/// * `camera_sensitivity` - Speed at which camera moves arround target (in a sphere).
/// * `camera_target` - Point at which camera is looking.
///
#[derive(Default, Debug)]
pub struct CameraBuilder {
    pub(crate) radius: Option<f32>,
    theta: Option<f32>,
    phi: Option<f32>,
    fov: Option<f32>,
    camera_sensitivity: Option<f32>,
    camera_target: Option<Point3<f32>>,
}

impl CameraBuilder {
    /// Creates default CameraBuilder
    fn new() -> Self {
        CameraBuilder {
            radius: None,
            theta: None,
            phi: None,
            fov: None,
            camera_sensitivity: None,
            camera_target: None,
        }
    }
    /// Changes distance (radius) to object centered
    pub fn change_distance_to_object(self, radius: f32) -> Self {
        CameraBuilder {
            radius: Some(radius),
            ..self
        }
    }
    /// Changes object being targeted
    pub fn with_target(self, x: f32, y: f32, z: f32) -> Self {
        CameraBuilder {
            camera_target: Some(Point3::new(x, y, z)),
            ..self
        }
    }
    /// Changes initial camera position in a sphere with center `camera_target`
    pub fn with_camera_position(self, theta: f32, phi: f32) -> Self {
        CameraBuilder {
            theta: Some(theta),
            phi: Some(phi),
            ..self
        }
    }
    /// Changes fov when using projection matrix
    pub fn with_fov(self, fov: f32) -> Self {
        CameraBuilder {
            fov: Some(fov),
            ..self
        }
    }
    /// Changes camera movement sensitivity arround object being targeted
    pub fn with_sensitivity(self, sensitivity: f32) -> Self {
        CameraBuilder {
            camera_sensitivity: Some(sensitivity),
            ..self
        }
    }
    /// # General Information
    ///
    /// Builds a Camera from parameters given.
    ///
    /// # Details
    ///
    /// Camera moves arround a sphere (theta, phi, radius) centered on a point of an object with a given radius.
    /// Object on camera is projected on viewport via a projection matrix with a certain fov. There's no plan to add orthogonal projection.
    /// Camera sensitivity helps move camera around sphere.
    ///
    /// # Parameters
    ///
    /// * `self` - Every parameter appearing in Camera struct but not here is derived from the ones that do appear
    /// * `mesh_length` - Length of mesh to display in order to properly set radius if not given
    /// * `height` - Winow height
    /// * `width` - Window width
    ///
    pub(crate) fn build(self, mesh_length: f32, height: u32, width: u32) -> Camera {
        // Normal fov is 45 degrees
        let fov = if let Some(fov) = self.fov { fov } else { 45.0 };
        // Obtain radius or get predetermined one (use of the predetermined radius is recommended)
        let radius = if let Some(radius) = self.radius {
            radius
        } else {
            mesh_length * 2.0
        };
        // y axis - position angle
        let theta = if let Some(theta) = self.theta {
            theta
        } else {
            90.0
        };
        // zx plane - position angle
        let phi = if let Some(phi) = self.phi { phi } else { 0.0 };
        // It also works for zoom
        let camera_sensitivity = if let Some(camera_sensitivity) = self.camera_sensitivity {
            camera_sensitivity
        } else {
            0.5
        };
        // Up vector is always (0,1,0) (y goes upwards)
        let up_vector = Vector3::new(0.0, 1.0, 0.0);
        // Camera target. Normally leaving 0,0,0 is best, since object's center is translated to such point.
        let camera_target = if let Some(camera_target) = self.camera_target {
            camera_target
        } else {
            Point3::new(0.0, 0.0, 0.0)
        };

        // After obtaining values from builder:
        // The easier values to obtain from mesh are near and far
        // near is obtained from max_length ( or radius )
        let mut near = radius - 50.0;
        if near <= 0.0 {
            near = 0.1;
        }
        // far is always 100 away from near
        let far = near + 100.0;
        // Aspect ratio is obtained from height and width of viewport
        let aspect_ratio: f32 = width as f32 / height as f32;
        // Camera position is given by theta and phi (since it's a sphere)
        let camera_position: Point3<f32> = Point3::new(
            theta.to_radians().sin() * phi.to_radians().sin(),
            theta.to_radians().cos(),
            theta.to_radians().sin() * phi.to_radians().cos(),
        ) * radius
            + Vector3::new(camera_target.x, camera_target.y, camera_target.z);
        // View and projection matrix
        // They are closely related, that's why they're both in the same structure.
        let view_matrix = Matrix4::look_at_rh(camera_position, camera_target, up_vector);
        let projection_matrix = cgmath::perspective(Deg(fov), aspect_ratio, near, far);
        // Change view matrix defaults to false.
        let active_view_change = false;

        Camera {
            camera_position,
            camera_target,
            up_vector,
            projection_matrix,
            theta,
            phi,
            radius,
            view_matrix,
            active_view_change,
            camera_sensitivity,
        }
    }
}

impl Camera {
    /// Create a camera builder.
    pub fn builder() -> CameraBuilder {
        CameraBuilder::new()
    }

    /// Change view matrix associated to camera.
    pub(crate) fn modify_view_matrix(&mut self) {
        self.view_matrix =
            Matrix4::look_at_rh(self.camera_position, self.camera_target, self.up_vector);
    }
}
