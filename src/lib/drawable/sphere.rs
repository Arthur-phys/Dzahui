use crate::{Drawable, FromObj, DzahuiWindow, Binder, Camera};
use cgmath::{Vector3, Matrix4, Point3, Vector4};
use std::ptr;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub id: usize
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, id: usize) -> Self {
        Sphere {center, radius, id}
    }

    pub fn get_translation_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.center.clone())
    }

    pub fn get_scale_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_scale(self.radius)
    }

    pub fn get_view_center(&self, camera: &Camera) -> Vector3<f32> {
        let view_center = camera.view_matrix * Vector4::new(self.center.x,self.center.y,self.center.z,1.0);
        Vector3::new(view_center.x,view_center.y,view_center.z)
    }
}

pub struct SphereList {
    // List of spheres
    pub spheres: Vec<Sphere>,
    // only one radius
    radius: f32,
    // Store spheres' render information here
    vertices: Vec<f64>,
    triangles: Vec<u32>,
    // to scale sphere to radius given
    pub scale_matrix: Matrix4<f32>
}

impl Drawable for SphereList {
    fn get_triangles(&self) -> &Vec<u32> {
        &self.triangles
    }
    fn get_vertices(&self) -> &Vec<f64> {
        &self.vertices
    }
    fn get_max_length(&self) -> f64 {
        (self.radius * 2.0) as f64
    }
    fn draw(&self, window: &DzahuiWindow, binder: &Binder) {
        let indices_len: i32 = self.get_triangles().len() as i32;
        // Draw only when window is created and inside loop
        // Drawn as triangles
        unsafe {
            gl::BindVertexArray(binder.vao);
            
            for sphere in &self.spheres {
                // Obtaining final model matrix: translate + scale
                let model_mat = self.get_translation_matrix_from_id(sphere.id) * self.scale_matrix;
                // Sending to shader
                window.shader.set_mat4("model", &model_mat);
                // Drawing
                gl::DrawElements(gl::TRIANGLES,indices_len,gl::UNSIGNED_INT,ptr::null());
            }
        }
    }
}

impl FromObj for SphereList {}

impl SphereList {
    pub fn new(centers: Vec<Vector3<f32>>, radius: f32, file: &str) -> Self {

        let spheres: Vec<Sphere> = centers.into_iter().enumerate().map(|(id,center)| {
            Sphere::new(center,radius,id)
        }).collect();

        let (vertices, triangles, ..) = SphereList::generate_fields(file,
        None);
        let scale_matrix = Matrix4::from_scale(radius);

        SphereList {spheres, radius, vertices, triangles, scale_matrix}
    }

    pub fn get_translation_matrix_from_id(&self, id: usize) -> Matrix4<f32> {
        self.spheres[id].get_translation_matrix()
    }
}
