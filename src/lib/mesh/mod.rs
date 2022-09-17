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
    pub conditions: Vec<VertexType>,
    pub ignored_coordinate: Option<usize>,
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

    pub fn get_ignored_coordinate<A: AsRef<str>>(file: A) -> Option<usize> {
        // Obtain unused coordinate index from .obj file.
        
        let file = File::open(file.as_ref()).expect("Error while opening the file. Does the file exists and is readable?");
        // Sets to check which one has only one element (i.e. which one should be ignored)
        // To implement set from list, use HashMap for better performance.
        let mut x: HashMap<String,f32> = HashMap::new();
        let mut y: HashMap<String,f32> = HashMap::new();
        let mut z: HashMap<String,f32> = HashMap::new();
        
        // Filtering lines based on them starting with 'v ' or not. These are the ones we're suppossed to check
        let lines = BufReader::new(file).lines().filter(|line| {
            match line {
                Ok(content) => content.starts_with("v "),
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }
        });
        
        // Every line is treated individually
        lines.for_each(|line| {
            
            match line {
                Ok(coordinates) => {
                    // splitting via space
                    let mut coordinates_iter = coordinates.split(" ");
                    // skip the 'v'
                    coordinates_iter.next();
                    // mapping to tuple for HashMap
                    let coordinates_vec: [(String,f32);3] = coordinates_iter.map(|c_str| {
                        // Necessary for -0.0 and 0.0 equality
                        if c_str.starts_with("0.0") || c_str.starts_with("-0.0") {
                            (String::from("0.0"),c_str.parse::<f32>().unwrap())
                        } else {
                            (c_str.to_string(),c_str.parse::<f32>().unwrap())
                        }
                    })
                    // Now the result is transformed into an array of tuples size 3
                    .into_iter().collect::<Vec<(String,f32)>>().try_into().expect(".obj's vertices should be composed of triads of numbers");
                    // Inserting into HashMap
                    // Do not use clone, find replacement if possible (String needs cloning because of ownership)
                    x.insert(coordinates_vec[0].0.clone(),coordinates_vec[0].1);
                    y.insert(coordinates_vec[1].0.clone(),coordinates_vec[1].1);
                    z.insert(coordinates_vec[2].0.clone(),coordinates_vec[2].1);
                },
                // Error case of line matching
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }
        });
        // After for_each, we verify which coordinate is constant
        if x.values().count() == 1 {
            Some(0)
        } else if y.values().count() == 1 {
            Some(1)
        } else if z.values().count() == 1 {
            Some(2)
        } else {
            panic!("Only coordinates over a plane paralell to x, y or z axis are accepted. Check .obj file.");
        }
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
    
    #[test]
    fn verify_coordinates_mesh() {
        let y = Mesh::get_ignored_coordinate("/home/Arthur/Tesis/Dzahui/assets/untitled.obj");
        assert!(y == Some(1));
    }
    
    #[test]
    fn parse_coordinates() {
    
        let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj").build();
        assert!(new_mesh.vertices == vec![-1.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0]);
        assert!(new_mesh.indices == vec![0,1,2]);
    }
    
    #[test]
    fn is_max_distance() {
    
        let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj").build();
        println!("{}",new_mesh.max_length);
        assert!(new_mesh.max_length >= 1.90);
        assert!(new_mesh.max_length <= 2.10);
    }
}