use dzahui::Mesh;

#[test]
fn verify_coordinates_mesh() {
    let y = Mesh::get_ignored_coordinate("/home/Arthur/Tesis/Dzahui/assets/untitled.obj");
    assert!(y == Some(1));
}

#[test]
fn parse_coordinates() {

    let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj").with_vertex_body("./assets/sphere.obj");
    let new_mesh = new_mesh.build_without_setup();
    assert!(new_mesh.vertices == vec![-1.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0]);
    assert!(new_mesh.triangles == vec![0,1,2]);
}

#[test]
fn is_max_distance() {

    let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj").with_vertex_body("./assets/sphere.obj").build_without_setup();
    println!("{}",new_mesh.max_length);
    assert!(new_mesh.max_length >= 1.90);
    assert!(new_mesh.max_length <= 2.10);
}