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
/// * `dimension` - Enum with mesh's dimension. Needs to be set to enable/disable checking for repeated coordinate in `.obj` if it's 2D.
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
    /// When a mesh is set to 2D or 1D, a verification is made on the file, ensuring that one of three coordinates is effectively constant.
    /// Verifying returns the coordinate that is zero.
    /// Later on, when reading the file again, coordinates are switched so that z-coordinate becomes a zero coordinate, regardless of it's previous values in .obj, 
    /// and the original zero coordinate becomes populated with the z-coordinate values.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - Only the file and dimension within self is needed to make the verification. 
    /// 
    fn ignored_coordinate(&self) -> Result<[bool;3],Error> {

        if let MeshDimension::Three = self.dimension {
            return Ok([false;3]);
        }
        
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

        // After for_each, we verify which coordinate is constant
        match self.dimension {
            MeshDimension::One => {
                if x.values().count() == 1 && y.values().count() == 1 {
                    Ok([true,true,false])
                } else if y.values().count() == 1 && z.values().count() == 1  {
                    Ok([false,true,true])
                } else if z.values().count() == 1 && x.values().count() == 1 {
                    Ok([true,false,true])
                } else {
                    panic!("Only coordinates over a line paralell to x, y or z planes are accepted. Check .obj file.");
                }
            },
            MeshDimension::Two => {
                if x.values().count() == 1 {
                    Ok([true,false,false])
                } else if y.values().count() == 1 {
                    Ok([false,true,false])
                } else if z.values().count() == 1 {
                    Ok([false,false,true])
                } else {
                    panic!("Only coordinates over a plane paralell to x, y or z axis are accepted. Check .obj file.");
                }
            },
            _ => Ok([false;3])
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

                        // If 'ignored_coordinate' passes (in the case of D2), this unwrap is warranted to succeed.
                        match vertices.append(ndarray::Axis(0),Array1::from_vec(coordinate).view()) {
                            Err(err) => panic!("{}",err),
                            _ => ()
                        }
                    }
                        // Whenever there is a f
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

    /// # General Information
    /// 
    /// Basically an implementation of merge sort.
    /// Helps indetify vertices of mesh.
    /// First and last vertices are boundaries. Everything else is an internal vertex.
    /// 
    /// 
    fn define_mesh_vertices_1d(vertices: &Array1<f64>) -> Array1<VertexType> {

        let non_zero_vertices: Vec<f64> = vertices.iter().filter_map(|x: &f64| {
            if *x != 0.0 {
                Some(*x)
            } else {
                None
            }
        }).collect();

        fn merge(mut initial_array: Array1<f64>, left_array: Array1<f64>, right_array: Array1<f64>) -> Array1<f64> {
            let mut i = 0;
            let mut j = 0;
            let mut k = 0;

            while i < left_array.len() && j < right_array.len() {
                if left_array[i] < right_array[j] {
                    initial_array[k] = left_array[i];
                    i += 1;
                } else {
                    initial_array[k] = right_array[j];
                    j += 1;
                }
                k += 1;
            }
            if i >= left_array.len() {
                initial_array[k..] = right_array[j..];
            } else {
                initial_array[k..] = left_array[i..]
            }

            initial_array
        }

        fn merge_sort(mut arr: Array1<f64>) -> Array1<f64> {
            let len = arr.len();
            if len < 2 {
                return arr
            } else {
                let mid = len / 2;
                let left = arr[..mid];
                let right = arr[len-mid..];
                let left = merge_sort(left);
                let right = merge_sort(right);
                merge(arr,left,right)
            }
        }


        todo!()
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