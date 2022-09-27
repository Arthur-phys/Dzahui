use std::io::{BufReader,BufRead};
use cgmath::{Matrix4,Vector3};
use ndarray::{Array1,arr1};
use std::collections::HashMap;
use std::fs::File;

use crate::{simulation::drawable::binder::Binder,Error};
use super::{Mesh, vertex_type::{VertexType, Condition}};

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
    One,
    Two,
    Three
}

/// # General Information
/// 
/// Needed elements to create mesh (2D or 3D). Builds real structure providing parsing of .obj and distinguishing internal an boundary vertices. 
/// 
/// # Fields
/// 
/// * `location` - Path to mesh's `.obj`.
/// * `dimension` - Enum with mesh's dimension. Needs to be set to enable/disable checking for repeated coordinate in `.obj` if it's 2D or 1D.
///
#[derive(Debug)]
pub(crate) struct MeshBuilder {
    location: String,
}

impl MeshBuilder {
    
    /// Creates default/initial instance.
    pub(crate) fn new<B>(location: B) -> Self
    where B: AsRef<str> {
        Self {
            location: location.as_ref().to_string(),
        }
    }

    /// Checks wether a line in an obj has only three vertices.
    /// Part of the checkup made to a given input file.
    fn obj_vertex_checker<A>(line: &A) -> Result<Vec<f64>,Error>
    where A: AsRef<str> { 
        
        let mut line_parts = line.as_ref().split(" ");
        line_parts.next();
        let line_parts: Vec<f64> = line_parts.map(|c| c.parse::<f64>().unwrap()).collect();
        
        if line_parts.len() != 3 {
           return Err(Error::Parse("A vertex line should contain 3 vertices only".to_string()));
        }

        Ok(line_parts)
    }
    
    /// Verifies the amount of face specifications per line is 3 and also that all of them have the correct syntax.
    /// Part of the checkup made to a given input file.
    fn obj_face_checker<A>(line: &A) -> Result<Vec<u32>, Error>
    where A: AsRef<str> {

        let mut triangle_faces = vec![];
        
        let mut line_parts = line.as_ref().split(" ");
        line_parts.next();
        let line_parts: Vec<&str> = line_parts.collect();
        
        // Check lenght of line
        if line_parts.len() != 3 {
            return Err(Error::Parse("Amount of face specificating elements should be 3.".to_string()));
        }
        
        // Check for each part structur /a/b/c
        for face in line_parts {
            let mut face_part = face.split("/");
    
            let face_element: u32 = face_part.next().unwrap().parse::<u32>().unwrap();
    
            if face_part.count() != 2 {
                return Err(Error::Parse("Amount of elements per face specification should be 3 in format a/b/c.".to_string()));
            }
            triangle_faces.push(face_element-1);
        }
        
        Ok(triangle_faces)
    }

    /// # General information
    /// 
    /// Checks values of x, y and z coordinates to see if one or two of them is effectively constant.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - Only the file in self is needed to make the verification.
    /// 
    fn check_for_constant_coordinates(&self) -> Result<[HashMap<String,f32>;3],Error> {
        
        let file = File::open(&self.location)?;
        
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
        Ok([x,y,z])
    }

