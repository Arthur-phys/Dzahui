pub mod mesh_2d;
pub mod mesh_3d;
pub mod vertex;

use super::Drawable;
use vertex::VertexList;
use cgmath::{Vector3, Matrix4};
use mesh_2d::Mesh2D;
use mesh_3d::Mesh3D;

/// # General Information
/// 
/// Needed elements to create mesh (2D or 3D). Provides option to personalize vertices.
/// 
/// # Fields
/// 
/// * `location` - Path to mesh's `.obj`.
/// * `dimension` - Enum with mesh's dimension. Needs to be set to enable/disable checkoing for repeated coordinate in `.obj` if it's 2D.
/// * `vertex_body` - Allows vertex personalization if set.
/// 
pub struct MeshBuilder<A: AsRef<str>, B: AsRef<str>> {
    location: A,
    dimension: MeshDimension,
    vertex_body: Option<B>
}

impl<A: AsRef<str>, B: AsRef<str>> MeshBuilder<A,B> {
    
    /// Creates default instance.
    fn new(location: A, dimension: MeshDimension) -> Self {
        Self {
            location,
            dimension,
            vertex_body: None
        }
    }
    /// Obtains new vertex body to draw
    fn with_vertex_body(self, vertex_body: B) -> Self {
        Self {
            vertex_body: Some(vertex_body),
            ..self
        }
    }
    /// # General Information
    /// 
    /// ddd
    /// 
    /// # Parameters
    /// 
    /// ddd
    /// 
    fn build(self) -> Box<dyn HighlightableVertices> {
        
        // Creating mesh placed in box to accept both Mesh2D and Mesh3D
        let mesh: Box<dyn HighlightableVertices> = match self.dimension {
            MeshDimension::Two => Box::new(Mesh2D::new(self.location)),
            MeshDimension::Three => Box::new(Mesh3D::new(self.location))
        };
        mesh
    }
}

pub trait HighlightableVertices: Drawable {

    fn create_highlightable_vertices(&self, radius: f32, file: &str) -> VertexList {
    
        let vertices = self.get_vertices();
        let centers: Vec<Vector3<f32>> = (0..vertices.len()).step_by(3).map(|i| {
            Vector3::new(vertices[i] as f32,vertices[i+1] as f32,vertices[i+2] as f32)
        }).collect();

        VertexList::new(centers, radius, file) 
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
pub enum MeshDimension {
    Two,
    Three
}