use std::io::{BufReader,BufRead};
use cgmath::{Matrix4,Vector3};
use ndarray::Array1;
use std::collections::HashMap;
use std::fs::File;

use crate::{simulation::drawable::binder::Binder,Error};
use super::{Mesh, vertex_type::VertexType};

/// # General Information
/// 
/// Enum to tell if mesh being in a plane should be checked.
/// 
/// # Arms
/// 
/// * `Two` - Plane figure. Additional check-up to confirm property will be applied simplifying final mesh.
///  * `Three` - 3D Body. No dimensional check-ups are done. Results depend solely on user's .obj
/// 
#[derive(Debug)]
pub enum MeshDimension {
    Two,
    Three
}

/// # General Information
/// 
/// Needed elements to create mesh (2D or 3D). Provides option to personalize vertices.
/// 
/// # Fields
/// 
/// * `location` - Path to mesh's `.obj`.
/// * `dimension` - Enum with mesh's dimension. Needs to be set to enable/disable checking for repeated coordinate in `.obj` if it's 2D.
///
#[derive(Debug)]
pub struct MeshBuilder {
    location: String,
    dimension: MeshDimension
}

impl MeshBuilder {
    
    /// Creates default/initial instance.
    pub(crate) fn new<B>(location: B) -> Self
    where B: AsRef<str> {
        Self {
            location: location.as_ref().to_string(),
            dimension: MeshDimension::Two,
        }
    }
    /// Changes mesh dimension.
    pub(crate) fn with_mesh_in_3d(self) -> Self {
        Self {
            dimension: MeshDimension::Three,
            ..self
        }
    }

    /// Checks wether a line in an obj has only three vertices.
    /// Part of the checkup made to a given input file.
    fn obj_vertex_checker<A>(line: A) -> Result<bool,Error>
    where A: AsRef<str> { 
        
        let line_parts: Vec<&str> = line.as_ref().split(" ").collect();
        if line_parts.len() != 4 {
           return Err(Error::Parse("A vertex line should contain 3 vertices only".to_string()));
        }
        Ok(true)
    }
    
    /// Verifies the amount of face specifications per line is 3 and also that all of them have the correct syntax.
    /// Part of the checkup made to a given input file.
    fn obj_face_checker<A>(line: A) -> Result<bool, Error>
    where A: AsRef<str> {

        let line_parts: Vec<&str> = line.as_ref().split(" ").collect();
        // Check lenght of line
        if line_parts.len() != 4 {
            return Err(Error::Parse("Amount of face specificating elements should be 3.".to_string()));
        }
        // Check for each part structur /a/b/c
        for face in line_parts {
            let face_parts: Vec<&str> = face.split("/").collect();
            if face_parts.len() != 3 {
                return Err(Error::Parse("Amount of elements per face specification should be 3 in format a/b/c.".to_string()));
            }
        }
        Ok(true)
    }

