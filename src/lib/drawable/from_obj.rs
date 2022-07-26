use std::{io::{BufReader, BufRead},collections::HashMap,fs::File};

// If a drawable can come from a file, then the file needs a certain format
// Therefore, it needs to be checked
pub trait FromObj {

    fn vertex_checker(line: &str) -> Result<bool,&str> {
        // Default vertex checker. Verifies amount of vertices per line is 3  
        let line_parts: Vec<&str> = line.split(" ").collect();
        if line_parts.len() != 4 {
           return Err("Amount of numbers per vertex should be 3.");
        }
        Ok(true)
    }

    fn face_checker(line: &str) -> Result<bool, &str> {
        // Default face checker. Verifies amount of face specifications per line is 3 and also have the correct syntax  
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

    fn check_obj(file: &str) {
        // Takes a series of functions that check one line at a time of a given .obj file.
        // Each function represents a check that has to be done on certain lines.
        // Returns a true value if file is usable by the program
        // Check for file extension
        if !file.ends_with(".obj") {
            panic!("File chosen does not match extension allowed.");
        }

        // Initializing file
        let file = File::open(file).expect("Error while opening file. Does file exists and is readable?");
        let reader = BufReader::new(file).lines();

        // For each line checks are made
        reader.for_each(|line| {
            match line {
                Ok(content) => {
                    if content.starts_with("v ") {

                        match Self::vertex_checker(&content) {
                            Err(e) => panic!("file checking failed because of: {}",e),
                            _ => {}
                        }

                    } else if content.starts_with("f ") {

                        match Self::face_checker(&content) {
                            Err(e) => panic!("file checking failed because of: {}",e),
                            _ => {}
                        }

                    }
                },
                Err(e) => panic!("Unable to read file properly: {}", e) 
            }
        });
    }

    fn compare_distances(max_min: &HashMap<&str,f32>) -> f32 {
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

    fn generate_fields(file: &str, ignored_coordinate: Option<usize>) -> (Vec<f32>,Vec<u32>,f32,[f32;3]) {
        // Obtains variables from .obj. To use after file check.

        // Initial variables
        let mut coordinates: Vec<f32> = Vec::new();
        let mut triangles: Vec<u32> = Vec::new();

        let file = File::open(file).expect("Error while opening file. Does file exists and is readable?");

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
}
