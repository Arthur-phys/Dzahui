use dzahui::{DzahuiWindow, DzahuiWindowBuilder};

/// Simple mesh visualization on 2D.
fn main() {
    
    /* MESH VIEW */
    // 3D MESH
    // let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/big_mesh.obj").with_mesh_in_3d();
    // 2D MESH
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/sphere.obj").with_mesh_in_3d();

    let window = window_builder.build();
    window.run();

}
