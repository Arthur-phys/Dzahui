use std::fs::File;
use std::io::{BufReader,BufRead};
use std::collections::HashMap;

pub struct Mesh {
    vertices: Vec<[f64;2]>,
    triangles: Vec<[f64;3]>
}   

impl Mesh {
    pub fn new(file: File) {
        let ignored_coordinate = Mesh::verify_coordinates(&file);
    }

    pub fn verify_coordinates(file: &File) -> &str {
        let mut x: HashMap<String,f32> = HashMap::new();
        let mut y: HashMap<String,f32> = HashMap::new();
        let mut z: HashMap<String,f32> = HashMap::new();
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
                    coordinates_iter.next();
                    let coordinates_vec: Vec<(String,f32)> = coordinates_iter.map(|c_str| (c_str.to_string(),c_str.parse::<f32>().unwrap()))
                    .into_iter().collect(); //use match instead of unwrap
                    x.insert(coordinates_vec[0].0.clone(),coordinates_vec[0].1); // Do not use clone, find replacement if possible
                    y.insert(coordinates_vec[1].0.clone(),coordinates_vec[1].1);
                    z.insert(coordinates_vec[2].0.clone(),coordinates_vec[2].1);
                },
                Err(error) => panic!("Unable to read file propperly {:?}",error)
            }
        });
        println!("{:?}",y);
        println!("{:?}",x);
        if x.values().count() == 1 {
            "x"
        } else if y.values().count() == 1 {
            "y"
        } else if z.values().count() == 1 {
            "z"
        } else if x.values().count() == 2 { // 0.0 different from -0.0 for mapping. Look for solution
            let x_val: Vec<&f32> = x.values().collect();
            if *x_val[0] == *x_val[1] {
                return "x"
            } else {
                panic!("Only coordinates over a plane paralell to x, y or z axis are accepted. Check .obj file.");
            }
        } else if y.values().count() == 2 {
            let y_val: Vec<&f32> = y.values().collect();
            if *y_val[0] == *y_val[1] {
                return "y"
            } else {
                panic!("Only coordinates over a plane paralell to x, y or z axis are accepted. Check .obj file.");
            }
        } else if z.values().count() == 2 {
            let z_val: Vec<&f32> = z.values().collect();
            if *z_val[0] == *z_val[1] {
                return "z"
            } else {
                panic!("Only coordinates over a plane paralell to x, y or z axis are accepted. Check .obj file.");
            }
        } else {
            panic!("Only coordinates over a plane paralell to x, y or z axis are accepted. Check .obj file.");
        }
    }

    pub fn run_graphical() {
        
    }
}

#[cfg(test)]
mod test {

}