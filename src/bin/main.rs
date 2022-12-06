use dzahui::{DzahuiWindow, DzahuiWindowBuilder, DiffussionParams};

fn main() {
    /* TIME DEPENDENT DIFFUSSION */
    let diffussion_params = DiffussionParams::time_dependent()
    .b(1.0)
    .mu(1.0)
    .boundary_conditions(1.0, 15.0)
    .initial_conditions([0_f64,135_f64,1215_f64,15432_f64,212141_f64,43431_f64, 6565_f64,655_f64]).build();
    
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
        .solve_1d_time_dependent_diffussion(diffussion_params)
        .with_integration_iteration(150).with_time_step(0.001);

    /* TIME INDEPENDENT DIFFUSSION */
    // let diffussion_params = DiffussionParams::time_independent()
    //     .b(1.0)
    //     .mu(1.0)
    //     .boundary_conditions(1.0,15.0);
    
    // let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
    //     .solve_1d_diffussion(diffussion_params).with_integration_iteration(150);


    /* MESH VIEW */
    // 3D MESH
    // let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/sphere.obj").with_mesh_in_3d();
    // 2D MESH
    // let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/big_mesh.obj");

    let window = window_builder.build();
    window.run();

}
