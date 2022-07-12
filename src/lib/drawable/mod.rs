// Common functions in drawable (2D or 3D objects)

use std::{collections::HashMap,ptr,mem,fs::File,os::raw::c_void,io::{BufReader, BufRead}};
use gl::{self,types::{GLdouble, GLsizei, GLsizeiptr, GLuint}};
use cgmath::{Matrix4,SquareMatrix, Vector3};
use crate::{DzahuiWindow, SphereList};

pub mod mesh2d;
pub mod mesh3d;
pub mod sphere;

// Variables asocciated with GPU and drawable object
pub struct Binder {
    pub vbo: u32, // Vertex Buffer Object - Vertices Generated by Mesh
    pub vao: u32, // Vertex Array Object - Binds vertices and it's configuration with OpenGL
    pub ebo: u32, // Element Buffer Object - Indices to draw vertices
}

impl Binder {
    pub fn new(vbo: u32, vao: u32, ebo: u32) -> Binder {
        Binder { vbo, vao, ebo }
    }
}

pub trait HighlightableVertices: Drawable {

    fn create_highlightable_vertices(&self, radius: f32, file: &str) -> SphereList {
    
        let vertices = self.get_vertices();
        let centers: Vec<Vector3<f32>> = (0..vertices.len()).step_by(3).map(|i| {
            Vector3::new(vertices[i] as f32,vertices[i+1] as f32,vertices[i+2] as f32)
        }).collect();

        SphereList::new(centers,radius, file)
        
    }
}
// All drawable objects implement a draw and setup function
pub trait Drawable {
    // Getters
    fn get_vertices(&self) -> &Vec<f64>;
    fn get_triangles(&self) -> &Vec<u32>;
    fn get_max_length(&self) -> f64;

    // Needed methods
    fn setup(&self, binder: &mut Binder) {

        let vertices = self.get_vertices();
        let triangles = self.get_triangles();

        unsafe {
            // Create VAO
            gl::GenVertexArrays(1,&mut binder.vao);
            // Bind Vertex Array Object first
            // Since it is bound first, it binds to tthe EBO and VBO (because they are the only ones being bound after it)
            gl::BindVertexArray(binder.vao);
            
            // Generates a VBO in GPU
            gl::GenBuffers(1, &mut binder.vbo);
            // Generates a EBO in GPU
            gl::GenBuffers(1, &mut binder.ebo);
            // Bind VBO
            gl::BindBuffer(gl::ARRAY_BUFFER,binder.vbo);
            // Point to data, specify data length and how it should be drawn (static draw serves to only draw once).
            gl::BufferData(gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLdouble>()) as GLsizeiptr,
                &vertices[0] as *const f64 as *const c_void,
                gl::STATIC_DRAW);// Double casting to raw pointer. Equivalent to C's void type when used as pointer.
                
            // Bind EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,binder.ebo);
            // Point to data, specify data length and hot it should be drawn
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (triangles.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                &triangles[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW);
                
            // How should coordinates be read.
            // Reading starts at index 0
            // Each coordinate is composed of 3 values
            // No normalized coordinates
            // The next coordinate is located 3 values after the first index of the previous one
            // The offset to start reading coordinates (for position it's normally zero. It is used when having texture and/or color coordinates)
            gl::VertexAttribPointer(0,3,gl::DOUBLE,
                gl::FALSE,
                (3*mem::size_of::<GLdouble>()) as GLsizei,
                ptr::null());
                        
            // Enable vertex atributes giving vertex location (setup in vertex shader).
            gl::EnableVertexAttribArray(0);
            // Comment to see the traingles filled instead of only the lines that form them
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }

    fn draw(&self, window: &DzahuiWindow, binder: &Binder) {
        let indices_len: i32 = self.get_triangles().len() as i32;
        // use mesh model matrix
        window.shader.set_mat4("model", &Matrix4::identity());
        // Draw only when window is created and inside loop
        // Drawn as triangles
        unsafe {
            // Bind mesh array
            gl::BindVertexArray(binder.vao);
            // Draw
            gl::DrawElements(gl::TRIANGLES,indices_len,gl::UNSIGNED_INT,ptr::null());
        }
    }
}

// If a drawable can come from a file, then the file needs a certain format
// Therefore, it needs to be checked
pub trait FromObj {

    fn compare_distances(max_min: &HashMap<&str,f64>) -> f64 {
        // Gets bigger distance from hashmap with specific entries.
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

    fn generate_fields(file: &str, ignored_coordinate: Option<usize>) -> (Vec<f64>,Vec<u32>,f64,[f64;3]) {
        // Initial variables
        let mut coordinates: Vec<f64> = Vec::new();
        let mut triangles: Vec<u32> = Vec::new();

        if !file.ends_with(".obj") {
            panic!("File chosen does not match extension allowed");
        }
        let file = File::open(file).expect("Error while opening file. Does file exists and is readable?");

        // Coordinates to calculate max length and closest element to 0
        let mut max_min = HashMap::from([
            ("x_min",0.0),
            ("y_min",0.0),
            ("z_min",0.0),
            ("x_max",0.0),
            ("y_max",0.0),
            ("z_max",0.0),
            ("x_closest",f64::MAX),
            ("y_closest",f64::MAX),
            ("z_closest",f64::MAX),
            ("min_distance", f64::MAX)
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
                        if coordinate.len() != 3 {
                            panic!("Every line starting with 'v ' should be three elements long")
                        }

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

                        // Check for closest value to 0
                        let min_distance = max_min.get_mut("min_distance").unwrap();
                        let new_distance = coordinate[0].powf(2.0) + coordinate[1].powf(2.0) + coordinate[2].powf(2.0);
                        let new_distance = new_distance.sqrt();
                        
                        if new_distance < *min_distance {
                            *min_distance = new_distance;
                            let x_closest = max_min.get_mut("x_closest").unwrap();
                            *x_closest = coordinate[0];
                            let y_closest = max_min.get_mut("y_closest").unwrap();
                            *y_closest = coordinate[1];
                            let z_closest = max_min.get_mut("z_closest").unwrap();
                            *z_closest = coordinate[2];
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
            
        let x_closest = *max_min.get("x_closest").unwrap();
        let y_closest = *max_min.get("y_closest").unwrap();
        let z_closest = *max_min.get("z_closest").unwrap();
        
        let max_distance = Self::compare_distances(&max_min);
        
        (coordinates,triangles,max_distance,[x_closest,y_closest,z_closest])
    }
}
