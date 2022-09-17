use std::io::{BufReader,BufRead};
use cgmath::{Matrix4,Vector3};
use ndarray::{Array3,Array1};
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
    dimension: MeshDimension,
    vertices: Option<Array1<f64>>,
    indices: Option<Array1<usize>>,
    conditions: Option<Array1<VertexType>>
}

impl MeshBuilder {
    
    /// Creates default instance.
    pub(crate) fn new<B>(location: B) -> Self
    where B: AsRef<str> {
        Self {
            location: location.as_ref().to_string(),
            dimension: MeshDimension::Two,
            vertices: None,
            indices: None,
            conditions: None
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
    fn obj_vertex_checker(line: &str) -> Result<bool,&str> { 
        
        let line_parts: Vec<&str> = line.split(" ").collect();
        if line_parts.len() != 4 {
           return Err("Amount of numbers per vertex should be 3.");
        }
        Ok(true)
    }
    
    /// Verifies the amount of face specifications per line is 3 and also that all of them have the correct syntax.
    /// Part of the checkup made to a given input file.
    fn obj_face_checker(line: &str) -> Result<bool, &str> {

        let line_parts: Vec<&str> = line.split(" ").collect();
        // Check lenght of line
        if line_parts.len() != 4 {
            return Err("Amount of face specification elements should be 3.");
        }
        // Check for each part structur /a/b/c
        for face in line_parts {
            let face_parts: Vec<&str> = face.split("/").collect();
            if face_parts.len() != 3 {
                return Err("Amount of elements per face specification should be 3 in format a/b/c.");
            }
        }
        Ok(true)
    }

    /// # General Information
    ///
    /// Checks a given .obj file one line at a time. Returns a true value if file can be used to create a mesh.
    /// 
    /// # Parameters
    /// 
    /// * `file` - Path to a given .obj.
    ///
    fn check_obj<A>(file: A) -> Result<bool,Error>
    where A: AsRef<str> {

        let file = file.as_ref().to_string();
        
        if !file.ends_with(".obj") {
            return Err(Error::ExtensionNotAllowed(file,String::from("Mesh creation")));
        }

        // Initializing file
        let file = File::open(file)?;
        let reader = BufReader::new(file).lines();

        // For each line checks are made
        reader.for_each(|line| {

            let content = match line {
                Ok(content) => content,
                Result::Err(e) => panic!(),
            };
            if content.starts_with("v ") {

                let res = Self::obj_vertex_checker(&content).expect("Not implemented"); 

            } else if content.starts_with("f ") {

                let res = Self::obj_face_checker(&content).expect("Not implemented");
            }}
        );

        Ok(true)
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

    fn set_vertices_indices_and_conditions<A: AsRef<str>>(file: A) -> (Array1<f64>,Array1<u32>,f32,[f32;3]) {
        // Obtains variables from .obj. To use after file check.

        // Initial variables
        let mut coordinates: Vec<f32> = Vec::new();
        let mut triangles: Vec<u32> = Vec::new();

        let file = File::open(file.as_ref()).expect("Error while opening file. Does file exists and is readable?");

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
                        let mut coordinate: Vec<f32> = coordinates_iter.map(|c| c.parse::<f32>().unwrap()).collect();

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

                        // If 'get_ignored_coordinate' passes (in the case of D2), this unwrap is warranted to succeed.
                        coordinates.append(&mut coordinate);
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
                            triangles.append(&mut triangle);
                        }
                    },
                // Error case of line matching
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }
        });
        
        // Obtain middle point as if object was a parallelepiped
        let middle_point = [max_min.get("x_max").unwrap()-max_min.get("x_min").unwrap() / 2.0,
            max_min.get("y_max").unwrap()-max_min.get("y_min").unwrap() / 2.0, max_min.get("z_max").unwrap()-max_min.get("z_min").unwrap() / 2.0];
        
        let max_distance = Self::compare_distances(&max_min);
        
        (coordinates,triangles,max_distance,middle_point)
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

        let mut ignored_coordinate = None;
        let (vertices, indices, max_length, mid_point) = match self.dimension {

            MeshDimension::Two => {
                ignored_coordinate = Mesh::get_ignored_coordinate(self.location);
                // Obtained coordinates from 'generate_fields()' function
                MeshBuilder::generate_fields(self.location, ignored_coordinate)

            },

            MeshDimension::Three => {
                MeshBuilder::generate_fields(self.location, None)
            }
        };

        // Translate matrix to given point
        let model_matrix = Matrix4::from_translation(Vector3::new(
            mid_point[0] as f32,
            mid_point[1] as f32,
            mid_point[2] as f32
        ));
        

        Mesh {
            ignored_coordinate,
            vertices,
            indices,
            max_length,
            model_matrix,
            binder
        }
    }
}