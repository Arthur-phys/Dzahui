use crate::{Drawable, FromObj, DzahuiWindow, Binder, Camera};
use cgmath::{Vector3, Matrix4, Vector4};
use num::Float;
use std::ptr;

/// # General Information
/// 
/// C
#[derive(Debug)]
pub struct Vertex {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub id: usize
}

impl Vertex {
    pub fn new(center: Vector3<f32>, radius: f32, id: usize) -> Self {
        Vertex {center, radius, id}
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
    pub(crate) spheres: Vec<Vertex>,
    // only one radius
    radius: f32,
    // Store spheres' render information here
    vertices: Vec<f32>,
    triangles: Vec<u32>,
    // to scale sphere to radius given
    pub(crate) scale_matrix: Matrix4<f32>,
    binder: Binder
}

impl Drawable for SphereList {
    
    fn get_triangles(&self) -> &Vec<u32> {
        &self.triangles
    }

    fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    fn get_max_length(&self) -> f32 {
        (self.radius * 2.0)
    }

    fn get_binder(&self) -> &Binder {
        &self.binder
    }

    fn draw(&self, window: &DzahuiWindow) {
        let indices_len: i32 = self.get_triangles().len() as i32;
        // Draw only when window is created and inside loop
        // Drawn as triangles
        unsafe {
            gl::BindVertexArray(self.get_binder().vao);
            
            for sphere in &self.spheres {
                // Obtaining final model matrix: translate + scale
                let model_mat = self.get_translation_matrix_from_id(sphere.id) * self.scale_matrix;
                // Sending to shader
                window.geometry_shader.set_mat4("model", &model_mat);
                // Drawing
                gl::DrawElements(gl::TRIANGLES,indices_len,gl::UNSIGNED_INT,ptr::null());
            }
        }
    }
}

impl FromObj for SphereList {}

impl SphereList {
    pub fn new(centers: Vec<Vector3<f32>>, radius: f32, file: &str) -> Self {

        let spheres: Vec<Vertex> = centers.into_iter().enumerate().map(|(id,center)| {
            Vertex::new(center,radius,id)
        }).collect();

        let (vertices, triangles, ..) = SphereList::generate_fields(file,
        None);
        let scale_matrix = Matrix4::from_scale(radius);

        let mut binder = Binder::new();
        binder.setup();

        SphereList {
            spheres,
            binder,
            radius,
            vertices,
            triangles,
            scale_matrix
        }
    }

    pub fn get_translation_matrix_from_id(&self, id: usize) -> Matrix4<f32> {
        self.spheres[id].get_translation_matrix()
    }
}
