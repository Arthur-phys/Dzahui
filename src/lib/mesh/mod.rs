pub(crate) mod mesh_builder;
pub(crate) mod vertex_type;

use std::{fs::File,io::{BufReader, BufRead}};
use std::collections::HashMap;
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
/// * `ignored_coordinate` - 2D Mesh should ignore one entry: The one which is the same in all of .obj vertex specification.
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
pub struct Mesh {
    pub conditions: Array1<VertexType>,
    pub max_length: f64,
    pub model_matrix: Matrix4<f32>,
    binder: Binder,
    pub indices: Array1<u32>,
    pub vertices: Array1<f64>,
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
}


impl Bindable for Mesh {
    fn get_binder(&self) -> &Binder {
        &self.binder
    }

    fn get_mut_binder(&mut self) -> &mut Binder {
        &mut self.binder
    }
}

impl Drawable for Mesh {

    fn get_triangles(&self) -> &Array1<u32> {
        &self.indices
    }

    fn get_vertices(&self) -> Array1<f32> {
        Array1::from_iter(self.vertices.iter().map(|x| x.to_f32().unwrap()))
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

#[cfg(test)]
mod test {
    use super::Mesh;
    use ndarray::Array1;
    
    #[test]
    fn parse_coordinates() {
    
        let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj").build();
        assert!(new_mesh.vertices == Array1::from_vec(vec![-1.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0]));
        assert!(new_mesh.indices == Array1::from_vec(vec![0,1,2]));
    }
    
    #[test]
    fn is_max_distance() {
    
        let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj").build();
        println!("{}",new_mesh.max_length);
        assert!(new_mesh.max_length >= 1.90);
        assert!(new_mesh.max_length <= 2.10);
    }
}