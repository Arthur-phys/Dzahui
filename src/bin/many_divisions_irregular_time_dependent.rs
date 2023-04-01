use dzahui::{DzahuiWindow, DzahuiWindowBuilder, DiffussionParams};

/// Simple time dependent diffussion example with irregular mesh and many divisions
fn main() {
    /* TIME DEPENDENT DIFFUSSION */
    let diffussion_params = DiffussionParams::time_dependent()
    .b(1.0)
    .mu(1.0)
    .boundary_conditions(1.0, 24.0)
    .initial_conditions([
    10.975,
    10.35,
    10.62,
    10.243,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    10.73,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    ]).build();
    
    //Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar_many_divisions_irregular.obj")
        .solve_1d_time_dependent_diffussion(diffussion_params)
        .with_integration_iteration(150).with_time_step(0.00001).enable_height_multiplier(20_f64);

    let window = window_builder.build();
    window.run();
    
}