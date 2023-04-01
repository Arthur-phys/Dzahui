use dzahui::{DzahuiWindow, DzahuiWindowBuilder};

/// Simple mesh visualization in 2D.
fn main() {
    
    /* MESH VIEW */
    // 3D MESH
    // let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/big_mesh.obj").with_mesh_in_3d();
    // 2D MESH
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/untitled.obj");

    let window = window_builder.build();
    window.run();

}
