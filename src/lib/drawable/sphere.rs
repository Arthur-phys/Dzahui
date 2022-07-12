use cgmath::{Vector3, Matrix4};
use crate::{Drawable, FromObj, DzahuiWindow, Binder};
use std::ptr;

pub struct SphereList {
    // List of center points
    pub centers: Vec<Vector3<f32>>,
    // Unique radius for all spheres
    radius: f32,
    // Vector of id's
    pub ids: Vec<usize>,
    // Store sphere render information here
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
}

impl FromObj for SphereList {}

impl SphereList {
    pub fn new(centers: Vec<Vector3<f32>>, radius: f32, file: &str) -> Self {

        let ids: Vec<usize> = (0..centers.len()).collect();

        let (vertices, triangles, ..) = SphereList::generate_fields(file,
        None);
        let scale_matrix = Matrix4::from_scale(radius);

        SphereList {centers, radius, ids, vertices, triangles, scale_matrix}
    }

    pub fn get_translation_matrix_from_id(&self, id: usize) -> Matrix4<f32> {

        let center = &self.centers[id];
        Matrix4::from_translation(center.clone())
    }

    pub fn draw_list(&self, window: &DzahuiWindow, binder: &Binder) {
        let indices_len: i32 = self.get_triangles().len() as i32;
        // Draw only when window is created and inside loop
        // Drawn as triangles
        unsafe {
            for i in &self.ids {
                // Obtaining final model matrix: translate + scale
                let model_mat = self.get_translation_matrix_from_id(*i) * self.scale_matrix;
                // Sending to shader
                window.shader.set_mat4("model", &model_mat);
                // Drawing
                gl::BindVertexArray(binder.vao);
                gl::DrawElements(gl::TRIANGLES,indices_len,gl::UNSIGNED_INT,ptr::null());
            }
        }
    }
}
