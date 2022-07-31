use dzahui::{DzahuiWindowBuilder, DzahuiWindow};

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder<&str,&str,&str,&str, &str, &str> = DzahuiWindow::builder("/home/Arthur/Tesis/Dzahui/assets/3dmesh.obj");
    let window= window_builder.with_mesh_in_3d().with_vertex_body("./assets/sphere.obj").with_vertex_body_size(0.03).build();
    window.run();
}