use std::{fs::File,io::{BufReader, BufRead}};
use std::collections::HashMap;
use cgmath::{Matrix4, Vector3};
use super::super::{Drawable, from_obj::FromObj, Binder};
use crate::HighlightableVertices;

/// # General Information
/// 
/// Representation of a plane figure. Contains information to draw it to screen and move/rotate it to final position
/// 
/// # Fields
/// 
/// * `vertices` -  Vertices in 3d space. Normally used in triads. Specified in gl configuration.
/// * `triangles` - Indices that map to vertices. Normally used in triads. Specified in gl configuration.
/// * `ignored_coordinate` - 2D Mesh should ignore one entry: The one which is the same in all of .obj vertex specification.
/// * `max_length` - Maximum length of figure. Used to center camera arround mesh.
/// * `model_matrix` - Translates and rotates object to final world position.
/// * `binder` - vao, vbo and ebo GPU variables bound to mesh drawable in GPU.
/// 
pub struct Mesh2D {
    pub vertices: Vec<f32>,
    pub triangles: Vec<u32>, 
    pub ignored_coordinate: usize,
    pub max_length: f32,
    pub model_matrix: Matrix4<f32>,
    binder: Binder,
}

impl Drawable for Mesh2D {

    fn get_triangles(&self) -> &Vec<u32> {
        &self.triangles
    }

    fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    fn get_max_length(&self) -> f32 {
        self.max_length
    }
    
    fn get_binder(&self) -> &Binder {
        &self.binder
    }
}

// Generate mesh from .obj file
impl FromObj for Mesh2D {}
// Generate spheres to identify vertices of mesh
impl HighlightableVertices for Mesh2D {

    fn get_model_matrix(&self) -> &Matrix4<f32> {
        &self.model_matrix
    }
    
}


impl Mesh2D {

    /// # General Information
    /// 
    /// 
    /// 
    /// # Parameters
    /// 
    /// 
    /// 
    pub fn new(file: &str) -> Mesh2D {

        // First the integrity of .obj file is checked
        let ignored_coordinate = Mesh2D::get_ignored_coordinate(file);

        // Obtained coordinates from 'generate_fields()' function
        let (
            vertices,
            triangles,
            max_length,
            closest_point
        ) = Mesh2D::generate_fields(
            &file,
            Some(ignored_coordinate)
        );

        // Translate matrix to given point
        let model_matrix = Matrix4::from_translation(Vector3::new(
            closest_point[0] as f32,
            closest_point[1] as f32,
            0.0
        ));

        // Binder
        let mut binder = Binder::new();
        // connect binder with gpu
        binder.setup();
        

        let mesh = Mesh2D {
            ignored_coordinate,
            vertices,
            triangles,
            max_length,
            model_matrix,
            binder,
        };

        mesh
    }

    pub fn get_ignored_coordinate(file: &str) -> usize {
        // Obtain unused coordinate index from .obj file.
        
        let file = File::open(file).expect("Error while opening the file. Does the file exists and is readdable?");
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
            0
        } else if y.values().count() == 1 {
            1
        } else if z.values().count() == 1 {
            2
        } else {
            panic!("Only coordinates over a plane paralell to x, y or z axis are accepted. Check .obj file.");
        }
    }
}

#[cfg(test)]
mod test {
    
}