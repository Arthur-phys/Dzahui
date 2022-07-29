pub mod vertex;

use super::{Drawable, Binder, from_obj::FromObj};
use std::{fs::File,io::{BufReader, BufRead}};
use cgmath::{Vector3, Matrix4};
use std::collections::HashMap;
use vertex::VertexList;

/// # General Information
/// 
/// Representation of a plane figure. Contains information to draw to screen and move/rotate it to final position.
/// 
/// # Fields
/// 
/// * `vertices` -  Vertices in 3d space. Normally used in triads. Specified in gl configuration.
/// * `triangles` - Indices that map to vertices. Normally used in triads. Specified in gl configuration.
/// * `ignored_coordinate` - 2D Mesh should ignore one entry: The one which is the same in all of .obj vertex specification.
/// * `max_length` - Maximum length of figure. Used to center camera arround mesh.
/// * `model_matrix` - Translates and rotates object to final world position.
/// * `binder` - vao, vbo and ebo variables bound to mesh drawable in GPU.
///
#[derive(Debug)]
pub struct Mesh {
    binder: Binder,
    pub selectable_vertices: VertexList,
    dimension: MeshDimension,
    pub vertices: Vec<f32>,
    pub triangles: Vec<u32>, 
    pub ignored_coordinate: Option<usize>,
    pub max_length: f32,
    pub model_matrix: Matrix4<f32>,
}

/// # General Information
/// 
/// Needed elements to create mesh (2D or 3D). Provides option to personalize vertices.
/// 
/// # Fields
/// 
/// * `location` - Path to mesh's `.obj`.
/// * `dimension` - Enum with mesh's dimension. Needs to be set to enable/disable checkoing for repeated coordinate in `.obj` if it's 2D.
/// * `vertex_body` - Allows vertex personalization if set.
///
#[derive(Debug)]
pub struct MeshBuilder<A: AsRef<str>, B: AsRef<str>> {
    location: B,
    dimension: MeshDimension, 
    vertex_body: Option<A>,
    size: Option<f32>
}

impl<A: AsRef<str>, B: AsRef<str>> MeshBuilder<A,B> {
    
    /// Creates default instance.
    fn new(location: B) -> Self {
        Self {
            location,
            dimension: MeshDimension::Two,
            vertex_body: None,
            size: None
        }
    }
    /// Obtains new vertex body to draw.
    pub fn with_vertex_body(self, vertex_body: A) -> Self {
        Self {
            vertex_body: Some(vertex_body),
            ..self
        }
    }
    /// Changes mesh dimension.
    pub fn with_mesh_in_3d(self) -> Self {
        Self {
            dimension: MeshDimension::Three,
            ..self
        }
    }
    /// Change size.
    pub fn with_size(self, size: f32) -> Self {
        Self {
            size: Some(size),
            ..self
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
    fn create_highlightable_vertices(vertices: &Vec<f32>, size: f32, file: &str) -> VertexList {
    
        let centers: Vec<Vector3<f32>> = (0..vertices.len()).step_by(3).map(|i| {
            Vector3::new(vertices[i] as f32,vertices[i+1] as f32,vertices[i+2] as f32)
        }).collect();

        VertexList::new(centers, size, file) 
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

        let vertex_body_file = if let Some(vertex_body_file) = self.vertex_body { vertex_body_file.as_ref().to_string() } else 
        { "./assets/sphere.obj".to_string() };
        let mut ignored_coordinate = None;

        let (vertices, triangles, max_length, closest_point) = match self.dimension {

            MeshDimension::Two => {
                ignored_coordinate = Mesh::get_ignored_coordinate(self.location.as_ref().to_string());
                // Obtained coordinates from 'generate_fields()' function
                Mesh::generate_fields(self.location, ignored_coordinate)

            },

            MeshDimension::Three => {
                Mesh::generate_fields(self.location, None)
            }
        };

        // Translate matrix to given point
        // NOT OK. model_matrix changes for 3d
        let model_matrix = Matrix4::from_translation(Vector3::new(
            closest_point[0] as f32,
            closest_point[1] as f32,
            0.0
        ));

        // Binder
        let mut binder = Binder::new();
        // connect binder with gpu
        binder.setup();

        // Selectable vertices
        let size = if let Some(size) = self.size { size } else { max_length/(vertices.len() as f32) };

        let selectable_vertices = Self::create_highlightable_vertices(&vertices, size,
            vertex_body_file.as_str());
        

        Mesh {
            ignored_coordinate,
            selectable_vertices,
            vertices,
            triangles,
            max_length,
            model_matrix,
            binder,
            dimension: self.dimension,
        }
    }
}

impl Drawable for Mesh {

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
impl FromObj for Mesh {}

impl Mesh {

    /// Getter for model_matrix
    pub fn get_model_matrix(&self) -> &Matrix4<f32> {
        &self.model_matrix
    }

    /// Creates new instance of builder
    pub fn builder<A: AsRef<str>, B: AsRef<str>>(location: B) -> MeshBuilder<A,B> {
        MeshBuilder::new(location)
    }

    pub fn get_ignored_coordinate<A: AsRef<str>>(file: A) -> Option<usize> {
        // Obtain unused coordinate index from .obj file.
        
        let file = File::open(file.as_ref()).expect("Error while opening the file. Does the file exists and is readdable?");
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

/// # General Information
/// 
/// Enum to tell if mesh being in a plane should be checked.
/// 
/// # Arms
/// 
/// * `Two` - Plane figure. Additional check-up to confirm property will be applied simplifying final mesh.
///  * `Three` - 3D Body. No check-ups are done. Results depend solely on user's .obj
#[derive(Debug)]
pub enum MeshDimension {
    Two,
    Three
}