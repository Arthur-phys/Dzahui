use dzahui::{Mesh,Dimension};
use std::fs::File;

#[test]
fn verify_coordinates_mesh() {
    let f = File::open("/home/Arthur/Tesis/Dzahui/assets/untitled.obj").unwrap();
    let y = Mesh::get_ignored_coordinate(&f);
    assert!(y == Some(1));
}

#[test]
fn parse_coordinates() {
    let f = File::open("/home/Arthur/Tesis/Dzahui/assets/test.obj").unwrap();
    let new_mesh = Mesh::new(f, Dimension::D2);
    assert!(new_mesh.vertices == vec![-1.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0]);
    assert!(new_mesh.triangles == vec![0,1,2]);
}