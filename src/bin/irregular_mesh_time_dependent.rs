use dzahui::{DzahuiWindow, DzahuiWindowBuilder, DiffussionParams};

/// Simple time-dependent diffussion example with irregular nesh
fn main() {
    /* TIME DEPENDENT DIFFUSSION */
    let diffussion_params = DiffussionParams::time_dependent()
    .b(1.0)
    .mu(1.0)
    .boundary_conditions(1.0, 2400.0)
    .initial_conditions([
        0_f64,5_f64,12_f64,
        22_f64,21_f64,23_f64,
        45_f64,67_f64,97_f64,
        112_f64,156_f64,189_f64,
        188_f64,200_f64,256_f64,
        378_f64,423_f64,655_f64,
        892_f64,1000_f64,1255_f64,
        3000_f64,3000_f64,6655_f64,
        6565_f64,3000_f64,655_f64,
        400_f64,376_f64,356_f64,
        400_f64,
        ]).build();
    
    //Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar_irregular.obj")
        .solve_1d_time_dependent_diffussion(diffussion_params)
        .with_integration_iteration(150).with_time_step(0.00000001).enable_height_multiplier(3_f64);

    let window = window_builder.build();
    window.run();
    
}