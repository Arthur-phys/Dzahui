pub(crate) mod mesh_builder;
pub(crate) mod vertex_type;

use ndarray::Array1;
use cgmath::Matrix4;
use num::ToPrimitive;

use crate::{simulation::drawable::{binder::Binder,Drawable,Bindable}, Error};
use mesh_builder::MeshBuilder;
use vertex_type::VertexType;

/// # General Information
/// 
/// Representation of a plane figure/ body. Contains information to draw to screen and move/rotate it to final position.
/// 
/// # Fields
/// 
/// ## Numerical Integration Fields
/// 
/// * `conditions` - Kind of vertex a given triad is. Can be a boundary or internal vertex. Also contains the possible initial/boundary condition.
/// 
/// ## Drawing Fields
/// 
/// * `max_length` - Maximum length of figure. Used to center camera arround objective.
/// * `model_matrix` - Translates and rotates object to final world position.
/// * `binder` - vao, vbo and ebo variables bound to mesh drawable in GPU.
/// * `indices` - Indices that map to vertices. Normally used in triads. Specified in gl configuration.
///
/// ## Shared Fields
///
/// * `vertices` -  Vertices in 3d space. Normally used in triads. Specified in gl configuration.
///
#[derive(Debug)]
pub(crate) struct Mesh {
    pub(crate) conditions: Array1<VertexType>,
    pub(crate) max_length: f64,
    pub(crate) model_matrix: Matrix4<f32>,
    binder: Binder,
    pub(crate) indices: Array1<u32>,
    pub(crate) vertices: Array1<f64>,
}

impl Mesh {

    /// Getter for model_matrix
    pub fn get_model_matrix(&self) -> &Matrix4<f32> {
        &self.model_matrix
    }

    /// Creates new instance of builder
    pub fn builder<B>(location: B) -> MeshBuilder
    where B: AsRef<str> {
        MeshBuilder::new(location)
    }

    //
    pub(crate) fn filter_for_solving_1d(&self) -> Array1<f64> {
        // size of vertex is 6. There are double the vertices in 1d since a new pair is generated to draw a bar, therefore len is divided by 12.
        let vertices_len = self.vertices.len() / 12;
        self.vertices.iter().enumerate().filter_map(|(idx,x)| {if idx % 6 == 0 && idx < vertices_len*6 {Some(*x)} else {None}}).collect()
    }
}


impl Bindable for Mesh {
    fn get_binder(&self) -> Result<&Binder,Error> {
        Ok(&self.binder)
    }

    fn get_mut_binder(&mut self) -> Result<&mut Binder,Error> {
        Ok(&mut self.binder)
    }
}

impl Drawable for Mesh {

    fn get_triangles(&self) -> Result<&Array1<u32>,Error> {
        Ok(&self.indices)
    }

    fn get_vertices(&self) -> Result<Array1<f32>,Error> {
        Ok(Array1::from_iter(self.vertices.iter().map(|x| x.to_f32().unwrap())))
    }

    fn get_max_length(&self) -> Result<f32,Error> {
        let max_len = self.max_length.to_f32();
        
        match max_len {
            Some(f) => {
                if f.is_finite() {
                    Ok(f)
                } else {
                    Err(Error::Overflow)
                }
            },
            None => Err(Error::Unimplemented)
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::Mesh;
//     use ndarray::Array1;
    
//     #[test]
//     fn parse_coordinates() {
    
//         let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj").build().unwrap();
//         assert!(new_mesh.vertices == Array1::from_vec(vec![-1.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0]));
//         assert!(new_mesh.indices == Array1::from_vec(vec![0,1,2]));
//     }
    
//     #[test]
//     fn is_max_distance() {
    
//         let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj").build().unwrap();
//         println!("{}",new_mesh.max_length);
//         assert!(new_mesh.max_length >= 1.90);
//         assert!(new_mesh.max_length <= 2.10);
//     }
// }