use std::fs::File;
use std::io::{BufReader, BufRead, Seek};
use std::collections::HashMap;

// Enum serves to indicate which kind of simulation will be done
pub enum Dimension {
    D3,
    D2,
}
// Mesh should work for 2d and 3d
// Contains vertices and indices to generate triangles via gl
pub struct Mesh {
    pub vertices: Vec<f64>,
    pub triangles: Vec<u64>,
    pub ignored_coordinate: Option<usize>
}


impl Mesh {
    // New implementation differs in 3d and 2d because in one there has to be an ignored coordinate
    pub fn new(mut file: File, dim: Dimension) -> Mesh {
        // First the integrity of .obj file is checked
        let mut ignored_coordinate = None;
        match dim {
            Dimension::D3 => {},
            Dimension::D2 => {
                ignored_coordinate = Mesh::get_ignored_coordinate(&file);
                // Then the file is rewinded and we can start to obtain values
                file.rewind().unwrap();
            }
        }
        // Obtained coordinates from 'generate_coordinates()' function
        let (vertices, triangles) = Mesh::generate_coordinates(&file, ignored_coordinate);

        Mesh {
            ignored_coordinate,
            vertices,
            triangles
        }
    }

    pub fn get_ignored_coordinate(file: &File) -> Option<usize> {
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
            Some(0)
        } else if y.values().count() == 1 {
            Some(1)
        } else if z.values().count() == 1 {
            Some(2)
        } else {
            panic!("Only coordinates over a plane paralell to x, y or z axis are accepted. Check .obj file.");
        }
    }

    fn generate_coordinates(file: &File, ignored_coordinate: Option<usize>) -> (Vec<f64>,Vec<u64>) {

        // Initial variables
        let mut coordinates: Vec<f64> = Vec::new();
        let mut triangles: Vec<u64> = Vec::new();
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
                                match ignored_coordinate {
                                    Some(ic) => {
                                        // If there is an ignored coordinate:
                                        coordinate.remove(ic);
                                        // Last coordinate (z) becomes zero
                                        coordinate.push(0.0);
                                    },
                                    None => {}
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
                                let mut triangle: Vec<u64> = triangles_iter.map(|c| {
                                     // Do not use unwrap so much
                                    let vertex: u64 = c.split("/").next().unwrap().parse::<u64>().unwrap();
                                    // Return vertex-1 to match with index start in opengl (not_it/it/not_it)
                                    vertex-1
                                }).collect();
                                // Push into triangles vector of u64
                                triangles.append(&mut triangle);
                            }
                        },
                        // Error case of line matching
                        Err(error) => panic!("Unable to read file propperly {:?}",error)
                    }
                });
        (coordinates,triangles)
    }

    // pub fn run_graphical<G: Graphics>(&self, color:[f32;4], transform: Matrix2d, g: &mut G) {
        // self.triangles.iter().for_each(|triangle| {
        //     let traingle_vertices = [self.vertices[triangle[0]-1],self.vertices[triangle[1]-1],self.vertices[triangle[2]-1]];
        //     // .obj index starts on 1
        //     piston_window::polygon(color,&traingle_vertices,transform,g);
        // });
    //     panic!();
    // }
}

#[cfg(test)]
mod test {

}