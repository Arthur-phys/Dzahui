// Internal dependencies
use super::Mesh;
use crate::{simulation::drawable::binder::Binder, Error};

// External dependencies
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};
use cgmath::{Matrix4, Vector3};
use ndarray::Array1;
use std::fs::File;


/// # General Information
///
/// Enum to tell if additional checking over mesh should be done.
///
/// # Arms
///
/// * `One` - Line. In 1D, two of three coordinates must be constant throught the whole mesh.
/// * `Two` - Plane figure. In 2D, one coordinate needs to be constant throught the whole mesh.
/// * `Three` - 3D Body. No dimensional check-ups are done. Results depend solely on user's mesh.
///
#[derive(Debug)]
pub enum MeshDimension {
    One,
    Two,
    Three,
}

/// # General Information
///
/// **Needs .obj**.
/// Needed elements to create mesh (1D, 2D or 3D). Builds real structure parsing .obj and distinguishing internal and boundary vertices.
///
/// # Fields
///
/// * `location` - Path to .obj.
///
#[derive(Debug)]
pub(crate) struct MeshBuilder {
    location: String,
}

impl MeshBuilder {
    /// Creates initial instance. Not to be used on it's own. Use `Mesh::build`.
    pub(crate) fn new<B>(location: B) -> Self
    where
        B: AsRef<str>,
    {
        Self {
            location: location.as_ref().to_string(),
        }
    }

    /// Checks wether a line starting with 'v ' in an obj has the three vertices needed.
    /// Auxiliar function used inside build methods.
    /// Part of the checkup made to a given input file.
    fn obj_vertex_checker<A>(line: &A) -> Result<Vec<f64>, Error>
    where
        A: AsRef<str>,
    {
        let mut line_parts = line.as_ref().split(" ");
        line_parts.next();
        let line_parts: Vec<f64> = line_parts
            .map(|c| -> Result<f64, Error> {

                c.parse::<f64>().map_err(|e| {
                    Error::MeshParse(format!(
                        "Error while parsing vertex coordinate from obj: {}",
                        e
                    ))
                })
            
            })
            .collect::<Result<Vec<f64>, _>>()?;

        if line_parts.len() != 3 {
            return Err(Error::MeshParse(
                "A vertex line should contain 3 elements only".to_string(),
            ));
        }

        Ok(line_parts)
    }

    /// Verifies the amount of face specifications per line is 3 and also that all of them have the correct syntax 'a/b/c'.
    /// Auxiliar function used inside build methods.
    /// Part of the checkup made to a given input file.
    fn obj_face_checker<A>(line: &A) -> Result<Vec<u32>, Error>
    where
        A: AsRef<str>,
    {
        let mut triangle_faces = vec![];

        let mut line_parts = line.as_ref().split(" ");
        line_parts.next();
        let line_parts: Vec<&str> = line_parts.collect();

        // Check lenght of line
        if line_parts.len() != 3 {
            return Err(Error::MeshParse(
                "Amount of face specificating elements should be 3.".to_string(),
            ));
        }

        // Check for each part structur /a/b/c
        for face in line_parts {
            let mut face_part = face.split("/");

            let face_element = face_part.next();
            let face_element: u32 = if let Some(f) = face_element {
                f.parse::<u32>().map_err(|e| {
                    Error::MeshParse(format!("Error while parsing face coordinate: {}", e))
                })
            } else {
                Err(Error::MeshParse(format!("Error while parsing face coordinate")))
            }?;

            if face_part.count() != 2 {
                return Err(Error::MeshParse(
                    "Amount of elements per face specification should be 3 in format a/b/c."
                        .to_string(),
                ));
            }
            triangle_faces.push(face_element - 1);
        }

        Ok(triangle_faces)
    }

