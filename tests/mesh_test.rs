use Dzahui::Mesh;
use std::fs::File;

#[test]
fn verify_coordinates_mesh() {
    let f = File::open("/home/Arthur/Tesis/Dzahui/assets/untitled.obj").unwrap();
    let y = Mesh::verify_coordinates(&f);
    assert!(y == 1);
}

#[test]
fn parse_coordinates() {
    let f = File::open("/home/Arthur/Tesis/Dzahui/assets/test.obj").unwrap();
    let new_mesh = Mesh::new(f);
    assert!(new_mesh.vertices == vec![[-1.0,0.0],[1.0,0.0],[0.0,1.0]]);
    assert!(new_mesh.triangles == vec![[1,2,3]]);
}