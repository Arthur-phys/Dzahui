use cgmath::{Matrix4, Vector3};
use super::{Drawable, FromObj, HighlightableVertices};

pub struct Mesh3D {
    pub vertices: Vec<f64>, // Vertices in 3d space (normally used in triads, but that's specified in the gl configuration)
    pub triangles: Vec<u32>, // Indices that map to vertices (normally used in triads, but that's specified in the gl configuration)
    pub max_length: f64, // maximum length in x, y or z. To use with camera
    pub model_matrix: Matrix4<f32>, // matrix to translate mesh to middle point (only used once)
}

impl Drawable for Mesh3D {
    // Only need to implement getters for important methods to work
    fn get_triangles(&self) -> &Vec<u32> {
        &self.triangles
    }

    fn get_vertices(&self) -> &Vec<f64> {
        &self.vertices
    }

    fn get_max_length(&self) -> f64 {
        self.max_length
    }
}

impl FromObj for Mesh3D {}
impl HighlightableVertices for Mesh3D {}

impl Mesh3D {
    // New implementation differs in 3d and 2d because in one there has to be an ignored coordinate
    pub fn new(file: &str) -> Mesh3D {
        // Obtained coordinates from 'generate_coordinates()' function
        let (vertices, triangles, max_length, closest_point) = Mesh3D::generate_fields(
            &file, None);

        let model_matrix = Matrix4::from_translation(Vector3::new(closest_point[0] as f32,
            closest_point[1] as f32,closest_point[2] as f32));

        Mesh3D {
            vertices,
            triangles,
            max_length,
            model_matrix,
        }
    }
}

#[cfg(test)]
mod test {

}