    /// # General information
    ///
    /// Returns hashmap with every diferent value per coordinate inside .obj.
    /// It's useful to check if a given .obj is a 2d or 1d mesh.
    ///
    /// # Parameters
    ///
    /// * `&self` - Only the file in self is needed to make the verification.
    ///
    fn check_for_constant_coordinates(&self) -> Result<[HashMap<String, f32>; 3], Error> {
        let file = File::open(&self.location)?;

        // Sets to check which one has only one element (i.e. which one should be ignored)
        // To implement set from list, use HashMap for better performance.
        let mut x: HashMap<String, f32> = HashMap::new();
        let mut y: HashMap<String, f32> = HashMap::new();
        let mut z: HashMap<String, f32> = HashMap::new();

        // Every line is treated individually
        BufReader::new(file).lines().map(|line| -> Result<(),Error> {

            let coordinates = line?;
            
            if coordinates.starts_with("v ") {
                // splitting via space
                let mut coordinates_iter = coordinates.split(" ");
                // skip the 'v'
                coordinates_iter.next();

                // mapping to tuple for HashMap
                let coordinates_vec: [(String, f32); 3] = coordinates_iter
                    .map(|c_str| -> Result<(String, f32),Error> {

                        // Necessary for -0.0 and 0.0 equality
                        if c_str.starts_with("0.0") || c_str.starts_with("-0.0") {
                            Ok((String::from("0.0"), c_str.parse::<f32>()?))
                        } else {
                            Ok((c_str.to_string(), c_str.parse::<f32>()?))
                        }
                    
                    })
                    // Now the result is transformed into an array of tuples size 3
                    .collect::<Result<Vec<(String, f32)>,Error>>()?
                    .try_into()
                    .map_err(|_err| -> Error {Error::Infallible})?;
                // Inserting into HashMap
                // Do not use clone, find replacement if possible (String needs cloning because of ownership)
                x.insert(coordinates_vec[0].0.clone(), coordinates_vec[0].1);
                y.insert(coordinates_vec[1].0.clone(), coordinates_vec[1].1);
                z.insert(coordinates_vec[2].0.clone(), coordinates_vec[2].1);

            } else {
            }
            
            Ok(())
        
        }).collect::<Result<Vec<()>,Error>>()?;
        
        Ok([x, y, z])
    }