    /// # General Information
    /// 
    /// ddd
    /// 
    /// # Parameters
    /// 
    /// ddd
    /// 
    pub fn build_mesh_1d(self) -> Result<Mesh,Error> {
        
        let binder = Binder::new();
        let mut vertices: Vec<f64> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut conditions: Vec<VertexType>;
        let max_length: f64;
        let mut middle_point: [f32;3] = [0.;3];
        let file = File::open(&self.location)?;

        // Check for two constant coordinates
        let [set_x,set_y,set_z]= self.check_for_constant_coordinates()?;

        let constant_coordinates: [usize;2] = if set_x.values().count() == 1 && set_y.values().count() == 1 {
            [1,0]
        } else if set_y.values().count() == 1 && set_z.values().count() == 1  {
            [2,1]
        } else if set_z.values().count() == 1 && set_x.values().count() == 1 {
            [2,0]
        } else {
            return Err(Error::Parse("Only coordinates over a line paralell to x, y or z axis are accepted. Check .obj file.".to_string()));
        };

        // Obtain ordered vertices

        let reader = BufReader::new(file).lines();    
        reader.for_each(|line| {
            // Each line we're interested in starts with 'v '
            match line {
                
                Ok(content) => {
                    // Whenever there is a v
                    if content.starts_with("v ") {
                        // Check line integrity
                        let mut coordinate = match MeshBuilder::obj_vertex_checker(&content) {
                            Ok(coord) => coord,
                            Err(error) => panic!("{}",error.to_string())
                        };

                        for coord in constant_coordinates {
                            coordinate.remove(coord);
                            coordinate.push(0.0);
                        }

                        let new_value = coordinate[0];
                        vertices.append(&mut coordinate);
                        // Insertion sort skipping zero coordinates
                        let mut j = vertices.len() as i32 - 5 - 1;
                        while j>=0 && vertices[j as usize] > new_value {
                            vertices[j as usize + 3] = vertices[j as usize];
                            j-=3;
                        }
                        vertices[(j + 3) as usize] = new_value;
                    }
                },
                // Error case of line matching
                Err(error) => panic!("Unable to read file propperly {}",error)
            }
        });

        let vertices_len: u32 = vertices.len() as u32;
        // Create a second vector of vertices above the first one to make a bar (seen on screen, for solving it serves no purpose) and append it to the first.
        max_length = - vertices[0] + vertices[vertices_len as usize - 3];
        let prom_width = max_length * 3.0 / (vertices_len as f64 - 3.);
        vertices.append(&mut vertices.iter().enumerate().map(|(idx,x)| {if idx % 3 == 1 {prom_width} else {*x}}).collect::<Vec<f64>>());
        
        // Create indices for drawing
        indices.append(&mut vec![0,1,vertices_len/3]);
        indices.append(&mut vec![(vertices_len - 3)/3,(vertices_len * 2 - 3)/3,(vertices_len * 2 - 6)/3]);
        for i in 1..(vertices_len)/3 - 1 {
            indices.append(&mut vec![i,i + vertices_len/3,i + vertices_len/3 - 1,i,i + 1,i + vertices_len/3])
        }

        //Create vector of vertex types
        conditions = vec![VertexType::Internal(arr1(&[0.0,0.0,0.0]));vertices_len as usize * 2];
        conditions[0] = VertexType::Boundary(Condition::Dirichlet(arr1(&[0.0,0.0,0.0])));
        conditions[vertices_len as usize - 1] = VertexType::Boundary(Condition::Dirichlet(arr1(&[0.0,0.0,0.0])));
        conditions[vertices_len as usize] = VertexType::Boundary(Condition::Dirichlet(arr1(&[0.0,0.0,0.0])));
        conditions[vertices_len as usize * 2 - 1] = VertexType::Boundary(Condition::Dirichlet(arr1(&[0.0,0.0,0.0])));

        // get middle point
        middle_point[0] = max_length as f32 / 2.;
        middle_point[1] = prom_width as f32 / 2.; 

        // Translate matrix to given point
        let model_matrix = Matrix4::from_translation(Vector3::new(
            middle_point[0] as f32,
            middle_point[1] as f32,
            middle_point[2] as f32
        ));
        
        Ok(Mesh {
            vertices: Array1::from_vec(vertices),
            indices: Array1::from_vec(indices),
            max_length,
            conditions: Array1::from_vec(conditions),
            model_matrix,
            binder,
        })
    }

