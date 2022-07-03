use std::{ptr,mem,fs::File,os::raw::c_void,io::{BufReader, BufRead, Seek}};
use gl::{self,types::{GLdouble, GLsizei, GLsizeiptr, GLuint}};
use std::collections::HashMap;
use cgmath::{Matrix4, Vector3};
use super::{Binder, Drawable, FromObj};

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

    fn setup(&self, binder: &mut Binder) {
        
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
                (self.vertices.len() * mem::size_of::<GLdouble>()) as GLsizeiptr,
                &self.vertices[0] as *const f64 as *const c_void,
                gl::STATIC_DRAW);// Double casting to raw pointer. Equivalent to C's void type when used as pointer.
                
            // Bind EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,binder.ebo);
            // Point to data, specify data length and hot it should be drawn
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (self.triangles.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                &self.triangles[0] as *const u32 as *const c_void,
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

    fn draw(&self) {
        let indices_len: i32 = self.triangles.len() as i32;
        // Draw only when window is created and inside loop
        // Drawn as triangles
        unsafe {
            gl::DrawElements(gl::TRIANGLES,indices_len,gl::UNSIGNED_INT,ptr::null());
        }
    }
}

impl FromObj for Mesh2D {
    fn generate_fields() {
        
    }
}


impl Mesh2D {
    // New implementation differs in 3d and 2d because in one there has to be an ignored coordinate
    pub fn new(mut file: File) -> Mesh2D {
        // First the integrity of .obj file is checked
        let ignored_coordinate = Mesh2D::get_ignored_coordinate(&file);
        // Then the file is rewinded and we can start to obtain values
        file.rewind().unwrap();

        // Obtained coordinates from 'generate_coordinates()' function
        let (vertices, triangles, max_length,mid_point) = Mesh2D::generate_coordinates(&file, ignored_coordinate);

        let model_matrix = Matrix4::from_translation(Vector3::new(mid_point[0] as f32,mid_point[1] as f32,mid_point[2] as f32));

        Mesh2D {
            ignored_coordinate,
            vertices,
            triangles,
            max_length,
            model_matrix,
        }
    }

    
    pub fn get_ignored_coordinate(file: &File) -> usize {
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
                    // Should use match instead of expect
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

    fn compare_distances(max_min: HashMap<&str,f64>) -> f64 {
        // GEst bigger distance from hashmap with specific entries.
        let x_min = max_min.get("x_min").unwrap();
        let y_min = max_min.get("y_min").unwrap();
        let x_max = max_min.get("x_max").unwrap();
        let y_max = max_min.get("y_max").unwrap();
        let d_x = *x_max-*x_min;
        let d_y = *y_max-*y_min;
        if d_x > d_y {
            d_x
        } else {
            d_y
        }

    }
    
    fn generate_coordinates(file: &File, ignored_coordinate: usize) -> (Vec<f64>,Vec<u32>,f64,[f64;3]) {
        // Initial variables
        let mut coordinates: Vec<f64> = Vec::new();
        let mut triangles: Vec<u32> = Vec::new();

        // Coordinates to calculate max length
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
                        coordinate.remove(ignored_coordinate);
                        // Last coordinate (z) becomes zero
                        coordinate.push(0.0);
        
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
                            *y_min = coordinate[0];
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
                                // Do not use unwrap so much
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
            
        let x_middle = max_min.get("x_max").unwrap() + max_min.get("x_min").unwrap() / 2.0;
        let y_middle = max_min.get("y_max").unwrap() + max_min.get("y_min").unwrap() / 2.0;
        let z_middle = max_min.get("z_max").unwrap() + max_min.get("z_min").unwrap() / 2.0;
        
        let max_distance = Mesh2D::compare_distances(max_min);
        
        (coordinates,triangles,max_distance,[x_middle,y_middle,z_middle])
    }
}

#[cfg(test)]
mod test {
    
}