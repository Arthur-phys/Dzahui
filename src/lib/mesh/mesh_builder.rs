use std::io::{BufReader,BufRead};
use cgmath::{Matrix4,Vector3};
use ndarray::{Array1,s};
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
    One,
    Two,
    Three
}

/// Holder of .obj fields. Temporary object. Not to be used on it's own.
struct Obj {
    pub vertices: Array1<f64>,
    pub indices: Array1<u32>,
    pub max_length: f64,
    pub middle_point: [f64;3]
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

    /// Changes mesh dimension to 3D.
    pub(crate) fn with_mesh_in_3d(self) -> Self {
        Self {
            dimension: MeshDimension::Three,
            ..self
        }
    }

    /// Changes mesh dimension to 1D.
    pub(crate) fn with_mesh_in_1d(self) -> Self {
        Self {
            dimension: MeshDimension::One,
            ..self
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

    /// Obtains variables from .obj. To use after file check.
    fn get_vertices_and_indices(&self, ignored_coord: [bool;3]) -> Result<Obj, Error> {

        // Initial variables
        let mut vertices: Array1<f64> = Array1::from_vec(vec![]);
        let mut indices: Array1<u32> = Array1::from_vec(vec![]);

        let file = File::open(&self.location).expect("Error while opening file. Does file exists and is readable?");

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

                        // If there is an ignored coordinate:
                        if ignored_coord.contains(&true) {
                            let mut i: usize = 0;
                            for (index,coord) in ignored_coord.iter().enumerate() {
                                if *coord {
                                    coordinate.remove(index - i);
                                    // Last coordinate becomes zero
                                    coordinate.push(0.0);
                                    i += 1;
                                }
                            }
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

                        match vertices.append(ndarray::Axis(0),Array1::from_vec(coordinate).view()) {
                            Err(err) => panic!("{}",err),
                            _ => ()
                        }
                    }
                        // Whenever there is an f
                        else if content.starts_with("f ") {
                            // Splitting via single space
                            let triangle = match MeshBuilder::obj_face_checker(&content) {
                                Ok(tr) => tr,
                                Err(err) => panic!("{}",err)
                            };
                            // Push into triangles vector of u32
                            match indices.append(ndarray::Axis(0),Array1::from_vec(triangle).view()) {
                                Err(err) => panic!("{}",err),
                                _ => ()
                            }
                        }
                    },
                // Error case of line matching
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }

            Ok(())
        }).collect::<Result<Vec<_>,_>>()?;
        
        // Obtain middle point as if object was a parallelepiped
        let middle_point = [max_min.get("x_max").unwrap()-max_min.get("x_min").unwrap() / 2.0,
            max_min.get("y_max").unwrap()-max_min.get("y_min").unwrap() / 2.0, max_min.get("z_max").unwrap()-max_min.get("z_min").unwrap() / 2.0];
        
        let max_length = Self::compare_distances(&max_min);

        Ok(Obj {
            vertices,
            indices,
            max_length,
            middle_point
        })
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

    /// # General Information
    /// 
    /// ddd
    /// 
    /// # Parameters
    /// 
    /// ddd
    /// 
    pub(crate) fn build(self) -> Result<Mesh,Error> {

        let binder = Binder::new();
        let mut vertices: Vec<f64> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut codnitions: Vec<VertexType> = vec![];
        let file = File::open(&self.location)?;
        
        match self.dimension {
            MeshDimension::One => {
                
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
                                let mut j = vertices.len() - 5 - 1;
                                while j>=0 && vertices[j] > new_value {
                                    vertices[j+3] = vertices[j];
                                    j-=3;
                                }
                                vertices[j+3] = new_value;
                            }
                        },
                        // Error case of line matching
                        Err(error) => panic!("Unable to read file propperly {}",error)
                    }
                });

                let vertices_len: u32 = vertices.len() as u32;
                // Create a second vector of vertices above the first one to make a bar (seen on screen, for solving it serves no purpose) and append it to the first.
                let max_width = vertices[0] - vertices[vertices_len as usize - 3];
                let prom_width = max_width * (3 / vertices_len) as f64;
                vertices.append(&mut vertices.iter().enumerate().map(|(idx,x)| {if idx % 3 == 1 {prom_width} else {*x}}).collect::<Vec<f64>>());
                // Create indices for drawing
                indices.append(&mut vec![0,1,vertices_len]);
                indices.append(&mut vec![(vertices_len - 1),(vertices_len * 2 - 1),(vertices_len * 2 - 2)]);
                for i in 1..(vertices_len) - 1 {
                    indices.append(&mut vec![i,i + vertices_len,i + vertices_len - 1,i,i + 1,i + vertices_len])
                }
                
            },
            MeshDimension::Two => {
                
                // Check for one constant coordinate 
                let [set_x,set_y,set_z]= self.check_for_constant_coordinates()?;
                
                let constant_coordinate: u32 = if set_x.values().count() == 1 {
                    1
                } else if set_y.values().count() == 1 {
                    2
                } else if set_z.values().count() == 1 {
                    3
                } else {
                    return Err(Error::Parse("Only coordinates over a plane paralell to x, y or z plane are accepted. Check .obj file.".to_string()));
                };

            },
            MeshDimension::Three => {

            }
        }

        let ignored_coordinate = self.ignored_coordinate()?;
        let obj = self.get_vertices_and_indices(ignored_coordinate)?;

        // Translate matrix to given point
        let model_matrix = Matrix4::from_translation(Vector3::new(
            obj.middle_point[0] as f32,
            obj.middle_point[1] as f32,
            obj.middle_point[2] as f32
        ));
        
        // Initializing array of conditions for mesh
        let conditions: Array1<VertexType> = Array1::from_vec(Vec::with_capacity(obj.vertices.len() / 3));

        Ok(Mesh {
            vertices: obj.vertices,
            indices: obj.indices,
            max_length: obj.max_length,
            conditions,
            model_matrix,
            binder,
        })
    }
}

#[cfg(test)]
mod test {

    use super::MeshBuilder;

    #[test]
    fn verify_coordinates_mesh() {

        let new_builder = MeshBuilder::new("/home/Arthur/Tesis/Dzahui/assets/untitled.obj");
        let y = new_builder.ignored_coordinate().unwrap();
        assert!(y == [false,true,false]);
    }

    #[test]
    fn verify_coordinates_mesh_1d() {

        let new_builder = MeshBuilder::new("/home/Arthur/Tesis/Dzahui/assets/1dbar.obj").with_mesh_in_1d();
        let y = new_builder.ignored_coordinate().unwrap();
        assert!(y == [false,true,true]);
    }

}