    /// # General Information
    ///
    /// Builds a one dimensional mesh.
    /// Only a line of well defined points is needed. The method will create another paralell line to generate a bar
    /// copying every important element.
    /// Colors for mesh are inserted into vertices array, therefore, every vertex has 6 entries: 3 for coordinates and 3 for color (RGB),
    /// **Faces are not needed in .obj for this method**
    ///
    /// # Parameters
    ///
    /// `self` - Consumes builder.
    ///
    pub fn build_mesh_1d(self) -> Result<Mesh, Error> {
        // Generate every element needed at a functional scope.
        let binder = Binder::new();
        let mut vertices: Vec<f64> = vec![];
        let mut indices: Vec<u32> = vec![];
        let max_length: f64;
        let mut middle_point: [f32; 3] = [0.; 3];
        let file = File::open(&self.location)?;

        // Obtain hashmaps of coordinates
        let [set_x, set_y, set_z] = self.check_for_constant_coordinates()?;

        // Obtain constant coordinates
        let constant_coordinates: [usize; 2] = if set_x.values().count() == 1
            && set_y.values().count() == 1
        {
            [1, 0]
        } else if set_y.values().count() == 1 && set_z.values().count() == 1 {
            [2, 1]
        } else if set_z.values().count() == 1 && set_x.values().count() == 1 {
            [2, 0]
        } else {
            return Err(Error::MeshParse("Only coordinates over a line paralell to x, y or z axis are accepted. Check .obj file.".to_string()));
        };

        // Obtain ordered vertices
        let reader = BufReader::new(file).lines();
        reader
            .map(|line| -> Result<(), Error> {
                // Each line we're interested in starts with 'v '
                match line {
                    Ok(content) => {
                        // Whenever there is a v
                        if content.starts_with("v ") {
                            // Check line integrity
                            let mut coordinate = MeshBuilder::obj_vertex_checker(&content)?;

                            // Remove both coordinates. Since they are ordered above, this can be done as below
                            for coord in constant_coordinates {
                                coordinate.remove(coord);
                                coordinate.push(0.0);
                            }

                            // copying coordinate's only non-zero value as is needed below
                            let new_value = coordinate[0];
                            // Adding coordinate
                            vertices.append(&mut coordinate);
                            // Adding initial color
                            vertices.append(&mut vec![0.0, 0.0, 1.0]);
                            // Insertion sort skipping zero coordinates to order line (smaller to bigger elements) in case .obj is not.
                            let mut j = vertices.len() as i32 - 6 - 5 - 1;
                            while j >= 0 && vertices[j as usize] > new_value {
                                vertices[j as usize + 6] = vertices[j as usize];
                                j -= 6;
                            }
                            vertices[(j + 6) as usize] = new_value;
                        }
                        Ok(())
                    }
                    // Error case of line matching
                    Err(error) => Err(Error::Io(error)),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let vertices_len: u32 = vertices.len() as u32;
        // Obtain max_length easily once vertices are ordered
        max_length = -vertices[0] + vertices[vertices_len as usize - 6];
        // Prom width serves to give height to bar
        let prom_width = max_length * 6.0 / (vertices_len as f64 - 6.);
        // Create a second vector of vertices above the first one to make a bar (to be seen on screen, it serves no other purpose) and append it to the first.
        vertices.append(
            &mut vertices
                .iter()
                .enumerate()
                .map(|(idx, x)| if idx % 6 == 1 { prom_width } else { *x })
                .collect::<Vec<f64>>(),
        );

        // Create indices for drawing
        // First triangle
        indices.append(&mut vec![0, 1, vertices_len / 6]);
        // Last triangle
        indices.append(&mut vec![
            (vertices_len - 6) / 6,
            (vertices_len * 2 - 6) / 6,
            (vertices_len * 2 - 12) / 6,
        ]);
        // Every other (intermediate) triangle
        for i in 1..(vertices_len) / 6 - 1 {
            indices.append(&mut vec![
                i,
                i + vertices_len / 6,
                i + vertices_len / 6 - 1,
                i,
                i + 1,
                i + vertices_len / 6,
            ])
        }

        // get middle point for camera
        middle_point[0] = max_length as f32 / 2_f32;
        middle_point[1] = prom_width as f32 / 2_f32;

        // Translate matrix to given point
        let model_matrix = Matrix4::from_translation(Vector3::new(
            -middle_point[0] as f32,
            -middle_point[1] as f32,
            middle_point[2] as f32,
        ));

        Ok(Mesh {
            vertices: Array1::from_vec(vertices),
            indices: Array1::from_vec(indices),
            boundary_indices: None,
            max_length,
            model_matrix,
            binder,
        })
    }

    /// # General Information
    ///
    /// Builds a two dimensional mesh.
    /// A different approach needs to be taken to distinguish boundary vertices from internal ones. Algorithm consists on checking if a given edge
    /// from mesh appears once or more. If it appears only once, then the vertex is at the boundary (since it's only adjacent to a single traingle), otherwise,
    /// it's internal.
    /// Colors for mesh are inserted into vertices array, therefore, every vertex has 6 entries: 3 for coordinates and 3 for color (RGB).
    ///
    /// # Parameters
    ///
    /// `self` - Consumes builder.
    ///
    pub fn build_mesh_2d(self) -> Result<Mesh, Error> {
        // Generate every element needed at a functional scope.
        let binder = Binder::new();
        let mut vertices: Vec<f64> = vec![];
        let mut indices: Vec<u32> = vec![];
        let max_length: f64;
        let mut middle_point: [f32; 3] = [0.; 3];
        let file = File::open(&self.location)?;

        // Obtain hashmaps of every coordinate with only different coordinates' value.
        let [set_x, set_y, set_z] = self.check_for_constant_coordinates()?;

        // Get constant coordinate
        let constant_coordinate: usize = if set_x.values().count() == 1 {
            0
        } else if set_y.values().count() == 1 {
            1
        } else if set_z.values().count() == 1 {
            2
        } else {
            return Err(Error::MeshParse("Only coordinates over a plane paralell to x, y or z plane are accepted. Check .obj file.".to_string()));
        };

        // Generate maximum and minimum value hashmap for x and y to encapsulate mesh in a square (for proper viewing purposes).
        let mut max_min = HashMap::from([
            ("x_min", 0.0),
            ("y_min", 0.0),
            ("x_max", 0.0),
            ("y_max", 0.0),
        ]);

        // Primary data structure for boundary vertices algorithm (first we work with edges in the form (a,b))
        let mut boundary_edges: HashMap<[u32; 2], usize> = HashMap::new();

        let reader = BufReader::new(file).lines();
        reader
            .map(|line| -> Result<(), Error> {
                // Each line we're interested in is either a 'v ' or an 'f '
                let content = line?;
                
                // Whenever there is a v
                if content.starts_with("v ") {

                    // Check line integrity
                    let mut coordinate = MeshBuilder::obj_vertex_checker(&content)?;

                    // Remotion of the constant coordinate
                    coordinate.remove(constant_coordinate);
                    coordinate.push(0.0);

                    // Check for min and max values
                    let x_min = max_min.get_mut("x_min").ok_or(Error::Infallible)?;
                    if &coordinate[0] < x_min {
                        *x_min = coordinate[0];
                    }
                    let x_max = max_min.get_mut("x_max").ok_or(Error::Infallible)?;
                    if &coordinate[0] > x_max {
                        *x_max = coordinate[0];
                    }
                    let y_min = max_min.get_mut("y_min").ok_or(Error::Infallible)?;
                    if &coordinate[1] < y_min {
                        *y_min = coordinate[1];
                    }
                    let y_max = max_min.get_mut("y_max").ok_or(Error::Infallible)?;
                    if &coordinate[1] < y_max {
                        *y_max = coordinate[1];
                    }

                    vertices.append(&mut coordinate);
                    // Adding initial color: blue
                    vertices.append(&mut vec![0.0, 0.0, 1.0]);
                }
                // Whenever there is an f
                else if content.starts_with("f ") {
                    // Splitting via single space
                    let mut triangle = MeshBuilder::obj_face_checker(&content)?;

                    // filling boundary edges hashmap to obtain boundary vertices
                    // three possible combinations. Find better way to insert them
                    if let Some(counter) =
                        boundary_edges.get_mut(&[triangle[0], triangle[1]])
                    {
                        *counter += 1;
                    } else {
                        boundary_edges.insert([triangle[0], triangle[1]], 1);
                    }
                    if let Some(counter) =
                        boundary_edges.get_mut(&[triangle[0], triangle[2]])
                    {
                        *counter += 1;
                    } else {
                        boundary_edges.insert([triangle[0], triangle[2]], 1);
                    }
                    if let Some(counter) =
                        boundary_edges.get_mut(&[triangle[2], triangle[1]])
                    {
                        *counter += 1;
                    } else {
                        boundary_edges.insert([triangle[2], triangle[1]], 1);
                    }

                    // Push into triangles vector of u32
                    indices.append(&mut triangle);
                }
                Ok(())
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Obtaining max and min from hashmap
        let len_x = max_min.get("x_max").ok_or(Error::Infallible)? - max_min.get("x_min").ok_or(Error::Infallible)?;
        let len_y = max_min.get("y_max").ok_or(Error::Infallible)? - max_min.get("y_min").ok_or(Error::Infallible)?;
        // Obtaining middle point
        middle_point[0] = len_x as f32 / 2.0;
        middle_point[1] = len_y as f32 / 2.0;
        // Finally obtaining max length
        max_length = if len_x > len_y { len_x } else { len_y };

        // reducing boundary edges to vertices with a filter based on wether they are at the boundary or not.
        let boundary_indices: Vec<u32> = HashSet::<u32>::from_iter(
            boundary_edges
                .into_iter()
                .filter(|(_duple, counter)| if *counter != 1 { false } else { true })
                .collect::<HashMap<[u32; 2], usize>>()
                .into_keys()
                .flatten(),
        )
        .into_iter().collect();

        log::info!("{:?}",boundary_indices);

        // Model matrix for viewing purposes
        let model_matrix = Matrix4::from_translation(Vector3::new(
            middle_point[0] as f32,
            middle_point[1] as f32,
            middle_point[2] as f32,
        ));

        Ok(Mesh {
            vertices: Array1::from_vec(vertices),
            indices: Array1::from_vec(indices),
            boundary_indices: Some(boundary_indices),
            max_length,
            model_matrix,
            binder,
        })
    }

    /// # General Information
    ///
    /// Builds a three dimensional mesh.
    /// A different approach needs to be taken to distinguish boundary vertices from internal ones. Algorithm not yet implemented
    /// Colors for mesh are inserted into vertices array, therefore, every vertex has 6 entries: 3 for coordinates and 3 for color (RGB).
    ///
    /// # Parameters
    ///
    /// `self` - Consumes builder.
    ///
    pub fn build_mesh_3d(self) -> Result<Mesh, Error> {
        let binder = Binder::new();
        let mut vertices: Vec<f64> = vec![];
        let mut indices: Vec<u32> = vec![];
        let max_length: f64;
        let mut middle_point: [f32; 3] = [0.; 3];
        let file = File::open(&self.location)?;

        let mut max_min = HashMap::from([
            ("x_min", 0.0),
            ("y_min", 0.0),
            ("z_min", 0.0),
            ("x_max", 0.0),
            ("y_max", 0.0),
            ("z_max", 0.0),
        ]);

        let reader = BufReader::new(file).lines();
        reader
            .map(|line| -> Result<(), Error> {
                // Each line we're interested in is either a 'v ' or an 'f '
                
                let content = line?;
                // Whenever there is a v
                if content.starts_with("v ") {
                    // Check line integrity
                    let mut coordinate = MeshBuilder::obj_vertex_checker(&content)?;

                    // Check for min and max
                    let x_min = max_min.get_mut("x_min").ok_or(Error::Infallible)?;
                    if &coordinate[0] < x_min {
                        *x_min = coordinate[0];
                    }
                    let x_max = max_min.get_mut("x_max").ok_or(Error::Infallible)?;
                    if &coordinate[0] > x_max {
                        *x_max = coordinate[0];
                    }
                    let y_min = max_min.get_mut("y_min").ok_or(Error::Infallible)?;
                    if &coordinate[1] < y_min {
                        *y_min = coordinate[1];
                    }
                    let y_max = max_min.get_mut("y_max").ok_or(Error::Infallible)?;
                    if &coordinate[1] < y_max {
                        *y_max = coordinate[1];
                    }
                    let z_min = max_min.get_mut("z_min").ok_or(Error::Infallible)?;
                    if &coordinate[1] < z_min {
                        *z_min = coordinate[1];
                    }
                    let z_max = max_min.get_mut("z_max").ok_or(Error::Infallible)?;
                    if &coordinate[1] < z_max {
                        *z_max = coordinate[1];
                    }

                    vertices.append(&mut coordinate);
                    vertices.append(&mut vec![0.0, 0.0, 1.0]);
                }
                // Whenever there is an f
                else if content.starts_with("f ") {
                    // Splitting via single space
                    let mut triangle = MeshBuilder::obj_face_checker(&content)?;
                    // Push into triangles vector of u32
                    indices.append(&mut triangle);
                }
                    
                

                Ok(())
            })
            .collect::<Result<Vec<_>, _>>()?;

        let len_x = max_min.get("x_max").ok_or(Error::Infallible)? - max_min.get("x_min").ok_or(Error::Infallible)?;
        let len_y = max_min.get("y_max").ok_or(Error::Infallible)? - max_min.get("y_min").ok_or(Error::Infallible)?;
        let len_z = max_min.get("z_max").ok_or(Error::Infallible)? - max_min.get("z_min").ok_or(Error::Infallible)?;
        middle_point[0] = len_x as f32 / 2.0;
        middle_point[1] = len_y as f32 / 2.0;
        middle_point[2] = len_z as f32 / 2.0;

        max_length = if len_x >= len_y && len_x >= len_z {
            len_x
        } else if len_y >= len_x && len_y >= len_z {
            len_y
        } else {
            len_z
        };
        // Translate matrix to given point
        let model_matrix = Matrix4::from_translation(Vector3::new(
            middle_point[0] as f32,
            middle_point[1] as f32,
            middle_point[2] as f32,
        ));

        Ok(Mesh {
            vertices: Array1::from_vec(vertices),
            indices: Array1::from_vec(indices),
            boundary_indices: None,
            max_length,
            model_matrix,
            binder,
        })
    }
}

trait Sortable: IntoIterator + Sized + PartialEq + PartialOrd {
    
    fn merge_sort(self) -> Self {
        

        todo!()
    }

}