pub mod mesh_2d;
pub mod mesh_3d;
pub mod sphere;

use super::Drawable;
use sphere::SphereList;
use cgmath::{Vector3, Matrix4};

pub trait HighlightableVertices: Drawable {

    fn create_highlightable_vertices(&self, radius: f32, file: &str) -> SphereList {
    
        let vertices = self.get_vertices();
        let centers: Vec<Vector3<f32>> = (0..vertices.len()).step_by(3).map(|i| {
            Vector3::new(vertices[i] as f32,vertices[i+1] as f32,vertices[i+2] as f32)
        }).collect();

        SphereList::new(centers, radius, file) 
    }

    /// Obtains model matrix for a drawable object. Getter
    fn get_model_matrix(&self) -> &Matrix4<f32>;
}

/// # General Information
/// 
/// Enum to tell if mesh being in a plane should be checked.
/// 
/// # Arms
/// 
/// * `Two` - Plane figure. Additional check-up to confirm property will be applied simplifying final mesh.
///  * `Three` - 3D Body. No check-ups are done. Results depend solely on user's .obj
pub enum MeshDimension<A: AsRef<str>> {
    Two(A),
    Three(A)
}