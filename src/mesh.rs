use std::fs::File;
use std::io::{BufReader, BufRead, Seek};
use std::collections::HashMap;
use piston_window::{self,Graphics,types::Matrix2d};


pub struct Mesh {
    pub vertices: Vec<[f64;2]>,
    pub triangles: Vec<[usize;3]>,
    pub ignored_coordinate: usize
}   

impl Mesh {
    pub fn new(mut file: File) -> Mesh {
        let ignored_coordinate = Mesh::verify_coordinates(&file);
        file.rewind().unwrap();
        let (vertices, triangles) = Mesh::generate_coordinates(&file, ignored_coordinate);

        Mesh {
            ignored_coordinate,
            vertices,
            triangles
        }
    }

    pub fn verify_coordinates(file: &File) -> usize {
        let mut x: HashMap<String,f64> = HashMap::new(); // to implement set from list. Use HashMap for better performance.
        let mut y: HashMap<String,f64> = HashMap::new();
        let mut z: HashMap<String,f64> = HashMap::new();
        let lines = BufReader::new(file).lines().filter(|line| {
            match line {
                Ok(content) => content.starts_with("v "),
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }
        });
        lines.for_each(|line| {
            match line {
                Ok(coordinates) => {
                    let mut coordinates_iter = coordinates.split(" ");
                    coordinates_iter.next(); // skip the 'v'
                    let coordinates_vec: [(String,f64);3] = coordinates_iter.map(|c_str| {
                        if c_str.starts_with("0.0") || c_str.starts_with("-0.0") { // Necessary for -0.0 and 0.0 equality
                            (String::from("0.0"),c_str.parse::<f64>().unwrap())
                        } else {
                            (c_str.to_string(),c_str.parse::<f64>().unwrap())
                        }
                    })
                    .into_iter().collect::<Vec<(String,f64)>>().try_into().expect(".obj's vertices should be composed of triads of numbers"); //use match instead of unwrap
                    x.insert(coordinates_vec[0].0.clone(),coordinates_vec[0].1); // Do not use clone, find replacement if possible
                    y.insert(coordinates_vec[1].0.clone(),coordinates_vec[1].1);
                    z.insert(coordinates_vec[2].0.clone(),coordinates_vec[2].1);
                },
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }
        });
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

    fn generate_coordinates(file: &File, ignored_coordinate: usize) -> (Vec<[f64;2]>,Vec<[usize;3]>) {

        let mut coordinates: Vec<[f64;2]> = Vec::new();
        let mut triangles: Vec<[usize;3]> = Vec::new();

        let reader = BufReader::new(file).lines();
        reader.for_each(|line| {
            match line {
                Ok(content) => {
                    if content.starts_with("v ") {
                        let mut coordinates_iter = content.split(" ");
                        coordinates_iter.next(); // skip the v
                        let mut coordinate: Vec<f64> = coordinates_iter.map(|c| c.parse::<f64>().unwrap()).collect();
                        coordinate.remove(ignored_coordinate);
                        coordinates.push(coordinate.try_into().unwrap()); // if 'verify_coordinates' passes, this unwrap is guaranteed to succeed.
                    }
                    else if content.starts_with("f ") {
                        let mut triangles_iter = content.split(" ");
                        triangles_iter.next(); // skip the f
                        let triangle: Vec<usize> = triangles_iter.map(|c| {
                            let vertex: usize = c.split("/").next().unwrap().parse::<usize>().unwrap(); // do not use unwrap so much
                            vertex
                        }).collect();
                        triangles.push(triangle.try_into().expect(".obj file formatted incorrectly. Please check the faces (f) section"));
                    }
                },
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }
        });

        (coordinates,triangles)
    }

    pub fn run_graphical<G: Graphics>(&self, color:[f32;4], transform: Matrix2d, g: &mut G) {
        self.triangles.iter().for_each(|triangle| {
            let traingle_vertices = [self.vertices[triangle[0]-1],self.vertices[triangle[1]-1],self.vertices[triangle[2]-1]];
            // .obj index starts on 1
            piston_window::polygon(color,&traingle_vertices,transform,g);
        });
    }
}

#[cfg(test)]
mod test {

}