    /// # General information
    /// 
    /// When a mesh is set to 2D, a verification is made on the file, ensuring that one of three coordinates is effectively zero.
    /// Verifying returns the coordinate that is zero.
    /// Later on, when reading the file again, coordinates are switched so that z-coordinate becomes the zero coordinate, regardless of it's previous role in .obj, 
    /// and the original zero coordinate becomes populated with the z-coordinate values.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - Only the fiule within self.¿ is needed to make the verification. 
    /// 
    fn ignored_coordinate(&self) -> Option<usize> {
        
        let file = File::open(self.location).expect("Error while opening the file. Does the file exists and is readable?");
        
        // Sets to check which one has only one element (i.e. which one should be ignored)
        // To implement set from list, use HashMap for better performance.
        let mut x: HashMap<String,f32> = HashMap::new();
        let mut y: HashMap<String,f32> = HashMap::new();
        let mut z: HashMap<String,f32> = HashMap::new();
        
        
        // Every line is treated individually
        BufReader::new(file).lines().for_each(|line| {
            
            match line {
                Ok(coordinates) => {
                
                    if coordinates.starts_with("v ") {

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

                    } else {

                    }
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

    /// Gets biggest distance from hashmap with specific entries related to farthest values in a mesh.
    fn compare_distances(max_min: &HashMap<&str,f64>) -> f64 {

        let x_min = max_min.get("x_min").unwrap();
        let y_min = max_min.get("y_min").unwrap();
        let z_min = max_min.get("z_min").unwrap();
        let x_max = max_min.get("x_max").unwrap();
        let y_max = max_min.get("y_max").unwrap();
        let z_max = max_min.get("z_max").unwrap();
        
        let d_x = *x_max-*x_min;
        let d_y = *y_max-*y_min;
        let d_z = *z_max-*z_min;

        if d_x >= d_y && d_x >= d_z {
            d_x
        } else if d_y >= d_z && d_y >= d_x {
            d_y
        } else {
            d_z
        }
    }

    /// Obtains variables from .obj. To use after file check.
    fn get_vertices_indices_and_conditions(&self, ignored_coordinate: Option<usize>) -> (Array1<f64>,Array1<u32>,Array1<VertexType>,f64,[f64;3]) {

        // Initial variables
        let mut coordinates: Array1<f64> = Array1::from_vec(vec![]);
        let mut triangles: Array1<u32> = Array1::from_vec(vec![]);

        let file = File::open(self.location).expect("Error while opening file. Does file exists and is readable?");

        // Coordinates to calculate max length and closest element to 0
        let mut max_min = HashMap::from([
            ("x_min",0.0),
            ("y_min",0.0),
            ("z_min",0.0),
            ("x_max",0.0),
            ("y_max",0.0),
            ("z_max",0.0),
        ]);

        let reader = BufReader::new(file).lines();    
        reader.for_each(|line| {

            // Each line we're interested in is either a 'v ' or an 'f '
            match line {
                Ok(content) => {
                    // Whenever there is a v
                    if content.starts_with("v ") {
                        // Splitting via single space
                        let mut coordinates_iter = content.split(" ");
                        // Skip the v
                        coordinates_iter.next();
                        // Every coordinate is added. They need to be parsed to f64.
                        let mut coordinate: Vec<f64> = coordinates_iter.map(|c| c.parse::<f64>().unwrap()).collect();

                        // If there is an ignored coordinate:
                        if let Some(ic) = ignored_coordinate {
                            coordinate.remove(ic);
                            // Last coordinate (z) becomes zero
                            coordinate.push(0.0);
                        } else {
                            // Check for z only on 3d
                            // Chech min and max value
                            let z_min = max_min.get_mut("z_min").unwrap();
                            if &coordinate[2] < z_min {
                                *z_min = coordinate[2];
                            }
                            let z_max = max_min.get_mut("z_max").unwrap();
                            if &coordinate[2] > z_max {
                                *z_max = coordinate[2];
                            }
                        }
                        // Check for min and max
                        let x_min = max_min.get_mut("x_min").unwrap();
                        if &coordinate[0] < x_min {
                            *x_min = coordinate[0];
                        }
                        let x_max = max_min.get_mut("x_max").unwrap();
                        if &coordinate[0] > x_max {
                            *x_max = coordinate[0];
                        }
                        let y_min = max_min.get_mut("y_min").unwrap();
                        if &coordinate[1] < y_min {
                            *y_min = coordinate[1];
                        }
                        let y_max = max_min.get_mut("y_max").unwrap();
                        if &coordinate[1] < y_max {
                            *y_max = coordinate[1];
                        }

                        // If 'ignored_coordinate' passes (in the case of D2), this unwrap is warranted to succeed.
                        coordinates.append(ndarray::Axis(0),Array1::from_vec(coordinate).view());
                    }
                        // Whenever there is a f
                        else if content.starts_with("f ") {
                            // Splitting via single space
                            let mut triangles_iter = content.split(" ");
                            // Skip the f
                            triangles_iter.next();
                            // Vertices are sepparated via '/'
                            let mut triangle: Vec<u32> = triangles_iter.map(|c| {
                                // There's no checking the line goes like 'f 1/2/2/ 1/1/1/ 2/3/2/'
                                let mut vertex: u32 = c.split("/").next().unwrap().parse::<u32>().unwrap();
                                // Return vertex-1 to match with index start in opengl (not_it/it/not_it)
                                vertex = vertex-1;
                                vertex
                            }).collect();
                            // Push into triangles vector of u32
                            triangles.append(ndarray::Axis(0),Array1::from_vec(triangle).view());
                        }
                    },
                // Error case of line matching
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }
        });

        // Initializing array of conditions for mesh
        let conditions: Array1<VertexType> = Array1::from_vec(Vec::with_capacity(coordinates.len() / 3));
        
        // Obtain middle point as if object was a parallelepiped
        let middle_point = [max_min.get("x_max").unwrap()-max_min.get("x_min").unwrap() / 2.0,
            max_min.get("y_max").unwrap()-max_min.get("y_min").unwrap() / 2.0, max_min.get("z_max").unwrap()-max_min.get("z_min").unwrap() / 2.0];
        
        let max_distance = Self::compare_distances(&max_min);
        
        (coordinates,triangles,conditions,max_distance,middle_point)
    }

    /// # General Information
    /// 
    /// ddd
    /// 
    /// # Parameters
    /// 
    /// ddd
    /// 
    pub fn build(self) -> Mesh {

        let binder = Binder::new();

        let (vertices, indices, conditions, max_length, mid_point) = match self.dimension {
            
            MeshDimension::Two => {
                // Obtained coordinates from 'generate_fields()' function
                let ignored_coordinate = self.ignored_coordinate();
                self.get_vertices_indices_and_conditions(ignored_coordinate)
            },
            MeshDimension::Three => {
                self.get_vertices_indices_and_conditions(None)
            }
        };

        // Translate matrix to given point
        let model_matrix = Matrix4::from_translation(Vector3::new(
            mid_point[0] as f32,
            mid_point[1] as f32,
            mid_point[2] as f32
        ));
        

        Mesh {
            vertices,
            indices,
            max_length,
            model_matrix,
            binder,
            conditions
        }
    }
}

#[cfg(test)]
mod test {

    use super::MeshBuilder;

    #[test]
    fn verify_coordinates_mesh() {

        let new_builder = MeshBuilder::new("/home/Arthur/Tesis/Dzahui/assets/untitled.obj");
        let y = new_builder.ignored_coordinate();
        assert!(y == Some(1));
    }

}