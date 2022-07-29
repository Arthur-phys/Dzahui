use crate::{Drawable, FromObj, DzahuiWindow, Binder, Camera};
use cgmath::{Vector3, Matrix4, Vector4};
use std::ptr;

/// # General Information
/// 
/// Vertex structure represents a single selectable point in a mesh. It can also be used to contain data (like initial/boundary conditions).
/// 
/// # Fields
/// 
/// * `center` - the vertex point itself.
/// * `id` - Identifies uniquely a vertex.
/// 
#[derive(Debug)]
pub struct Vertex {
    pub center: Vector3<f32>,
    pub id: usize
}

impl Vertex {
    /// Creates new instance of Vertex.
    pub fn new(center: Vector3<f32>, radius: f32, id: usize) -> Self {
        Vertex {
            center,
            id
        }
    }
    /// Matrix to translate vertex to a given location (normally determined by a mesh instance).
    pub fn get_translation_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.center.clone())
    }
    /// Obtain center coordinates as viewed from camera
    pub fn get_view_center(&self, camera: &Camera) -> Vector3<f32> {
        let view_center = camera.view_matrix * Vector4::new(self.center.x,self.center.y,self.center.z,1.0);
        Vector3::new(view_center.x,view_center.y,view_center.z)
    }
}

/// # General Information
/// 
/// Vertex list. Represents all of the vertices in a given mesh. It also givess them a visible body for the user to interact with.
/// 
/// # Fields
/// 
/// * `triangles` - Indices to draw vertices' body.
/// * `vertices` - Vertices' body coordinates.
/// * `binder`- vao, vbo and ebo variables bound to mesh drawable in GPU.
/// * `size` - Vertices' body size.
/// * `list_of_vertices` - Vector of instances of Vertex.
/// * `scale_matrix` - Matrix to scale vertices' body.
/// 
pub struct VertexList {
    triangles: Vec<u32>,
    vertices: Vec<f32>,
    binder: Binder,
    size: f32,
    pub(crate) list_of_vertices: Vec<Vertex>,
    pub(crate) scale_matrix: Matrix4<f32>,
}

impl Drawable for VertexList {
    
    fn get_triangles(&self) -> &Vec<u32> {
        &self.triangles
    }

    fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    fn get_max_length(&self) -> f32 {
        self.size * 2.0
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
            
            for vertex in &self.list_of_vertices {
                // Obtaining final model matrix: translate + scale
                let model_mat = self.get_translation_matrix_from_id(vertex.id) * self.scale_matrix;
                // Sending to shader
                window.geometry_shader.set_mat4("model", &model_mat);
                // Drawing
                gl::DrawElements(gl::TRIANGLES,indices_len,gl::UNSIGNED_INT,ptr::null());
            }
        }
    }
}

impl FromObj for VertexList {}

impl VertexList {
    pub fn new(centers: Vec<Vector3<f32>>, size: f32, file: &str) -> Self {

        let list_of_vertices: Vec<Vertex> = centers.into_iter().enumerate().map(|(id,center)| {
            Vertex::new(center,size,id)
        }).collect();

        let (vertices, triangles, ..) = VertexList::generate_fields(file,
        None);
        let scale_matrix = Matrix4::from_scale(size);

        let mut binder = Binder::new();
        binder.setup();

        VertexList {
            list_of_vertices,
            binder,
            size,
            vertices,
            triangles,
            scale_matrix
        }
    }

    pub fn get_translation_matrix_from_id(&self, id: usize) -> Matrix4<f32> {
        self.list_of_vertices[id].get_translation_matrix()
    }
}
