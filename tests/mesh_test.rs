use Dzahui::Mesh;
use std::fs::File;

#[test]
fn verify_mesh() {
    let f = File::open("/home/Arthur/Tesis/Dzahui/files/untitled.obj").unwrap();
    let y = Mesh::verify_coordinates(&f);
    assert!(y == "y");
}