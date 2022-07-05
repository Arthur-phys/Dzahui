use std::{fs::File,io::{BufReader, BufRead, Seek}};
use std::collections::HashMap;
use cgmath::{Matrix4, Vector3};
use super::{Drawable, FromObj};

// Mesh should work for 2d and 3d
// Contains vertices and indices to generate triangles via gl
pub struct Mesh2D {
    pub vertices: Vec<f64>, // Vertices in 3d space (normally used in triads, but that's specified in the gl configuration)
    pub triangles: Vec<u32>, // Indices that map to vertices (normally used in triads, but that's specified in the gl configuration)
    pub ignored_coordinate: usize, // 2D Mesh should ignore one coordinate
    pub max_length: f64, // maximum length in x, y or z. To use with camera
    pub model_matrix: Matrix4<f32>, // matrix to translate mesh to middle point (only used once)
}

impl Drawable for Mesh2D {
    // Only need to implement getters for important methods to work
    fn get_triangles(&self) -> &Vec<u32> {
        &self.triangles
    }

    fn get_vertices(&self) -> &Vec<f64> {
        &self.vertices
    }
}

impl FromObj for Mesh2D {}


impl Mesh2D {
    // New implementation differs in 3d and 2d because in one there has to be an ignored coordinate
    pub fn new(file: &str) -> Mesh2D {
        // First the integrity of .obj file is checked
        let ignored_coordinate = Mesh2D::get_ignored_coordinate(file);

        // Obtained coordinates from 'generate_coordinates()' function
        let (vertices, triangles, max_length, closest_point) = Mesh2D::generate_fields(
            &file, Some(ignored_coordinate));

        let model_matrix = Matrix4::from_translation(Vector3::new(closest_point[0] as f32,
            closest_point[1] as f32,0.0));

        Mesh2D {
            ignored_coordinate,
            vertices,
            triangles,
            max_length,
            model_matrix,
        }
    }

    
    pub fn get_ignored_coordinate(file: &str) -> usize {
        if !file.ends_with(".obj") {
            panic!("File chosen does not match extension allowed")
        }
        let file = File::open(file).expect("Error while opening the file. Does the file exists and is readdable?");
        // Sets to check which one has only one element (i.e. which one should be ignored)
        // To implement set from list, use HashMap for better performance.
        let mut x: HashMap<String,f64> = HashMap::new();
        let mut y: HashMap<String,f64> = HashMap::new();
        let mut z: HashMap<String,f64> = HashMap::new();
        
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
                    let coordinates_vec: [(String,f64);3] = coordinates_iter.map(|c_str| {
                        // Necessary for -0.0 and 0.0 equality
                        if c_str.starts_with("0.0") || c_str.starts_with("-0.0") {
                            (String::from("0.0"),c_str.parse::<f64>().unwrap())
                        } else {
                            (c_str.to_string(),c_str.parse::<f64>().unwrap())
                        }
                    })
                    // Now the result is transformed into an array of tuples size 3
                    .into_iter().collect::<Vec<(String,f64)>>().try_into().expect(".obj's vertices should be composed of triads of numbers");
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