    pub fn build_mesh_2d(self) -> Result<Mesh,Error> {

        let binder = Binder::new();
        let mut vertices: Vec<f64> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut conditions: Vec<VertexType> = vec![];
        let max_length: f64;
        let mut middle_point: [f32;3] = [0.;3];
        let file = File::open(&self.location)?;

         // Check for one constant coordinate 
        let [set_x,set_y,set_z]= self.check_for_constant_coordinates()?;
            
        let constant_coordinate: usize = if set_x.values().count() == 1 {
            0
        } else if set_y.values().count() == 1 {
            1
        } else if set_z.values().count() == 1 {
            2
        } else {
            return Err(Error::Parse("Only coordinates over a plane paralell to x, y or z plane are accepted. Check .obj file.".to_string()));
        };

        let mut max_min = HashMap::from([
            ("x_min",0.0),
            ("y_min",0.0),
            ("x_max",0.0),
            ("y_max",0.0),
        ]);

        let reader = BufReader::new(file).lines();    
        reader.map(|line| -> Result<(), Error> {
            // Each line we're interested in is either a 'v ' or an 'f '
            match line {
                Ok(content) => {
                    // Whenever there is a v
                    if content.starts_with("v ") {
                        
                        // Check line integrity
                        let mut coordinate = match MeshBuilder::obj_vertex_checker(&content) {
                            Ok(coord) => coord,
                            Err(error) => panic!("{}",error.to_string())
                        };

                        coordinate.remove(constant_coordinate);
                        coordinate.push(0.0);

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

                        vertices.append(&mut coordinate);
                    }
                        // Whenever there is an f
                        else if content.starts_with("f ") {
                            // Splitting via single space
                            let mut triangle = match MeshBuilder::obj_face_checker(&content) {
                                Ok(tr) => tr,
                                Err(err) => panic!("{}",err)
                            };
                            // Push into triangles vector of u32
                            indices.append(&mut triangle);
                        }
                    },
                // Error case of line matching
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }

            Ok(())
         }).collect::<Result<Vec<_>,_>>()?;

        let len_x = max_min.get("x_max").unwrap()-max_min.get("x_min").unwrap();
        let len_y = max_min.get("y_max").unwrap()-max_min.get("y_min").unwrap();
        middle_point[0] = len_x as f32 / 2.0;
        middle_point[1] = len_y as f32 / 2.0;
 
        max_length = if len_x > len_y {len_x} else {len_y};

        let model_matrix = Matrix4::from_translation(Vector3::new(
            middle_point[0] as f32,
            middle_point[1] as f32,
            middle_point[2] as f32
        ));
        
        Ok(Mesh {
            vertices: Array1::from_vec(vertices),
            indices: Array1::from_vec(indices),
            max_length,
            conditions: Array1::from_vec(conditions),
            model_matrix,
            binder,
        })
    }
    
    pub fn build_mesh_3d(self) -> Result<Mesh,Error> {

        let binder = Binder::new();
        let mut vertices: Vec<f64> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut conditions: Vec<VertexType> = vec![];
        let max_length: f64;
        let mut middle_point: [f32;3] = [0.;3];
        let file = File::open(&self.location)?;

        let mut max_min = HashMap::from([
            ("x_min",0.0),
            ("y_min",0.0),
            ("z_min",0.0),
            ("x_max",0.0),
            ("y_max",0.0),
            ("z_max",0.0)
        ]);

        let reader = BufReader::new(file).lines();    
        reader.map(|line| -> Result<(), Error> {
            // Each line we're interested in is either a 'v ' or an 'f '
            match line {
                Ok(content) => {
                    // Whenever there is a v
                    if content.starts_with("v ") {
                        
                        // Check line integrity
                        let mut coordinate = match MeshBuilder::obj_vertex_checker(&content) {
                            Ok(coord) => coord,
                            Err(error) => panic!("{}",error.to_string())
                        };


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
                        let z_min = max_min.get_mut("z_min").unwrap();
                        if &coordinate[1] < z_min {
                            *z_min = coordinate[1];
                        }
                        let z_max = max_min.get_mut("z_max").unwrap();
                        if &coordinate[1] < z_max {
                            *z_max = coordinate[1];
                        }

                        vertices.append(&mut coordinate);
                    }
                        // Whenever there is an f
                        else if content.starts_with("f ") {
                            // Splitting via single space
                            let mut triangle = match MeshBuilder::obj_face_checker(&content) {
                                Ok(tr) => tr,
                                Err(err) => panic!("{}",err)
                            };
                            // Push into triangles vector of u32
                            indices.append(&mut triangle);
                        }
                    },
                // Error case of line matching
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }

            Ok(())
        }).collect::<Result<Vec<_>,_>>()?;

        let len_x = max_min.get("x_max").unwrap()-max_min.get("x_min").unwrap();
        let len_y = max_min.get("y_max").unwrap()-max_min.get("y_min").unwrap();
        let len_z = max_min.get("z_max").unwrap()-max_min.get("z_min").unwrap();
        middle_point[0] = len_x as f32 / 2.0;
        middle_point[1] = len_y as f32 / 2.0;
        middle_point[2] = len_z as f32 / 2.0;

        max_length = if len_x >= len_y && len_x >= len_z {len_x} else if len_y >= len_x && len_y >= len_z {len_y} else {len_z};
        // Translate matrix to given point
        let model_matrix = Matrix4::from_translation(Vector3::new(
            middle_point[0] as f32,
            middle_point[1] as f32,
            middle_point[2] as f32
        ));

        Ok(Mesh {
            vertices: Array1::from_vec(vertices),
            indices: Array1::from_vec(indices),
            max_length,
            conditions: Array1::from_vec(conditions),
            model_matrix,
            binder,
        })
    }
}