use dzahui::Mesh2D;
use std::fs::File;

#[test]
fn verify_coordinates_mesh() {
    let y = Mesh2D::get_ignored_coordinate("/home/Arthur/Tesis/Dzahui/assets/untitled.obj");
    assert!(y == 1);
}

#[test]
fn parse_coordinates() {
    let new_mesh = Mesh2D::new("/home/Arthur/Tesis/Dzahui/assets/test.obj");
    assert!(new_mesh.vertices == vec![-1.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0]);
    assert!(new_mesh.triangles == vec![0,1,2]);
}

#[test]
fn is_max_distance() {
    let new_mesh = Mesh2D::new("/home/Arthur/Tesis/Dzahui/assets/test.obj");
    println!("{}",new_mesh.max_length);
    assert!(new_mesh.max_length >= 1.90);
    assert!(new_mesh.max_length <= 2.10);
}