use dzahui::{DzahuiWindow, DzahuiWindowBuilder, DiffussionParams};

/// Simple time-dependent diffussion example
fn main() {
    /* TIME DEPENDENT DIFFUSSION */
    let diffussion_params = DiffussionParams::time_dependent()
    .b(1.0)
    .mu(1.0)
    .boundary_conditions(1.0, 15.0)
    .initial_conditions([0_f64,135_f64,1215_f64,15432_f64,212141_f64,43431_f64,6565_f64,3000_f64,655_f64]).build();
    
    //Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
        .solve_1d_time_dependent_diffussion(diffussion_params)
        .with_integration_iteration(150).with_time_step(0.001);

    let window = window_builder.build();
    window.run();
    
}